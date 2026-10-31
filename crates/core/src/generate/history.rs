use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use rapi_routes::OwnerKind;

use crate::dump::{ChangeElement, DumpChange};
use crate::model::{
	Api, ChangeDetail, ChangeEntry, ChangeStatus, Deprecation, EntryValues, EnumItem,
	HistoryChanges, HistoryEntry, HistoryEvent, HistoryKind, ItemHistory, Member, MemberCounts,
	MemberDetail, OwnerChanges, Removed, RemovedOwner, RemovedOwners, Update,
};

const DEPRECATED: &str = "Deprecated";

pub fn owner_key(kind: OwnerKind, name: &str) -> String {
	format!("{}:{name}", kind.name())
}

pub fn item_key(kind: OwnerKind, owner: &str, name: &str) -> String {
	let prefix = match kind {
		OwnerKind::Class => "Member",
		OwnerKind::Enum => "EnumItem",
	};
	format!("{prefix}:{owner}.{name}")
}

fn entry_key(entry: &ChangeEntry) -> String {
	item_key(entry.owner_kind(), &entry.owner, &entry.name)
}

const fn status_kind(status: ChangeStatus) -> HistoryKind {
	match status {
		ChangeStatus::Added => HistoryKind::Added,
		ChangeStatus::Removed => HistoryKind::Removed,
		ChangeStatus::Changed => HistoryKind::Changed,
	}
}

fn field_name(label: &str) -> String {
	match label {
		"Security" | "Read security" | "Write security" => "security",
		"Thread safety" => "thread safety",
		"Capabilities" | "Read capabilities" | "Write capabilities" => "capabilities",
		"Property category" => "category",
		"Serialization" => "serialization",
		"Use instead" => "replacement",
		"Superclass" => "superclass",
		"Memory category" => "memory category",
		"Legacy names" => "legacy names",
		"Simulation access" => "simulation access",
		other => return other.to_lowercase(),
	}
	.to_owned()
}

type ParameterKey<'a> = (&'a str, &'a crate::model::ApiType, Option<&'a str>);

fn signature_key(member: &Member) -> (Vec<ParameterKey<'_>>, Option<&crate::model::ApiType>) {
	(
		member
			.parameters()
			.iter()
			.map(|parameter| {
				(
					parameter.name.as_str(),
					&parameter.ty,
					parameter.default.as_deref(),
				)
			})
			.collect(),
		member.returns(),
	)
}

fn member_fields(before: &Member, after: &Member) -> Vec<String> {
	if before.kind() != after.kind() {
		return vec!["signature".to_owned()];
	}
	if let (
		MemberDetail::Property {
			ty: before_type,
			default: before_default,
			..
		},
		MemberDetail::Property {
			ty: after_type,
			default: after_default,
			..
		},
	) = (&before.detail, &after.detail)
	{
		let mut fields = Vec::new();
		if before_type != after_type {
			fields.push("type".to_owned());
		}
		if before_default != after_default {
			fields.push("default value".to_owned());
		}
		return fields;
	}
	if signature_key(before) == signature_key(after) {
		return Vec::new();
	}
	let field = if matches!(before.detail, MemberDetail::Event { .. }) {
		"parameters"
	} else {
		"signature"
	};
	vec![field.to_owned()]
}

fn value_fields(entry: &ChangeEntry) -> Vec<String> {
	if entry.status != ChangeStatus::Changed {
		return Vec::new();
	}
	match &entry.values {
		EntryValues::EnumItem {
			before: Some(before),
			after: Some(after),
		} => {
			if before.value == after.value {
				Vec::new()
			} else {
				vec!["value".to_owned()]
			}
		}
		EntryValues::Member {
			before: Some(before),
			after: Some(after),
		} => member_fields(before, after),
		_ => Vec::new(),
	}
}

