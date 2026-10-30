use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoryKind {
	Added,
	Removed,
	Changed,
	Described,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct MemberCounts {
	pub added: u32,
	pub removed: u32,
	pub changed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Deprecation {
	Set,
	Clear,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryChanges {
	pub fields: Vec<String>,
	pub tags_added: u32,
	pub tags_removed: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deprecation: Option<Deprecation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deprecated: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct HistoryEvent {
	pub version: String,
	pub published: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HistoryEntry {
	pub version: String,
	pub published: String,
	pub kinds: Vec<HistoryKind>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<MemberCounts>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes: Option<HistoryChanges>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemHistory {
	pub entries: Vec<HistoryEntry>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub added: Option<HistoryEvent>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub added_before: Option<String>,
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub unrecorded: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changed: Option<HistoryEvent>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deprecated: Option<HistoryEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RemovedOwner {
	pub name: String,
	pub removed: HistoryEvent,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Removed<Value> {
	pub value: Value,
	pub removed: HistoryEvent,
}
