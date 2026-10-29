use rapi_routes::MemberKind;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::CountKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatRef {
	Class {
		name: String,
	},
	Enum {
		name: String,
	},
	DataType {
		name: String,
	},
	Member {
		kind: MemberKind,
		owner: String,
		name: String,
	},
	EnumItem {
		owner: String,
		name: String,
	},
	Parameter {
		owner: String,
		member: String,
		member_kind: MemberKind,
		name: String,
	},
	Name {
		of: CountKind,
		name: String,
	},
	Update {
		version: String,
		published: String,
	},
	Removed {
		of: CountKind,
		label: String,
		version: String,
		anchor: String,
	},
}

impl StatRef {
	pub const fn kind_name(&self) -> &'static str {
		match self {
			Self::Class { .. } => "Class",
			Self::Enum { .. } => "Enum",
			Self::DataType { .. } => "DataType",
			Self::Member { kind, .. } => kind.name(),
			Self::EnumItem { .. } => "EnumItem",
			Self::Parameter { .. } => "Parameter",
			Self::Name { .. } => "Name",
			Self::Update { .. } => "Update",
			Self::Removed { .. } => "Removed",
		}
	}

	pub const fn count_kind(&self) -> Option<CountKind> {
		match self {
			Self::Class { .. } => Some(CountKind::Class),
			Self::Enum { .. } => Some(CountKind::Enum),
			Self::DataType { .. } => Some(CountKind::DataType),
			Self::Member { kind, .. } => Some(CountKind::member(*kind)),
			Self::EnumItem { .. } => Some(CountKind::EnumItem),
			Self::Removed { of, .. } | Self::Name { of, .. } => Some(*of),
			Self::Parameter { .. } | Self::Update { .. } => None,
		}
	}
}

impl Serialize for StatRef {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut map = serializer.serialize_map(None)?;
		map.serialize_entry("kind", self.kind_name())?;
		match self {
			Self::Class { name } | Self::Enum { name } | Self::DataType { name } => {
				map.serialize_entry("name", name)?;
			}
			Self::Member { owner, name, .. } | Self::EnumItem { owner, name } => {
				map.serialize_entry("owner", owner)?;
				map.serialize_entry("name", name)?;
			}
			Self::Parameter {
				owner,
				member,
				member_kind,
				name,
			} => {
				map.serialize_entry("owner", owner)?;
				map.serialize_entry("member", member)?;
				map.serialize_entry("memberKind", member_kind.name())?;
				map.serialize_entry("name", name)?;
			}
			Self::Name { of, name } => {
				map.serialize_entry("of", of)?;
				map.serialize_entry("name", name)?;
			}
			Self::Update { version, published } => {
				map.serialize_entry("version", version)?;
				map.serialize_entry("published", published)?;
			}
			Self::Removed {
				of,
				label,
				version,
				anchor,
			} => {
				map.serialize_entry("of", of)?;
				map.serialize_entry("label", label)?;
				map.serialize_entry("version", version)?;
				map.serialize_entry("anchor", anchor)?;
			}
		}
		map.end()
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatRecord {
	pub id: String,
	pub label: String,
	pub hint: String,
	#[serde(serialize_with = "stat_value")]
	pub value: f64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unit: Option<String>,
	pub items: Vec<StatRef>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ties: Option<Ties>,
}

pub const fn has_items(record: &StatRecord) -> bool {
	!record.items.is_empty() || record.ties.is_some()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Ties {
	pub count: u32,
	pub kind: String,
}

#[expect(
	clippy::trivially_copy_pass_by_ref,
	reason = "serde passes fields by reference"
)]
fn stat_value<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
	if value.fract() == 0.0 && value.abs() < 1e15 {
		#[expect(
			clippy::cast_possible_truncation,
			reason = "the value is a whole number"
		)]
		let whole = *value as i64;
		serializer.serialize_i64(whole)
	} else {
		serializer.serialize_f64(*value)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecordGroup {
	pub id: String,
	pub title: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub table: Option<KindTable>,
	pub records: Vec<StatRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct KindRow {
	pub id: String,
	pub label: String,
	pub hint: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unit: Option<String>,
	pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct KindTable {
	pub kinds: Vec<CountKind>,
	pub rows: Vec<KindRow>,
}