const fn is_empty(changes: &HistoryChanges) -> bool {
	changes.fields.is_empty()
		&& changes.tags_added == 0
		&& changes.tags_removed == 0
		&& changes.deprecation.is_none()
		&& changes.deprecated.is_none()
}

fn summarize(details: &[ChangeDetail], fields: Vec<String>) -> Option<HistoryChanges> {
	let mut changes = HistoryChanges {
		fields,
		..HistoryChanges::default()
	};
	for detail in details {
		match detail {
			ChangeDetail::Tags { label, tags } => {
				let added = label == "Added tags" || label == "Tags";
				if tags.iter().any(|tag| tag == DEPRECATED) {
					changes.deprecation = Some(if added {
						Deprecation::Set
					} else {
						Deprecation::Clear
					});
				}
				let others = u32::try_from(tags.iter().filter(|tag| *tag != DEPRECATED).count())
					.unwrap_or(u32::MAX);
				if added {
					changes.tags_added += others;
				} else {
					changes.tags_removed += others;
				}
			}
			ChangeDetail::Value { label, .. } => {
				let field = field_name(label);
				if !changes.fields.contains(&field) {
					changes.fields.push(field);
				}
			}
		}
	}
	(!is_empty(&changes)).then_some(changes)
}

pub fn entry_changes(entry: &ChangeEntry) -> Option<HistoryChanges> {
	if entry.status == ChangeStatus::Changed {
		summarize(&entry.details, value_fields(entry))
	} else {
		None
	}
}

fn merge_entry(changes: &mut HistoryChanges, entry: &HistoryChanges) {
	for field in &entry.fields {
		if !changes.fields.contains(field) {
			changes.fields.push(field.clone());
		}
	}
	changes.tags_added += entry.tags_added;
	changes.tags_removed += entry.tags_removed;
	if entry.deprecation == Some(Deprecation::Set) {
		changes.deprecated = Some(changes.deprecated.unwrap_or(0) + 1);
	}
}

pub fn owner_changes(owner: &OwnerChanges) -> Option<HistoryChanges> {
	if matches!(
		owner.status,
		Some(ChangeStatus::Added | ChangeStatus::Removed)
	) {
		return None;
	}
	let mut changes = summarize(&owner.details, Vec::new()).unwrap_or_default();
	for entry in &owner.entries {
		if let Some(summary) = entry_changes(entry) {
			merge_entry(&mut changes, &summary);
		}
	}
	(!is_empty(&changes)).then_some(changes)
}

fn entry_kinds(entry: &ChangeEntry) -> Vec<HistoryKind> {
	let mut kinds = Vec::new();
	if entry.is_api_change() {
		kinds.push(status_kind(entry.status));
	}
	if !entry.descriptions.is_empty() {
		kinds.push(HistoryKind::Described);
	}
	kinds
}

fn owner_kinds(owner: &OwnerChanges) -> Vec<HistoryKind> {
	let mut kinds: Vec<HistoryKind> = owner.status.map(status_kind).into_iter().collect();
	if !owner.descriptions.is_empty() {
		kinds.push(HistoryKind::Described);
	}
	kinds
}

fn member_counts(owner: &OwnerChanges) -> Option<MemberCounts> {
	if owner.status == Some(ChangeStatus::Added) {
		return None;
	}
	let mut counts = MemberCounts::default();
	for entry in owner.entries.iter().filter(|entry| entry.is_api_change()) {
		match entry.status {
			ChangeStatus::Added => counts.added += 1,
			ChangeStatus::Removed => counts.removed += 1,
			ChangeStatus::Changed => counts.changed += 1,
		}
	}
	(counts.added + counts.removed + counts.changed > 0).then_some(counts)
}

fn tag_detail(details: &[ChangeDetail], labels: &[&str]) -> bool {
	details.iter().any(|detail| {
		matches!(detail, ChangeDetail::Tags { label, tags }
			if labels.contains(&label.as_str()) && tags.iter().any(|tag| tag == DEPRECATED))
	})
}

