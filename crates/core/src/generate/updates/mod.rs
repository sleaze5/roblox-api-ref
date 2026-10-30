mod entries;
mod fields;

pub use entries::{parse_enum_item, parse_member};
pub use fields::{change_details, raw_text};

use indexmap::IndexMap;
use rapi_routes::{MemberKind, OwnerKind};

use super::docs::{ApiNames, CodeContext, describe_html};
use crate::dump::{ChangeElement, DescriptionChange, DumpChange, DumpDiff, RobloxVersion};
use crate::generate::normalize::{normalize_enum_item, normalize_member};
use crate::js::compare_names;
use crate::model::{
	ChangeCounts, ChangeEntry, ChangeStatus, DescriptionDetail, EntryValues, OwnerChanges, Release,
	Update,
};

#[derive(Default)]
struct OwnerTable {
	owners: IndexMap<(OwnerKind, String), OwnerChanges>,
}

impl OwnerTable {
	fn get(&mut self, kind: OwnerKind, name: &str) -> &mut OwnerChanges {
		self.owners
			.entry((kind, name.to_owned()))
			.or_insert_with(|| OwnerChanges {
				kind,
				name: name.to_owned(),
				status: None,
				details: Vec::new(),
				descriptions: Vec::new(),
				entries: Vec::new(),
				tags: Vec::new(),
			})
	}

	fn add(&mut self, change: &DumpChange) {
		match change.element {
			ChangeElement::Class | ChangeElement::Enum => {
				let kind = if change.element == ChangeElement::Class {
					OwnerKind::Class
				} else {
					OwnerKind::Enum
				};
				let owner = self.get(kind, change.owner());
				owner.status = Some(entries::status_of(change.change));
				owner.details = entries::owner_details(change);
				owner.tags = entries::owner_tags(change);
			}
			ChangeElement::Member => {
				if let Some(entry) = entries::member_entry(change) {
					self.get(OwnerKind::Class, &entry.owner.clone())
						.entries
						.push(entry);
				}
			}
			ChangeElement::EnumItem => {
				if let Some(entry) = entries::enum_item_entry(change) {
					self.get(OwnerKind::Enum, &entry.owner.clone())
						.entries
						.push(entry);
				}
			}
		}
	}

	fn add_description(&mut self, change: &DescriptionChange, detail: DescriptionDetail) {
		let entry = match change.element {
			ChangeElement::Class | ChangeElement::Enum => {
				let kind = if change.element == ChangeElement::Class {
					OwnerKind::Class
				} else {
					OwnerKind::Enum
				};
				self.get(kind, change.owner()).descriptions.push(detail);
				return;
			}
			ChangeElement::Member => {
				let Some(member) = change
					.value
					.as_ref()
					.and_then(parse_member)
					.map(|raw| normalize_member(&raw))
				else {
					return;
				};
				let member = Box::new(member);
				described_entry(
					change,
					EntryValues::Member {
						before: Some(member.clone()),
						after: Some(member),
					},
				)
			}
			ChangeElement::EnumItem => {
				let Some(item) = change
					.value
					.as_ref()
					.and_then(parse_enum_item)
					.map(|raw| normalize_enum_item(&raw))
				else {
					return;
				};
				let item = Box::new(item);
				described_entry(
					change,
					EntryValues::EnumItem {
						before: Some(item.clone()),
						after: Some(item),
					},
				)
			}
		};
		let owner = self.get(entry.owner_kind(), &entry.owner.clone());
		if let Some(existing) = owner
			.entries
			.iter_mut()
			.find(|current| current.name == entry.name && current.status != ChangeStatus::Removed)
		{
			existing.descriptions.push(detail);
		} else {
			let mut entry = entry;
			entry.descriptions.push(detail);
			owner.entries.push(entry);
		}
	}
}

fn described_entry(change: &DescriptionChange, values: EntryValues) -> ChangeEntry {
	ChangeEntry {
		status: ChangeStatus::Changed,
		owner: change.owner().to_owned(),
		name: change.name().to_owned(),
		details: Vec::new(),
		descriptions: Vec::new(),
		values,
	}
}

