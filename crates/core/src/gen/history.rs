use std::collections::HashMap;

use indexmap::IndexMap;
use rapi_routes::OwnerKind;

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

fn signature_key(member: &Member) -> (Vec<(&str, &crate::model::ApiType, Option<&str>)>, Option<&crate::model::ApiType>) {
	(
		member
			.parameters()
			.iter()
			.map(|parameter| (parameter.name.as_str(), &parameter.ty, parameter.default.as_deref()))
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
	if matches!(owner.status, Some(ChangeStatus::Added | ChangeStatus::Removed)) {
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
	if owner.status == Some(ChangeStatus::Removed)
		|| tag_detail(&owner.details, &["Removed tags"])
	{
		return Some(Deprecation::Clear);
	}
	tag_detail(&owner.details, &["Tags", "Added tags"]).then_some(Deprecation::Set)
}

#[derive(Clone, Debug)]
pub enum RemovedValue {
	Member(Member),
	EnumItem(EnumItem),
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
					self.removed_owners.insert(key.clone(), (owner.kind, removed));
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

pub fn apply_history(api: &mut Api, table: &HistoryTable, start: &str) {
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
		table.history(key).map(|mut history| {
			if history.added.is_none() {
				history.added_before = Some(start.to_owned());
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
				RemovedValue::Member(value) => Some(Removed { value, removed }),
				RemovedValue::EnumItem(_) => None,
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
				RemovedValue::EnumItem(value) => Some(Removed { value, removed }),
				RemovedValue::Member(_) => None,
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