pub fn entry_deprecation(entry: &ChangeEntry) -> Option<Deprecation> {
	if entry.status == ChangeStatus::Removed || tag_detail(&entry.details, &["Removed tags"]) {
		return Some(Deprecation::Clear);
	}
	let born = entry.status == ChangeStatus::Added
		&& entry
			.after_tags()
			.is_some_and(|tags| tags.iter().any(|tag| tag == DEPRECATED));
	(born || tag_detail(&entry.details, &["Added tags"])).then_some(Deprecation::Set)
}

pub fn owner_deprecation(owner: &OwnerChanges) -> Option<Deprecation> {
	if owner.status == Some(ChangeStatus::Removed) || tag_detail(&owner.details, &["Removed tags"])
	{
		return Some(Deprecation::Clear);
	}
	tag_detail(&owner.details, &["Tags", "Added tags"]).then_some(Deprecation::Set)
}

#[derive(Clone, Debug)]
pub enum RemovedValue {
	Member(Box<Member>),
	EnumItem(Box<EnumItem>),
}

#[derive(Default)]
pub struct HistoryTable {
	entries: HashMap<String, Vec<HistoryEntry>>,
	deprecations: HashMap<String, HistoryEvent>,
	removed: HashMap<String, IndexMap<String, (RemovedValue, HistoryEvent)>>,
	removed_owners: IndexMap<String, (OwnerKind, RemovedOwner)>,
}

impl HistoryTable {
	pub fn removed_owners_of(&self, kind: OwnerKind) -> Vec<RemovedOwner> {
		let mut owners: Vec<RemovedOwner> = self
			.removed_owners
			.values()
			.filter(|(owner_kind, _)| *owner_kind == kind)
			.map(|(_, owner)| owner.clone())
			.collect();
		owners.sort_by(|a, b| b.removed.published.cmp(&a.removed.published));
		owners
	}

	fn deprecate(&mut self, key: &str, change: Option<Deprecation>, event: &HistoryEvent) {
		match change {
			Some(Deprecation::Set) => {
				self.deprecations.insert(key.to_owned(), event.clone());
			}
			Some(Deprecation::Clear) => {
				self.deprecations.remove(key);
			}
			None => {}
		}
	}

	fn track_removal(&mut self, owner: &str, entry: &ChangeEntry, event: &HistoryEvent) {
		let removed = self.removed.entry(owner.to_owned()).or_default();
		if entry.status == ChangeStatus::Removed {
			let value = match &entry.values {
				EntryValues::Member {
					before: Some(before),
					..
				} => Some(RemovedValue::Member(before.clone())),
				EntryValues::EnumItem {
					before: Some(before),
					..
				} => Some(RemovedValue::EnumItem(before.clone())),
				_ => None,
			};
			if let Some(value) = value {
				removed.insert(entry.name.clone(), (value, event.clone()));
			}
		} else if entry.status == ChangeStatus::Added {
			removed.shift_remove(&entry.name);
		}
	}

	pub fn removed_of(&self, owner: &str) -> Vec<(RemovedValue, HistoryEvent)> {
		let mut removed: Vec<(RemovedValue, HistoryEvent)> = self
			.removed
			.get(owner)
			.map(|items| items.values().cloned().collect())
			.unwrap_or_default();
		removed.sort_by(|a, b| b.1.published.cmp(&a.1.published));
		removed
	}

	fn push(&mut self, key: String, entry: HistoryEntry) {
		if entry.kinds.is_empty() && entry.members.is_none() {
			return;
		}
		self.entries.entry(key).or_default().push(entry);
	}

