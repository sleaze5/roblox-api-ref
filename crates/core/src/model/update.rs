use rapi_routes::OwnerKind;
use serde::{Deserialize, Serialize};

use super::{Doc, EnumItem, Member};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChangeStatus {
	Added,
	Removed,
	Changed,
}

impl ChangeStatus {
	pub const ALL: [Self; 3] = [Self::Added, Self::Removed, Self::Changed];

	pub const fn name(self) -> &'static str {
		match self {
			Self::Added => "Added",
			Self::Removed => "Removed",
			Self::Changed => "Changed",
		}
	}

	pub const fn word(self) -> &'static str {
		match self {
			Self::Added => "added",
			Self::Removed => "removed",
			Self::Changed => "changed",
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetailTone {
	Text,
	Code,
	Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailValue {
	pub text: String,
	pub tone: DetailTone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetailField {
	Security,
	ReadSecurity,
	WriteSecurity,
	Capabilities,
	ReadCapabilities,
	WriteCapabilities,
	ThreadSafety,
	PropertyCategory,
	Serialization,
	UseInstead,
	Superclass,
	MemoryCategory,
	LegacyNames,
	SimulationAccess,
	Other(String),
}

impl DetailField {
	pub fn label(&self) -> &str {
		match self {
			Self::Security => "Security",
			Self::ReadSecurity => "Read security",
			Self::WriteSecurity => "Write security",
			Self::Capabilities => "Capabilities",
			Self::ReadCapabilities => "Read capabilities",
			Self::WriteCapabilities => "Write capabilities",
			Self::ThreadSafety => "Thread safety",
			Self::PropertyCategory => "Property category",
			Self::Serialization => "Serialization",
			Self::UseInstead => "Use instead",
			Self::Superclass => "Superclass",
			Self::MemoryCategory => "Memory category",
			Self::LegacyNames => "Legacy names",
			Self::SimulationAccess => "Simulation access",
			Self::Other(field) => field,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagChange {
	Initial,
	Added,
	Removed,
}

impl TagChange {
	pub const fn label(self) -> &'static str {
		match self {
			Self::Initial => "Tags",
			Self::Added => "Added tags",
			Self::Removed => "Removed tags",
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeDetail {
	Value {
		field: DetailField,
		before: Option<DetailValue>,
		after: Option<DetailValue>,
	},
	Tags {
		change: TagChange,
		tags: Vec<String>,
	},
}

impl ChangeDetail {
	pub fn label(&self) -> &str {
		match self {
			Self::Value { field, .. } => field.label(),
			Self::Tags { change, .. } => change.label(),
		}
	}

	pub fn tags(&self, changes: &[TagChange]) -> &[String] {
		match self {
			Self::Tags { change, tags } if changes.contains(change) => tags,
			_ => &[],
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DescriptionPart {
	Description,
	Parameter,
	Return,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptionDetail {
	pub part: DescriptionPart,
	pub parameter: Option<String>,
	pub status: ChangeStatus,
	pub before: Option<Doc>,
	pub after: Option<Doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EntryValues {
	Member {
		before: Option<Box<Member>>,
		after: Option<Box<Member>>,
	},
	EnumItem {
		before: Option<Box<EnumItem>>,
		after: Option<Box<EnumItem>>,
	},
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChangeEntry {
	pub status: ChangeStatus,
	pub owner: String,
	pub name: String,
	pub details: Vec<ChangeDetail>,
	pub descriptions: Vec<DescriptionDetail>,
	pub values: EntryValues,
}

impl ChangeEntry {
	pub const fn owner_kind(&self) -> OwnerKind {
		match self.values {
			EntryValues::Member { .. } => OwnerKind::Class,
			EntryValues::EnumItem { .. } => OwnerKind::Enum,
		}
	}

	pub const fn is_member(&self) -> bool {
		matches!(self.values, EntryValues::Member { .. })
	}

	pub fn after_tags(&self) -> Option<&[String]> {
		match &self.values {
			EntryValues::Member { after, .. } => {
				after.as_ref().map(|member| member.tags.as_slice())
			}
			EntryValues::EnumItem { after, .. } => after.as_ref().map(|item| item.tags.as_slice()),
		}
	}

	pub fn before_tags(&self) -> Option<&[String]> {
		match &self.values {
			EntryValues::Member { before, .. } => {
				before.as_ref().map(|member| member.tags.as_slice())
			}
			EntryValues::EnumItem { before, .. } => {
				before.as_ref().map(|item| item.tags.as_slice())
			}
		}
	}

	pub fn has_values(&self) -> (bool, bool) {
		match &self.values {
			EntryValues::Member { before, after } => (before.is_some(), after.is_some()),
			EntryValues::EnumItem { before, after } => (before.is_some(), after.is_some()),
		}
	}

	pub fn is_api_change(&self) -> bool {
		self.status != ChangeStatus::Changed
			|| !self.details.is_empty()
			|| match &self.values {
				EntryValues::Member { before, after } => before != after,
				EntryValues::EnumItem { before, after } => before != after,
			}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnerChanges {
	pub kind: OwnerKind,
	pub name: String,
	pub status: Option<ChangeStatus>,
	pub details: Vec<ChangeDetail>,
	pub descriptions: Vec<DescriptionDetail>,
	pub entries: Vec<ChangeEntry>,
	pub tags: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChangeCounts {
	pub added: u32,
	pub removed: u32,
	pub changed: u32,
}

impl ChangeCounts {
	pub const fn get(&self, status: ChangeStatus) -> u32 {
		match status {
			ChangeStatus::Added => self.added,
			ChangeStatus::Removed => self.removed,
			ChangeStatus::Changed => self.changed,
		}
	}

	pub const fn bump(&mut self, status: ChangeStatus) {
		match status {
			ChangeStatus::Added => self.added += 1,
			ChangeStatus::Removed => self.removed += 1,
			ChangeStatus::Changed => self.changed += 1,
		}
	}

	pub const fn total(&self) -> u32 {
		self.added + self.removed + self.changed
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
	pub version: String,
	pub hash: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Update {
	pub version: String,
	pub hash: String,
	pub published: String,
	pub previous: Release,
	pub counts: ChangeCounts,
	pub description_count: Option<u32>,
	pub owners: Vec<OwnerChanges>,
}

impl Update {
	pub fn release(&self) -> Release {
		Release {
			version: self.version.clone(),
			hash: self.hash.clone(),
		}
	}
}
