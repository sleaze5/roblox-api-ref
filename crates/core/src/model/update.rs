use rapi_routes::OwnerKind;
use serde::{Deserialize, Serialize, Serializer};

use super::{Doc, EnumItem, Member};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DetailTone {
	Text,
	Code,
	Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DetailValue {
	pub text: String,
	pub tone: DetailTone,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChangeDetail {
	Value {
		label: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		before: Option<DetailValue>,
		#[serde(skip_serializing_if = "Option::is_none")]
		after: Option<DetailValue>,
	},
	Tags {
		label: String,
		tags: Vec<String>,
	},
}

impl ChangeDetail {
	pub fn label(&self) -> &str {
		match self {
			Self::Value { label, .. } | Self::Tags { label, .. } => label,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DescriptionDetail {
	pub part: DescriptionPart,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameter: Option<String>,
	pub status: ChangeStatus,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<Doc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<Doc>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum EntryValues {
	Member {
		#[serde(skip_serializing_if = "Option::is_none")]
		before: Option<Box<Member>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		after: Option<Box<Member>>,
	},
	EnumItem {
		#[serde(skip_serializing_if = "Option::is_none")]
		before: Option<Box<EnumItem>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		after: Option<Box<EnumItem>>,
	},
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEntry {
	pub status: ChangeStatus,
	pub owner: String,
	pub name: String,
	pub details: Vec<ChangeDetail>,
	pub descriptions: Vec<DescriptionDetail>,
	#[serde(flatten)]
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnerChanges {
	#[serde(serialize_with = "owner_kind")]
	pub kind: OwnerKind,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ChangeStatus>,
	pub details: Vec<ChangeDetail>,
	pub descriptions: Vec<DescriptionDetail>,
	pub entries: Vec<ChangeEntry>,
}

pub fn owner_kind<S: Serializer>(kind: &OwnerKind, serializer: S) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(kind.name())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Release {
	pub version: String,
	pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
	pub version: String,
	pub hash: String,
	pub published: String,
	pub previous: Release,
	pub counts: ChangeCounts,
	pub description_count: u32,
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