	pub fn apply(&mut self, update: &Update) {
		let event = HistoryEvent {
			version: update.version.clone(),
			published: update.published.clone(),
		};
		for owner in &update.owners {
			let key = owner_key(owner.kind, &owner.name);
			match owner.status {
				Some(ChangeStatus::Added) => {
					self.removed.remove(&key);
					self.removed_owners.shift_remove(&key);
				}
				Some(ChangeStatus::Removed) => {
					let removed = RemovedOwner {
						name: owner.name.clone(),
						removed: event.clone(),
					};
					self.removed_owners
						.insert(key.clone(), (owner.kind, removed));
				}
				_ => {}
			}
			self.deprecate(&key, owner_deprecation(owner), &event);
			self.push(
				key.clone(),
				HistoryEntry {
					version: event.version.clone(),
					published: event.published.clone(),
					kinds: owner_kinds(owner),
					members: member_counts(owner),
					changes: owner_changes(owner),
				},
			);
			for entry in &owner.entries {
				let entry_key = entry_key(entry);
				self.deprecate(&entry_key, entry_deprecation(entry), &event);
				self.track_removal(&key, entry, &event);
				self.push(
					entry_key,
					HistoryEntry {
						version: event.version.clone(),
						published: event.published.clone(),
						kinds: entry_kinds(entry),
						members: None,
						changes: entry_changes(entry),
					},
				);
			}
		}
	}

	pub fn history(&self, key: &str) -> Option<ItemHistory> {
		let entries = self.entries.get(key)?;
		let added_index = entries
			.iter()
			.rposition(|entry| entry.kinds.contains(&HistoryKind::Added));
		let changed_index = entries.iter().rposition(|entry| {
			entry.kinds.contains(&HistoryKind::Changed) || entry.members.is_some()
		});
		let event = |entry: &HistoryEntry| HistoryEvent {
			version: entry.version.clone(),
			published: entry.published.clone(),
		};
		let added = added_index.map(|index| event(&entries[index]));
		let changed = changed_index
			.filter(|changed| added_index.is_none_or(|added| *changed > added))
			.map(|index| event(&entries[index]));
		Some(ItemHistory {
			entries: entries.iter().rev().cloned().collect(),
			added,
			added_before: None,
			unrecorded: false,
			changed,
			deprecated: self.deprecations.get(key).cloned(),
		})
	}
}

pub fn build_history(updates: &[Update]) -> HistoryTable {
	let mut table = HistoryTable::default();
	for update in updates.iter().rev() {
		table.apply(update);
	}
	table
}

pub fn unrecorded_key(change: &DumpChange) -> String {
	match change.element {
		ChangeElement::Class => owner_key(OwnerKind::Class, change.owner()),
		ChangeElement::Enum => owner_key(OwnerKind::Enum, change.owner()),
		ChangeElement::Member => item_key(OwnerKind::Class, change.owner(), change.name()),
		ChangeElement::EnumItem => item_key(OwnerKind::Enum, change.owner(), change.name()),
	}
}

