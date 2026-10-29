mod entries;
mod fields;

pub use entries::{parse_enum_item, parse_member};
pub use fields::{change_details, raw_text};

use indexmap::IndexMap;
use rapi_routes::{MemberKind, OwnerKind};

use super::docs::{ApiNames, CodeContext, describe_html};
use crate::dump::{ChangeElement, DescriptionChange, DumpChange, DumpDiff, RobloxVersion};
use crate::gen::normalize::{normalize_enum_item, normalize_member};
use crate::js::compare_names;
use crate::model::{
	ChangeCounts, ChangeEntry, ChangeStatus, DescriptionDetail, EntryValues, OwnerChanges,
	Release, Update,
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
			}
			ChangeElement::Member => {
				if let Some(entry) = entries::member_entry(change) {
					self.get(OwnerKind::Class, &entry.owner.clone()).entries.push(entry);
				}
			}
			ChangeElement::EnumItem => {
				if let Some(entry) = entries::enum_item_entry(change) {
					self.get(OwnerKind::Enum, &entry.owner.clone()).entries.push(entry);
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
				described_entry(change, EntryValues::Member {
					before: Some(member.clone()),
					after: Some(member),
				})
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
				described_entry(change, EntryValues::EnumItem {
					before: Some(item.clone()),
					after: Some(item),
				})
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

fn description_detail(names: &ApiNames<'_>, change: &DescriptionChange) -> Option<DescriptionDetail> {
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
		EntryValues::EnumItem { before, after } => {
			after.as_ref().or(before.as_ref()).map_or(0, |item| item.value)
		}
		EntryValues::Member { before, after } => after.as_ref().or(before.as_ref()).map_or(0, |member| {
			i64::try_from(
				MemberKind::ALL
					.iter()
					.position(|kind| *kind == member.kind())
					.unwrap_or_default(),
			)
			.unwrap_or_default()
		}),
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
	let mut description_count = 0;
	for change in &diff.descriptions {
		if let Some(detail) = description_detail(names, change) {
			table.add_description(change, detail);
			description_count += 1;
		}
	}
	let mut owners: Vec<OwnerChanges> = table
		.owners
		.into_values()
		.filter(|owner| {
			owner.status.is_some() || !owner.entries.is_empty() || !owner.descriptions.is_empty()
		})
		.collect();
	owners.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| compare_names(&a.name, &b.name)));
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
	let mut updates: Vec<Update> = diffs
		.iter()
		.map(|dated| build_update(&dated.diff, &dated.published, names))
		.collect();
	updates.reverse();
	updates
}
