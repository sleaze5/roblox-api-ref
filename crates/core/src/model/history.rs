#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HistoryKind {
	Added,
	Removed,
	Changed,
	Described,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemberCounts {
	pub added: u32,
	pub removed: u32,
	pub changed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deprecation {
	Set,
	Clear,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryChanges {
	pub fields: Vec<String>,
	pub tags_added: u32,
	pub tags_removed: u32,
	pub deprecation: Option<Deprecation>,
	pub deprecated: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HistoryEvent {
	pub version: String,
	pub published: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryEntry {
	pub version: String,
	pub published: String,
	pub kinds: Vec<HistoryKind>,
	pub members: Option<MemberCounts>,
	pub changes: Option<HistoryChanges>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemHistory {
	pub entries: Vec<HistoryEntry>,
	pub added: Option<HistoryEvent>,
	pub added_before: Option<String>,
	pub unrecorded: bool,
	pub changed: Option<HistoryEvent>,
	pub deprecated: Option<HistoryEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedOwner {
	pub name: String,
	pub removed: HistoryEvent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Removed<Value> {
	pub value: Value,
	pub removed: HistoryEvent,
}