pub fn apply_history(
	api: &mut Api,
	table: &HistoryTable,
	start: &str,
	unrecorded: &HashSet<String>,
) {
	api.history_start = Some(start.to_owned());
	api.removed = Some(RemovedOwners {
		classes: table
			.removed_owners_of(OwnerKind::Class)
			.into_iter()
			.filter(|owner| !api.classes.contains_key(&owner.name))
			.collect(),
		enums: table
			.removed_owners_of(OwnerKind::Enum)
			.into_iter()
			.filter(|owner| !api.enums.contains_key(&owner.name))
			.collect(),
	});
	let attach = |key: &str| {
		let unrecorded = unrecorded.contains(key);
		let history = table
			.history(key)
			.or_else(|| unrecorded.then(ItemHistory::default));
		history.map(|mut history| {
			if history.added.is_none() {
				if unrecorded {
					history.unrecorded = true;
				} else {
					history.added_before = Some(start.to_owned());
				}
			}
			history
		})
	};
	for item in api.classes.values_mut() {
		let key = owner_key(OwnerKind::Class, &item.name);
		if let Some(history) = attach(&key) {
			item.history = Some(history);
		}
		let removed: Vec<Removed<Member>> = table
			.removed_of(&key)
			.into_iter()
			.filter_map(|(value, removed)| match value {
				RemovedValue::Member(value) => Some(Removed {
					value: *value,
					removed,
				}),
				RemovedValue::EnumItem(_) => None,
			})
			.filter(|removed| {
				!item
					.members
					.iter()
					.any(|member| member.name == removed.value.name)
			})
			.collect();
		if !removed.is_empty() {
			item.removed_members = Some(removed);
		}
		for member in &mut item.members {
			if let Some(history) = attach(&item_key(OwnerKind::Class, &item.name, &member.name)) {
				member.history = Some(history);
			}
		}
	}
	for item in api.enums.values_mut() {
		let key = owner_key(OwnerKind::Enum, &item.name);
		if let Some(history) = attach(&key) {
			item.history = Some(history);
		}
		let removed: Vec<Removed<EnumItem>> = table
			.removed_of(&key)
			.into_iter()
			.filter_map(|(value, removed)| match value {
				RemovedValue::EnumItem(value) => Some(Removed {
					value: *value,
					removed,
				}),
				RemovedValue::Member(_) => None,
			})
			.filter(|removed| {
				!item
					.items
					.iter()
					.any(|entry| entry.name == removed.value.name)
			})
			.collect();
		if !removed.is_empty() {
			item.removed_items = Some(removed);
		}
		for entry in &mut item.items {
			if let Some(history) = attach(&item_key(OwnerKind::Enum, &item.name, &entry.name)) {
				entry.history = Some(history);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use rapi_routes::OwnerKind;

	use super::*;
	use crate::model::{
		ApiType, Capabilities, ChangeCounts, ChangeDetail, ChangeEntry, ChangeStatus,
		DescriptionDetail, DescriptionPart, EntryValues, HistoryKind, Member, MemberCounts,
		MemberDetail, OwnerChanges, Release, Security, TypeCategory, Update,
	};

	fn property(ty: &str, default: &str) -> Member {
		Member {
			name: "Size".to_owned(),
			history: None,
			tags: Vec::new(),
			security: Security::None,
			thread_safety: "ReadSafe".to_owned(),
			capabilities: Capabilities::default(),
			preferred: None,
			preferred_target: None,
			description: None,
			sample: None,
			detail: MemberDetail::Property {
				ty: ApiType {
					category: TypeCategory::Primitive,
					name: ty.to_owned(),
					optional: false,
					members: None,
				},
				category: "Data".to_owned(),
				default: Some(default.to_owned()),
				write_security: Security::None,
				can_load: true,
				can_save: true,
				simulation_access: false,
			},
		}
	}

	fn entry(
		status: ChangeStatus,
		before: Option<Member>,
		after: Option<Member>,
		details: Vec<ChangeDetail>,
	) -> ChangeEntry {
		ChangeEntry {
			status,
			owner: "Part".to_owned(),
			name: "Size".to_owned(),
			details,
			descriptions: Vec::new(),
			values: EntryValues::Member {
				before: before.map(Box::new),
				after: after.map(Box::new),
			},
		}
	}

	fn changed(details: Vec<ChangeDetail>) -> ChangeEntry {
		entry(
			ChangeStatus::Changed,
			Some(property("int", "1")),
			Some(property("int", "1")),
			details,
		)
	}

	fn value(label: &str) -> ChangeDetail {
		ChangeDetail::Value {
			label: label.to_owned(),
			before: None,
			after: None,
		}
	}

	fn added_tags(tags: &[&str]) -> ChangeDetail {
		ChangeDetail::Tags {
			label: "Added tags".to_owned(),
			tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
		}
	}

	fn changes(fields: &[&str], tags_added: u32) -> HistoryChanges {
		HistoryChanges {
			fields: fields.iter().map(|field| (*field).to_owned()).collect(),
			tags_added,
			..HistoryChanges::default()
		}
	}

	#[test]
	fn names_the_signature_parts_that_changed() {
		let resized = entry(
			ChangeStatus::Changed,
			Some(property("int", "1")),
			Some(property("float", "2")),
			Vec::new(),
		);
		assert_eq!(
			entry_changes(&resized),
			Some(changes(&["type", "default value"], 0))
		);
	}

	#[test]
	fn groups_read_and_write_security_and_counts_tags_apart_from_deprecated() {
		let entry = changed(vec![
			value("Read security"),
			value("Write security"),
			added_tags(&["Deprecated", "Hidden"]),
		]);
		assert_eq!(
			entry_changes(&entry),
			Some(HistoryChanges {
				deprecation: Some(Deprecation::Set),
				..changes(&["security"], 1)
			})
		);
	}

	#[test]
	fn leaves_added_members_and_description_only_changes_without_a_summary() {
		let mut added = changed(Vec::new());
		added.status = ChangeStatus::Added;
		assert_eq!(entry_changes(&added), None);
		assert_eq!(entry_changes(&changed(Vec::new())), None);
	}

	fn part(
		status: Option<ChangeStatus>,
		details: Vec<ChangeDetail>,
		entries: Vec<ChangeEntry>,
	) -> OwnerChanges {
		OwnerChanges {
			kind: OwnerKind::Class,
			name: "Part".to_owned(),
			status,
			details,
			descriptions: Vec::new(),
			entries,
			tags: Vec::new(),
		}
	}

	#[test]
	fn summarizes_the_class_details_but_not_when_it_is_added() {
		let owner = part(
			Some(ChangeStatus::Changed),
			vec![value("Memory category")],
			Vec::new(),
		);
		assert_eq!(
			owner_changes(&owner),
			Some(changes(&["memory category"], 0))
		);
		let added = part(
			Some(ChangeStatus::Added),
			vec![value("Memory category")],
			Vec::new(),
		);
		assert_eq!(owner_changes(&added), None);
	}

	#[test]
	fn rolls_up_what_changed_on_the_class_members() {
		let owner = part(
			None,
			Vec::new(),
			vec![
				changed(vec![value("Security")]),
				changed(vec![
					value("Read security"),
					added_tags(&["Deprecated", "Yields"]),
				]),
			],
		);
		assert_eq!(
			owner_changes(&owner),
			Some(HistoryChanges {
				deprecated: Some(1),
				..changes(&["security"], 1)
			})
		);
	}

	fn update(version: &str, owners: Vec<OwnerChanges>) -> Update {
		Update {
			version: version.to_owned(),
			hash: String::new(),
			published: format!("{version}-date"),
			previous: Release {
				version: String::new(),
				hash: String::new(),
			},
			counts: ChangeCounts::default(),
			description_count: Some(0),
			owners,
		}
	}

	type Line = (String, Vec<HistoryKind>, Option<MemberCounts>);

	fn summary(history: Option<ItemHistory>) -> (Option<String>, Option<String>, Vec<Line>) {
		let history = history.expect("a history");
		(
			history.added.map(|event| event.version),
			history.changed.map(|event| event.version),
			history
				.entries
				.into_iter()
				.map(|entry| (entry.version, entry.kinds, entry.members))
				.collect(),
		)
	}

	fn table() -> HistoryTable {
		let added = entry(
			ChangeStatus::Added,
			None,
			Some(property("int", "1")),
			Vec::new(),
		);
		let resized = entry(
			ChangeStatus::Changed,
			Some(property("int", "1")),
			Some(property("int", "2")),
			Vec::new(),
		);
		let mut described = entry(
			ChangeStatus::Changed,
			Some(property("int", "2")),
			Some(property("int", "2")),
			Vec::new(),
		);
		described.descriptions.push(DescriptionDetail {
			part: DescriptionPart::Description,
			parameter: None,
			status: ChangeStatus::Added,
			before: None,
			after: None,
		});
		let removed = entry(
			ChangeStatus::Removed,
			Some(property("int", "2")),
			None,
			Vec::new(),
		);
		let mut updates = vec![
			update(
				"1",
				vec![part(
					Some(ChangeStatus::Added),
					Vec::new(),
					vec![added.clone()],
				)],
			),
			update("2", vec![part(None, Vec::new(), vec![resized])]),
			update("3", vec![part(None, Vec::new(), vec![described])]),
			update("4", vec![part(None, Vec::new(), vec![removed])]),
			update("5", vec![part(None, Vec::new(), vec![added])]),
		];
		updates.reverse();
		build_history(&updates)
	}

	fn kinds(version: &str, kinds: &[HistoryKind], members: Option<(u32, u32, u32)>) -> Line {
		let members = members.map(|(added, removed, changed)| MemberCounts {
			added,
			removed,
			changed,
		});
		(version.to_owned(), kinds.to_vec(), members)
	}

	#[test]
	fn lists_every_change_to_a_member_newest_first() {
		assert_eq!(
			summary(table().history(&item_key(OwnerKind::Class, "Part", "Size"))),
			(
				Some("5".to_owned()),
				None,
				vec![
					kinds("5", &[HistoryKind::Added], None),
					kinds("4", &[HistoryKind::Removed], None),
					kinds("3", &[HistoryKind::Described], None),
					kinds("2", &[HistoryKind::Changed], None),
					kinds("1", &[HistoryKind::Added], None),
				]
			)
		);
	}

	#[test]
	fn counts_member_changes_on_the_class_but_not_members_of_a_new_class() {
		assert_eq!(
			summary(table().history(&owner_key(OwnerKind::Class, "Part"))),
			(
				Some("1".to_owned()),
				Some("5".to_owned()),
				vec![
					kinds("5", &[], Some((1, 0, 0))),
					kinds("4", &[], Some((0, 1, 0))),
					kinds("2", &[], Some((0, 0, 1))),
					kinds("1", &[HistoryKind::Added], None),
				]
			)
		);
	}

	#[test]
	fn remembers_removed_members_until_they_come_back() {
		let removed = entry(
			ChangeStatus::Removed,
			Some(property("int", "1")),
			None,
			Vec::new(),
		);
		let once = build_history(&[update("1", vec![part(None, Vec::new(), vec![removed])])]);
		let versions: Vec<String> = once
			.removed_of(&owner_key(OwnerKind::Class, "Part"))
			.into_iter()
			.map(|(_, event)| event.version)
			.collect();
		assert_eq!(versions, ["1"]);
		assert!(
			table()
				.removed_of(&owner_key(OwnerKind::Class, "Part"))
				.is_empty()
		);
	}

	#[test]
	fn dates_a_deprecation_by_the_update_that_added_the_tag() {
		let added = entry(
			ChangeStatus::Added,
			None,
			Some(property("int", "1")),
			Vec::new(),
		);
		let deprecated = changed(vec![added_tags(&["Deprecated"])]);
		let table = build_history(&[
			update("2", vec![part(None, Vec::new(), vec![deprecated])]),
			update(
				"1",
				vec![part(Some(ChangeStatus::Added), Vec::new(), vec![added])],
			),
		]);
		let history = table
			.history(&item_key(OwnerKind::Class, "Part", "Size"))
			.expect("a history");
		assert_eq!(
			history.deprecated.map(|event| event.version).as_deref(),
			Some("2")
		);
	}

	#[test]
	fn marks_the_addition_as_not_recorded_when_the_class_predates_the_history() {
		let table = build_history(&[update(
			"2",
			vec![part(Some(ChangeStatus::Changed), Vec::new(), Vec::new())],
		)]);
		let history = table
			.history(&owner_key(OwnerKind::Class, "Part"))
			.expect("a history");
		assert_eq!(history.added, None);
	}
}
