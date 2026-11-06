use rapi_routes::MemberKind;

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
		href: String,
	},
}

impl StatRef {
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

#[derive(Clone, Debug, PartialEq)]
pub struct StatRecord {
	pub id: String,
	pub label: String,
	pub hint: String,
	pub value: f64,
	pub unit: Option<String>,
	pub items: Vec<StatRef>,
	pub ties: Option<Ties>,
}

pub const fn has_items(record: &StatRecord) -> bool {
	!record.items.is_empty() || record.ties.is_some()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ties {
	pub count: u32,
	pub sample: StatRef,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordGroup {
	pub id: String,
	pub title: String,
	pub table: Option<KindTable>,
	pub records: Vec<StatRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KindRow {
	pub id: String,
	pub label: String,
	pub hint: String,
	pub unit: Option<String>,
	pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KindTable {
	pub kinds: Vec<CountKind>,
	pub rows: Vec<KindRow>,
}
