use serde_json::Value;

use crate::api::tags::security_label;
use crate::dump::{DumpCapabilities, DumpTag, FieldChange};
use crate::generate::normalize::{normalize_capabilities, normalize_security};
use crate::js::compare_code_units;
use crate::model::{Capabilities, ChangeDetail, DetailTone, DetailValue};

const SIGNATURE_FIELDS: [&str; 6] = [
	"Default",
	"MemberType",
	"Parameters",
	"ReturnType",
	"Value",
	"ValueType",
];

fn value(text: impl Into<String>, tone: DetailTone) -> DetailValue {
	DetailValue {
		text: text.into(),
		tone,
	}
}

pub fn raw_text(raw: &Value) -> String {
	match raw {
		Value::String(text) => text.clone(),
		other => serde_json::to_string(other).unwrap_or_default(),
	}
}

fn value_detail(
	label: &str,
	before: Option<DetailValue>,
	after: Option<DetailValue>,
) -> Vec<ChangeDetail> {
	if before.as_ref().map(|value| &value.text) == after.as_ref().map(|value| &value.text) {
		return Vec::new();
	}
	vec![ChangeDetail::Value {
		label: label.to_owned(),
		before,
		after,
	}]
}

fn security_value(security: Option<&Value>) -> DetailValue {
	let text = match security {
		None | Some(Value::Null) => "None".to_owned(),
		Some(raw) => raw_text(raw),
	};
	value(security_label(normalize_security(&text)), DetailTone::Text)
}

fn is_object_like(raw: &Value) -> bool {
	matches!(raw, Value::Object(_) | Value::Array(_) | Value::Null)
}

fn security_details(change: &FieldChange) -> Vec<ChangeDetail> {
	if is_object_like(&change.before) && is_object_like(&change.after) {
		let side = |raw: &Value, key: &str| raw.get(key).cloned();
		let mut details = value_detail(
			"Read security",
			Some(security_value(side(&change.before, "Read").as_ref())),
			Some(security_value(side(&change.after, "Read").as_ref())),
		);
		details.extend(value_detail(
			"Write security",
			Some(security_value(side(&change.before, "Write").as_ref())),
			Some(security_value(side(&change.after, "Write").as_ref())),
		));
		return details;
	}
	value_detail(
		"Security",
		Some(security_value(Some(&change.before))),
		Some(security_value(Some(&change.after))),
	)
}

fn serialization_text(raw: &Value) -> &'static str {
	let flag = |key: &str| raw.get(key).and_then(Value::as_bool).unwrap_or(false);
	match (flag("CanLoad"), flag("CanSave")) {
		(true, true) => "Load and save",
		(true, false) => "Load only",
		(false, true) => "Save only",
		(false, false) => "Not serialized",
	}
}

fn capability_value(names: &[String]) -> DetailValue {
	if names.is_empty() {
		value("None", DetailTone::Text)
	} else {
		value(names.join(" + "), DetailTone::Code)
	}
}

fn capabilities_of(raw: &Value) -> Capabilities {
	if raw.is_null() {
		return normalize_capabilities(None);
	}
	let parsed: Option<DumpCapabilities> = serde_json::from_value(raw.clone()).ok();
	normalize_capabilities(parsed.as_ref())
}

fn capability_details(change: &FieldChange) -> Vec<ChangeDetail> {
	let before = capabilities_of(&change.before);
	let after = capabilities_of(&change.after);
	let shared =
		|capabilities: &Capabilities| capabilities.read.join(",") == capabilities.write.join(",");
	if shared(&before) && shared(&after) {
		return value_detail(
			"Capabilities",
			Some(capability_value(&before.read)),
			Some(capability_value(&after.read)),
		);
	}
	let mut details = value_detail(
		"Read capabilities",
		Some(capability_value(&before.read)),
		Some(capability_value(&after.read)),
	);
	details.extend(value_detail(
		"Write capabilities",
		Some(capability_value(&before.write)),
		Some(capability_value(&after.write)),
	));
	details
}

fn dump_tags(raw: &Value) -> Vec<DumpTag> {
	match raw {
		Value::Array(items) => items
			.iter()
			.filter_map(|item| serde_json::from_value(item.clone()).ok())
			.collect(),
		_ => Vec::new(),
	}
}