fn description_detail(
	names: &ApiNames<'_>,
	change: &DescriptionChange,
) -> Option<DescriptionDetail> {
	let context = match change.element {
		ChangeElement::Class | ChangeElement::Member => CodeContext::Class(change.owner()),
		ChangeElement::Enum | ChangeElement::EnumItem => CodeContext::Enum(change.owner()),
	};
	let convert = |html: Option<&String>| {
		html.and_then(|html| describe_html(names, context, html))
			.filter(|doc| !doc.is_empty())
	};
	let before = convert(change.before.as_ref());
	let after = convert(change.after.as_ref());
	if before == after {
		return None;
	}
	let status = match (&before, &after) {
		(None, _) => ChangeStatus::Added,
		(_, None) => ChangeStatus::Removed,
		_ => ChangeStatus::Changed,
	};
	Some(DescriptionDetail {
		part: change.part,
		parameter: change.parameter.clone(),
		status,
		before,
		after,
	})
}

fn entry_rank(entry: &ChangeEntry) -> i64 {
	match &entry.values {
		EntryValues::EnumItem { before, after } => after
			.as_ref()
			.or(before.as_ref())
			.map_or(0, |item| item.value),
		EntryValues::Member { before, after } => {
			after.as_ref().or(before.as_ref()).map_or(0, |member| {
				i64::try_from(
					MemberKind::ALL
						.iter()
						.position(|kind| *kind == member.kind())
						.unwrap_or_default(),
				)
				.unwrap_or_default()
			})
		}
	}
}

fn count_changes(owners: &IndexMap<(OwnerKind, String), OwnerChanges>) -> ChangeCounts {
	let mut counts = ChangeCounts::default();
	for owner in owners.values() {
		if let Some(status) = owner.status {
			counts.bump(status);
		}
		for entry in &owner.entries {
			counts.bump(entry.status);
		}
	}
	counts
}

pub fn build_update(diff: &DumpDiff, published: &str, names: &ApiNames<'_>) -> Update {
	let mut table = OwnerTable::default();
	for change in &diff.changes {
		table.add(change);
	}
	let counts = count_changes(&table.owners);
	let description_count = diff.descriptions.as_ref().map(|descriptions| {
		let mut count = 0;
		for change in descriptions {
			if let Some(detail) = description_detail(names, change) {
				table.add_description(change, detail);
				count += 1;
			}
		}
		count
	});
	let mut owners: Vec<OwnerChanges> = table
		.owners
		.into_values()
		.filter(|owner| {
			owner.status.is_some() || !owner.entries.is_empty() || !owner.descriptions.is_empty()
		})
		.collect();
	owners.sort_by(|a, b| {
		a.kind
			.cmp(&b.kind)
			.then_with(|| compare_names(&a.name, &b.name))
	});
	for owner in &mut owners {
		owner.entries.sort_by(|a, b| {
			entry_rank(a)
				.cmp(&entry_rank(b))
				.then_with(|| compare_names(&a.name, &b.name))
		});
	}
	Update {
		version: diff.to.version.clone(),
		hash: diff.to.hash.clone(),
		published: published.to_owned(),
		previous: Release {
			version: diff.from.version.clone(),
			hash: diff.from.hash.clone(),
		},
		counts,
		description_count,
		owners,
	}
}

pub struct DatedDiff {
	pub diff: DumpDiff,
	pub published: String,
}

pub fn check_diff(diff: &DumpDiff, from: &RobloxVersion, to: &RobloxVersion) -> Result<(), String> {
	if diff.from.hash != from.hash || diff.to.hash != to.hash {
		return Err(format!(
			"diffs/{}.json compares {} to {}, but roblox-versions.json lists {} before {}.",
			to.version, diff.from.version, diff.to.version, from.version, to.version
		));
	}
	Ok(())
}

pub fn build_updates(diffs: &[DatedDiff], names: &ApiNames<'_>) -> Vec<Update> {
	use rayon::prelude::*;
	let mut updates: Vec<Update> = diffs
		.par_iter()
		.map(|dated| build_update(&dated.diff, &dated.published, names))
		.collect();
	updates.reverse();
	updates
}

