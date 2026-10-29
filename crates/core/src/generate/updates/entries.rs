use serde_json::Value;

use super::fields::{change_details, string_tags};
use crate::dump::{ChangeElement, ChangeType, DumpChange, DumpEnumItem, DumpMember, FieldChange};
use crate::generate::normalize::{normalize_enum_item, normalize_member};
use crate::js::compare_code_units;
use crate::model::{ChangeDetail, ChangeEntry, ChangeStatus, DetailTone, DetailValue, EntryValues};

const ROOT_SUPERCLASS: &str = "<<<ROOT>>>";

pub const fn status_of(change: ChangeType) -> ChangeStatus {
	match change {
		ChangeType::Add => ChangeStatus::Added,
		ChangeType::Remove => ChangeStatus::Removed,
		ChangeType::Change => ChangeStatus::Changed,
	}
}

fn restore(value: &Value, changes: &[FieldChange]) -> Value {
	let mut before = value.clone();
	if let Value::Object(fields) = &mut before {
		for change in changes {
			if change.before.is_null() {
				fields.shift_remove(&change.field);
			} else {
				fields.insert(change.field.clone(), change.before.clone());
			}
		}
	}
	before
}

pub fn parse_member(raw: &Value) -> Option<DumpMember> {
	serde_json::from_value(raw.clone()).ok()
}

pub fn parse_enum_item(raw: &Value) -> Option<DumpEnumItem> {
	serde_json::from_value(raw.clone()).ok()
}

type Sides<T> = (ChangeStatus, Vec<ChangeDetail>, Option<T>, Option<T>);

fn sides<T: PartialEq>(
	change: &DumpChange,
	normalize: impl Fn(&Value) -> Option<T>,
) -> Option<Sides<T>> {
	let status = status_of(change.change);
	match change.change {
		ChangeType::Add => Some((status, Vec::new(), None, normalize(&change.value))),
		ChangeType::Remove => Some((status, Vec::new(), normalize(&change.value), None)),
		ChangeType::Change => {
			let fields = change.changes.as_deref().unwrap_or_default();
			let before = normalize(&restore(&change.value, fields));
			let after = normalize(&change.value);
			let details = change_details(fields);
			if details.is_empty() && before == after {
				return None;
			}
			Some((status, details, before, after))
		}
	}
}

pub fn member_entry(change: &DumpChange) -> Option<ChangeEntry> {
	let (status, details, before, after) = sides(change, |raw| {
		parse_member(raw).as_ref().map(normalize_member)
	})?;
	Some(ChangeEntry {
		status,
		owner: change.owner().to_owned(),
		name: change.name().to_owned(),
		details,
		descriptions: Vec::new(),
		values: EntryValues::Member {
			before: before.map(Box::new),
			after: after.map(Box::new),
		},
	})
}

pub fn enum_item_entry(change: &DumpChange) -> Option<ChangeEntry> {
	let (status, details, before, after) = sides(change, |raw| {
		parse_enum_item(raw).as_ref().map(normalize_enum_item)
	})?;
	Some(ChangeEntry {
		status,
		owner: change.owner().to_owned(),
		name: change.name().to_owned(),
		details,
		descriptions: Vec::new(),
		values: EntryValues::EnumItem {
			before: before.map(Box::new),
			after: after.map(Box::new),
		},
	})
}

fn added_details(change: &DumpChange) -> Vec<ChangeDetail> {
	let superclass = if change.element == ChangeElement::Class {
		change
			.value
			.get("Superclass")
			.map_or_else(String::new, super::fields::raw_text)
	} else {
		ROOT_SUPERCLASS.to_owned()
	};
	let mut details = Vec::new();
	if superclass != ROOT_SUPERCLASS {
		details.push(ChangeDetail::Value {
			label: "Superclass".to_owned(),
			before: None,
			after: Some(DetailValue {
				text: superclass,
				tone: DetailTone::Type,
			}),
		});
	}
	let mut tags = string_tags(change.value.get("Tags").unwrap_or(&Value::Null));
	tags.sort_by(|a, b| compare_code_units(a, b));
	if !tags.is_empty() {
		details.push(ChangeDetail::Tags {
			label: "Tags".to_owned(),
			tags,
		});
	}
	details
}

pub fn owner_tags(change: &DumpChange) -> Vec<String> {
	string_tags(change.value.get("Tags").unwrap_or(&Value::Null))
}

pub fn owner_details(change: &DumpChange) -> Vec<ChangeDetail> {
	match change.change {
		ChangeType::Add => added_details(change),
		ChangeType::Remove => Vec::new(),
		ChangeType::Change => change_details(change.changes.as_deref().unwrap_or_default()),
	}
}
