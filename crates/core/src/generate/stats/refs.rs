use rapi_routes::{MemberKind, OwnerKind, change_anchor, owner_anchor};

use crate::model::{
	Api, ChangeEntry, CountKind, EntryValues, OwnerChanges, StatRef, Update, UpdateEvent,
};

pub fn oldest_first(updates: &[Update]) -> impl Iterator<Item = &Update> {
	updates.iter().rev()
}

pub fn event_of(update: &Update) -> UpdateEvent {
	UpdateEvent {
		version: update.version.clone(),
		published: update.published.clone(),
	}
}

fn entry_member_kind(entry: &ChangeEntry) -> Option<MemberKind> {
	match &entry.values {
		EntryValues::Member { before, after } => after
			.as_ref()
			.or(before.as_ref())
			.map(|member| member.kind()),
		EntryValues::EnumItem { .. } => None,
	}
}

pub fn entry_kind(entry: &ChangeEntry) -> CountKind {
	match &entry.values {
		EntryValues::Member { .. } => {
			entry_member_kind(entry).map_or(CountKind::Property, CountKind::member)
		}
		EntryValues::EnumItem { .. } => CountKind::EnumItem,
	}
}

pub const fn owner_count_kind(kind: OwnerKind) -> CountKind {
	match kind {
		OwnerKind::Class => CountKind::Class,
		OwnerKind::Enum => CountKind::Enum,
	}
}

fn entry_separator(entry: &ChangeEntry) -> char {
	entry_member_kind(entry).map_or('.', MemberKind::separator)
}

pub fn entry_label(owner: &OwnerChanges, entry: &ChangeEntry) -> String {
	match owner.kind {
		OwnerKind::Enum => format!("Enum.{}.{}", owner.name, entry.name),
		OwnerKind::Class => format!("{}{}{}", owner.name, entry_separator(entry), entry.name),
	}
}

pub fn owner_label(kind: OwnerKind, name: &str) -> String {
	match kind {
		OwnerKind::Enum => format!("Enum.{name}"),
		OwnerKind::Class => name.to_owned(),
	}
}

pub fn removed_entry_ref(owner: &OwnerChanges, entry: &ChangeEntry, version: &str) -> StatRef {
	StatRef::Removed {
		of: entry_kind(entry),
		label: entry_label(owner, entry),
		version: version.to_owned(),
		anchor: change_anchor(owner.kind, &owner.name, &entry.name),
	}
}

pub fn removed_owner_ref(owner: &OwnerChanges, version: &str) -> StatRef {
	StatRef::Removed {
		of: owner_count_kind(owner.kind),
		label: owner_label(owner.kind, &owner.name),
		version: version.to_owned(),
		anchor: owner_anchor(owner.kind, &owner.name),
	}
}

pub fn current_entry_ref(api: &Api, owner: &OwnerChanges, entry: &ChangeEntry) -> Option<StatRef> {
	match owner.kind {
		OwnerKind::Enum => api
			.enums
			.get(&owner.name)
			.filter(|item| item.items.iter().any(|current| current.name == entry.name))
			.map(|_| StatRef::EnumItem {
				owner: owner.name.clone(),
				name: entry.name.clone(),
			}),
		OwnerKind::Class => api
			.classes
			.get(&owner.name)
			.and_then(|item| item.members.iter().find(|member| member.name == entry.name))
			.map(|member| StatRef::Member {
				kind: member.kind(),
				owner: owner.name.clone(),
				name: member.name.clone(),
			}),
	}
}

pub fn current_owner_ref(api: &Api, owner: &OwnerChanges) -> Option<StatRef> {
	match owner.kind {
		OwnerKind::Enum => api.enums.contains_key(&owner.name).then(|| StatRef::Enum {
			name: owner.name.clone(),
		}),
		OwnerKind::Class => api
			.classes
			.contains_key(&owner.name)
			.then(|| StatRef::Class {
				name: owner.name.clone(),
			}),
	}
}