#[cfg(test)]
mod tests {
	use indexmap::IndexMap;
	use serde_json::{Value, json};

	use super::*;

	fn update(changes: &Value, descriptions: &Value) -> Value {
		let diff: DumpDiff = serde_json::from_value(json!({
			"from": { "version": "0.1.0.1", "hash": "aaaaaaaaaaaaaaaa" },
			"to": { "version": "0.2.0.1", "hash": "bbbbbbbbbbbbbbbb" },
			"changes": changes,
			"descriptions": descriptions,
		}))
		.expect("a valid diff");
		let (classes, enums, data_types) = (IndexMap::new(), IndexMap::new(), IndexMap::new());
		let names = ApiNames {
			classes: &classes,
			enums: &enums,
			data_types: &data_types,
		};
		serde_json::to_value(build_update(&diff, "2026-10-19T23:04:06.000Z", &names))
			.expect("an update serializes")
	}

	fn owners(update: &Value) -> Vec<(String, String)> {
		update["owners"]
			.as_array()
			.into_iter()
			.flatten()
			.map(|owner| {
				(
					owner["kind"].as_str().unwrap_or_default().to_owned(),
					owner["name"].as_str().unwrap_or_default().to_owned(),
				)
			})
			.collect()
	}

	#[test]
	fn groups_changes_by_class_and_enum_classes_first() {
		let result = update(
			&json!([
				{ "element": "EnumItem", "enum": "Material", "item": "Sand", "type": "Add", "value": { "Name": "Sand", "Value": 3 } },
				{
					"element": "Member", "class": "Part", "member": "Touch", "type": "Remove",
					"value": { "MemberType": "Event", "Name": "Touch", "Parameters": [], "Security": "None" },
				},
			]),
			&json!([]),
		);
		assert_eq!(
			owners(&result),
			[
				("Class".to_owned(), "Part".to_owned()),
				("Enum".to_owned(), "Material".to_owned())
			]
		);
		assert_eq!(
			result["counts"],
			json!({ "Added": 1, "Removed": 1, "Changed": 0 })
		);
		assert_eq!(
			result["previous"],
			json!({ "version": "0.1.0.1", "hash": "aaaaaaaaaaaaaaaa" })
		);
	}

	#[test]
	fn restores_the_previous_value_of_a_changed_member_and_describes_its_details() {
		let result = update(
			&json!([{
				"element": "Member", "class": "Part", "member": "Size", "type": "Change",
				"changes": [
					{ "field": "Default", "before": "1", "after": "2" },
					{ "field": "Security", "before": "None", "after": "PluginSecurity" },
					{ "field": "Tags", "before": ["Hidden"], "after": ["Deprecated"] },
				],
				"value": {
					"MemberType": "Function", "Name": "Size", "Parameters": [],
					"ReturnType": { "Category": "Primitive", "Name": "int" },
					"Security": "PluginSecurity", "Tags": ["Deprecated"],
				},
			}]),
			&json!([]),
		);
		let entry = &result["owners"][0]["entries"][0];
		assert_eq!(
			(&entry["before"]["security"], &entry["before"]["tags"]),
			(&json!("None"), &json!(["Hidden"]))
		);
		assert_eq!(
			(&entry["after"]["security"], &entry["after"]["tags"]),
			(&json!("PluginSecurity"), &json!(["Deprecated"]))
		);
		assert_eq!(
			entry["details"],
			json!([
				{ "kind": "value", "label": "Security", "before": { "text": "No security", "tone": "text" }, "after": { "text": "Plugin security", "tone": "text" } },
				{ "kind": "tags", "label": "Added tags", "tags": ["Deprecated"] },
				{ "kind": "tags", "label": "Removed tags", "tags": ["Hidden"] },
			])
		);
	}

