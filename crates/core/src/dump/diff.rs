use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::DescriptionPart;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffVersion {
	pub version: String,
	pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldChange {
	pub field: String,
	pub before: Value,
	pub after: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeElement {
	Class,
	Member,
	Enum,
	EnumItem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
	Add,
	Remove,
	Change,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DumpChange {
	pub element: ChangeElement,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub class: Option<String>,
	#[serde(default, rename = "enum", skip_serializing_if = "Option::is_none")]
	pub enumeration: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub member: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub item: Option<String>,
	#[serde(rename = "type")]
	pub change: ChangeType,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub changes: Option<Vec<FieldChange>>,
	pub value: Value,
}

impl DumpChange {
	pub fn owner(&self) -> &str {
		self.class
			.as_deref()
			.or(self.enumeration.as_deref())
			.unwrap_or_default()
	}

	pub fn name(&self) -> &str {
		self.member
			.as_deref()
			.or(self.item.as_deref())
			.unwrap_or_default()
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DescriptionChange {
	pub element: ChangeElement,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub class: Option<String>,
	#[serde(default, rename = "enum", skip_serializing_if = "Option::is_none")]
	pub enumeration: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub member: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub item: Option<String>,
	pub part: DescriptionPart,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub parameter: Option<String>,
	pub before: Option<String>,
	pub after: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub value: Option<Value>,
}

impl DescriptionChange {
	pub fn owner(&self) -> &str {
		self.class
			.as_deref()
			.or(self.enumeration.as_deref())
			.unwrap_or_default()
	}

	pub fn name(&self) -> &str {
		self.member
			.as_deref()
			.or(self.item.as_deref())
			.unwrap_or_default()
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DumpDiff {
	pub from: DiffVersion,
	pub to: DiffVersion,
	pub changes: Vec<DumpChange>,
	pub descriptions: Vec<DescriptionChange>,
}

pub fn to_tab_json<T: Serialize>(value: &T) -> String {
	let mut buffer = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, formatter);
	value
		.serialize(&mut serializer)
		.expect("JSON values always serialize");
	let mut text = String::from_utf8(buffer).expect("serde_json writes UTF-8");
	text.push('\n');
	text
}