pub fn string_tags(raw: &Value) -> Vec<String> {
	crate::generate::normalize::string_tags(Some(&dump_tags(raw)))
}

fn preferred_value(raw: &Value) -> Option<DetailValue> {
	crate::generate::normalize::preferred_name(Some(&dump_tags(raw)))
		.map(|name| value(name, DetailTone::Code))
}

fn unique_sorted(tags: Vec<String>, other: &[String]) -> Vec<String> {
	let mut unique: Vec<String> = Vec::new();
	for tag in tags {
		if !other.contains(&tag) && !unique.contains(&tag) {
			unique.push(tag);
		}
	}
	unique.sort_by(|a, b| compare_code_units(a, b));
	unique
}

fn tag_details(before: &Value, after: &Value) -> Vec<ChangeDetail> {
	let previous = string_tags(before);
	let next = string_tags(after);
	let added = unique_sorted(next.clone(), &previous);
	let removed = unique_sorted(previous, &next);
	let mut details = Vec::new();
	if !added.is_empty() {
		details.push(ChangeDetail::Tags {
			label: "Added tags".to_owned(),
			tags: added,
		});
	}
	if !removed.is_empty() {
		details.push(ChangeDetail::Tags {
			label: "Removed tags".to_owned(),
			tags: removed,
		});
	}
	details.extend(value_detail(
		"Use instead",
		preferred_value(before),
		preferred_value(after),
	));
	details
}

fn optional_value(raw: &Value, tone: DetailTone) -> Option<DetailValue> {
	(!raw.is_null()).then(|| value(raw_text(raw), tone))
}

fn list_value(raw: &Value) -> DetailValue {
	let names: Vec<String> = match raw {
		Value::Array(items) => items.iter().map(raw_text).collect(),
		_ => Vec::new(),
	};
	if names.is_empty() {
		value("None", DetailTone::Text)
	} else {
		value(names.join(", "), DetailTone::Code)
	}
}

fn text_or(raw: &Value, fallback: &str) -> String {
	if raw.is_null() {
		fallback.to_owned()
	} else {
		raw_text(raw)
	}
}

fn field_details(change: &FieldChange) -> Vec<ChangeDetail> {
	let (before, after) = (&change.before, &change.after);
	match change.field.as_str() {
		"Tags" => tag_details(before, after),
		"Security" => security_details(change),
		"Capabilities" => capability_details(change),
		"Serialization" => value_detail(
			"Serialization",
			Some(value(serialization_text(before), DetailTone::Text)),
			Some(value(serialization_text(after), DetailTone::Text)),
		),
		"ThreadSafety" => value_detail(
			"Thread safety",
			Some(value(text_or(before, "Unknown"), DetailTone::Code)),
			Some(value(text_or(after, "Unknown"), DetailTone::Code)),
		),
		"SimulationAccess" => {
			let yes = |raw: &Value| {
				if raw == &Value::Bool(true) {
					"Yes"
				} else {
					"No"
				}
			};
			value_detail(
				"Simulation access",
				Some(value(yes(before), DetailTone::Text)),
				Some(value(yes(after), DetailTone::Text)),
			)
		}
		"Category" => value_detail(
			"Property category",
			optional_value(before, DetailTone::Code),
			optional_value(after, DetailTone::Code),
		),
		"Superclass" => value_detail(
			"Superclass",
			optional_value(before, DetailTone::Type),
			optional_value(after, DetailTone::Type),
		),
		"MemoryCategory" => value_detail(
			"Memory category",
			optional_value(before, DetailTone::Code),
			optional_value(after, DetailTone::Code),
		),
		"LegacyNames" => value_detail(
			"Legacy names",
			Some(list_value(before)),
			Some(list_value(after)),
		),
		field => value_detail(
			field,
			optional_value(before, DetailTone::Code),
			optional_value(after, DetailTone::Code),
		),
	}
}

pub fn change_details(changes: &[FieldChange]) -> Vec<ChangeDetail> {
	changes
		.iter()
		.filter(|change| !SIGNATURE_FIELDS.contains(&change.field.as_str()))
		.flat_map(field_details)
		.collect()
}