	#[test]
	fn skips_changes_that_the_site_does_not_show() {
		let result = update(
			&json!([{
				"element": "Member", "class": "Part", "member": "Name", "type": "Change",
				"changes": [{ "field": "Default", "before": "__api_dump_a__", "after": "__api_dump_b__" }],
				"value": {
					"MemberType": "Property", "Name": "Name", "Category": "Data", "Default": "__api_dump_b__",
					"ValueType": { "Category": "Primitive", "Name": "string" },
					"Security": { "Read": "None", "Write": "None" },
					"Serialization": { "CanLoad": true, "CanSave": true },
				},
			}]),
			&json!([]),
		);
		assert_eq!(result["owners"], json!([]));
		assert_eq!(
			result["counts"],
			json!({ "Added": 0, "Removed": 0, "Changed": 0 })
		);
	}

	#[test]
	fn sorts_enum_items_by_value() {
		let changes: Vec<Value> = [5, 1, 3]
			.iter()
			.map(|value| json!({ "element": "EnumItem", "enum": "Material", "item": format!("Item{value}"), "type": "Add", "value": { "Name": format!("Item{value}"), "Value": value } }))
			.collect();
		let result = update(&Value::Array(changes), &json!([]));
		let names: Vec<&str> = result["owners"][0]["entries"]
			.as_array()
			.unwrap()
			.iter()
			.map(|entry| entry["name"].as_str().unwrap())
			.collect();
		assert_eq!(names, ["Item1", "Item3", "Item5"]);
	}

	#[test]
	fn adds_description_changes_without_counting_them_as_api_changes() {
		let anchored = json!({
			"MemberType": "Property", "Name": "Anchored", "Category": "Behavior",
			"ValueType": { "Category": "Primitive", "Name": "bool" },
			"Security": { "Read": "None", "Write": "None" },
			"Serialization": { "CanLoad": true, "CanSave": true },
		});
		let result = update(
			&json!([]),
			&json!([
				{ "element": "Member", "class": "Part", "member": "Anchored", "part": "description", "before": "Old text.", "after": "New text.", "value": anchored },
				{ "element": "Class", "class": "Part", "part": "description", "before": null, "after": "A <code>Part</code>." },
				{ "element": "Member", "class": "Part", "member": "Anchored", "part": "return", "before": "Same.", "after": "<p>Same.</p>", "value": anchored },
			]),
		);
		assert_eq!(
			result["counts"],
			json!({ "Added": 0, "Removed": 0, "Changed": 0 })
		);
		assert_eq!(result["descriptionCount"], 2);
		let owner = &result["owners"][0];
		assert_eq!(
			owner["descriptions"],
			json!([{ "part": "description", "status": "Added", "after": ["A ", { "tag": "code", "children": ["Part"] }, "."] }])
		);
		let entries = owner["entries"].as_array().unwrap();
		assert_eq!(entries.len(), 1);
		assert_eq!(
			(&entries[0]["name"], &entries[0]["status"]),
			(&json!("Anchored"), &json!("Changed"))
		);
		assert_eq!(
			entries[0]["descriptions"],
			json!([{ "part": "description", "status": "Changed", "before": ["Old text."], "after": ["New text."] }])
		);
	}

	#[test]
	fn attaches_a_description_change_to_the_api_change_of_the_same_item() {
		let result = update(
			&json!([{ "element": "EnumItem", "enum": "Material", "item": "Sand", "type": "Add", "value": { "Name": "Sand", "Value": 3 } }]),
			&json!([{ "element": "EnumItem", "enum": "Material", "item": "Sand", "part": "description", "before": null, "after": "Grainy.", "value": { "Name": "Sand", "Value": 3 } }]),
		);
		let entries = result["owners"][0]["entries"].as_array().unwrap();
		assert_eq!(entries.len(), 1);
		assert_eq!(entries[0]["status"], "Added");
		assert_eq!(
			entries[0]["descriptions"],
			json!([{ "part": "description", "status": "Added", "after": ["Grainy."] }])
		);
		assert_eq!(result["counts"]["Added"], 1);
	}

	#[test]
	fn leaves_the_description_count_unknown_when_the_diff_has_no_descriptions() {
		let result = update(&json!([]), &Value::Null);
		assert_eq!(result["descriptionCount"], Value::Null);
		assert_eq!(update(&json!([]), &json!([]))["descriptionCount"], 0);
	}
}
