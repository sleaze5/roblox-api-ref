use rapi_routes::{
	MemberKind, OwnerKind, change_anchor, owner_anchor, removed_enum_item_href,
	removed_member_href, removed_owner_href, update_href,
};

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

fn owner_removed(api: &Api, kind: OwnerKind, name: &str) -> bool {
	let removed = match kind {
		OwnerKind::Class => api.removed_classes(),
		OwnerKind::Enum => api.removed_enums(),
	};
	removed.iter().any(|owner| owner.name == name)
}

fn removed_entry_href(api: &Api, owner: &OwnerChanges, entry: &ChangeEntry) -> Option<String> {
	if owner_removed(api, owner.kind, &owner.name) {
		return Some(removed_owner_href(owner.kind, &owner.name));
	}
	match owner.kind {
		OwnerKind::Enum => api
			.enums
			.get(&owner.name)?
			.removed_items
			.iter()
			.flatten()
			.any(|removed| removed.value.name == entry.name)
			.then(|| removed_enum_item_href(&owner.name, &entry.name)),
		OwnerKind::Class => {
			let kind = entry_member_kind(entry)?;
			api.classes
				.get(&owner.name)?
				.removed_members
				.iter()
				.flatten()
				.any(|removed| removed.value.name == entry.name && removed.value.kind() == kind)
				.then(|| removed_member_href(&owner.name, kind, &entry.name))
		}
	}
}

pub fn removed_entry_ref(
	api: &Api,
	owner: &OwnerChanges,
	entry: &ChangeEntry,
	version: &str,
) -> StatRef {
	StatRef::Removed {
		of: entry_kind(entry),
		label: entry_label(owner, entry),
		href: removed_entry_href(api, owner, entry).unwrap_or_else(|| {
			format!(
				"{}#{}",
				update_href(version),
				change_anchor(owner.kind, &owner.name, &entry.name)
			)
		}),
	}
}

pub fn removed_owner_ref(api: &Api, owner: &OwnerChanges, version: &str) -> StatRef {
	StatRef::Removed {
		of: owner_count_kind(owner.kind),
		label: owner_label(owner.kind, &owner.name),
		href: if owner_removed(api, owner.kind, &owner.name) {
			removed_owner_href(owner.kind, &owner.name)
		} else {
			format!(
				"{}#{}",
				update_href(version),
				owner_anchor(owner.kind, &owner.name)
			)
		},
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
