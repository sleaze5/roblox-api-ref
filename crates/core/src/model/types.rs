use indexmap::IndexMap;
use rapi_routes::{FACET_KINDS, FacetKind, MemberKind};
use serde::{Deserialize, Serialize, Serializer};

use super::{CodeSample, Doc, ItemHistory, Removed, RemovedOwner};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Security {
	None,
	PluginSecurity,
	LocalUserSecurity,
	WritePlayerSecurity,
	RobloxScriptSecurity,
	RobloxSecurity,
	NotAccessibleSecurity,
}

impl Security {
	pub const ALL: [Self; 7] = [
		Self::None,
		Self::PluginSecurity,
		Self::LocalUserSecurity,
		Self::WritePlayerSecurity,
		Self::RobloxScriptSecurity,
		Self::RobloxSecurity,
		Self::NotAccessibleSecurity,
	];

	pub fn parse(text: &str) -> Option<Self> {
		Self::ALL.into_iter().find(|level| level.name() == text)
	}

	pub const fn name(self) -> &'static str {
		match self {
			Self::None => "None",
			Self::PluginSecurity => "PluginSecurity",
			Self::LocalUserSecurity => "LocalUserSecurity",
			Self::WritePlayerSecurity => "WritePlayerSecurity",
			Self::RobloxScriptSecurity => "RobloxScriptSecurity",
			Self::RobloxSecurity => "RobloxSecurity",
			Self::NotAccessibleSecurity => "NotAccessibleSecurity",
		}
	}

	pub const fn rank(self) -> usize {
		self as usize
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TypeCategory {
	Primitive,
	Class,
	DataType,
	Enum,
	Group,
}

impl TypeCategory {
	pub const fn name(self) -> &'static str {
		match self {
			Self::Primitive => "Primitive",
			Self::Class => "Class",
			Self::DataType => "DataType",
			Self::Enum => "Enum",
			Self::Group => "Group",
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct ApiType {
	pub category: TypeCategory,
	pub name: String,
	pub optional: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<Vec<ApiType>>,
}

impl ApiType {
	pub fn flatten(&self) -> Vec<&Self> {
		let mut types = vec![self];
		for member in self.members.iter().flatten() {
			types.extend(member.flatten());
		}
		types
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Parameter {
	pub name: String,
	#[serde(rename = "type")]
	pub ty: ApiType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Doc>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Capabilities {
	pub read: Vec<String>,
	pub write: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PreferredTarget {
	pub owner: String,
	#[serde(serialize_with = "member_kind")]
	pub kind: MemberKind,
}

pub fn member_kind<S: Serializer>(kind: &MemberKind, serializer: S) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(kind.name())
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub history: Option<ItemHistory>,
	pub tags: Vec<String>,
	pub security: Security,
	pub thread_safety: String,
	pub capabilities: Capabilities,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_target: Option<PreferredTarget>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Doc>,
	#[serde(skip)]
	pub sample: Option<CodeSample>,
	#[serde(flatten)]
	pub detail: MemberDetail,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all_fields = "camelCase")]
pub enum MemberDetail {
	Property {
		#[serde(rename = "type")]
		ty: ApiType,
		category: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		default: Option<String>,
		write_security: Security,
		can_load: bool,
		can_save: bool,
		simulation_access: bool,
	},
	Method {
		parameters: Vec<Parameter>,
		returns: ApiType,
		#[serde(skip_serializing_if = "Option::is_none")]
		return_description: Option<Doc>,
		simulation_access: bool,
	},
	Event {
		parameters: Vec<Parameter>,
	},
	Callback {
		parameters: Vec<Parameter>,
		returns: ApiType,
		#[serde(skip_serializing_if = "Option::is_none")]
		return_description: Option<Doc>,
	},
}

impl Member {
	pub const fn kind(&self) -> MemberKind {
		match self.detail {
			MemberDetail::Property { .. } => MemberKind::Property,
			MemberDetail::Method { .. } => MemberKind::Method,
			MemberDetail::Event { .. } => MemberKind::Event,
			MemberDetail::Callback { .. } => MemberKind::Callback,
		}
	}

	pub fn parameters(&self) -> &[Parameter] {
		match &self.detail {
			MemberDetail::Property { .. } => &[],
			MemberDetail::Method { parameters, .. }
			| MemberDetail::Event { parameters }
			| MemberDetail::Callback { parameters, .. } => parameters,
		}
	}

	pub fn parameters_mut(&mut self) -> &mut [Parameter] {
		match &mut self.detail {
			MemberDetail::Property { .. } => &mut [],
			MemberDetail::Method { parameters, .. }
			| MemberDetail::Event { parameters }
			| MemberDetail::Callback { parameters, .. } => parameters,
		}
	}

	pub const fn returns(&self) -> Option<&ApiType> {
		match &self.detail {
			MemberDetail::Method { returns, .. } | MemberDetail::Callback { returns, .. } => {
				Some(returns)
			}
			_ => None,
		}
	}

	pub const fn return_description(&self) -> Option<&Doc> {
		match &self.detail {
			MemberDetail::Method {
				return_description, ..
			}
			| MemberDetail::Callback {
				return_description, ..
			} => return_description.as_ref(),
			_ => None,
		}
	}

	pub const fn value_type(&self) -> Option<&ApiType> {
		match &self.detail {
			MemberDetail::Property { ty, .. } => Some(ty),
			_ => None,
		}
	}

	pub const fn write_security(&self) -> Security {
		match self.detail {
			MemberDetail::Property { write_security, .. } => write_security,
			_ => self.security,
		}
	}

	pub fn property_default(&self) -> Option<&str> {
		match &self.detail {
			MemberDetail::Property { default, .. } => default.as_deref(),
			_ => None,
		}
	}

	pub const fn simulation_access(&self) -> bool {
		match self.detail {
			MemberDetail::Property {
				simulation_access, ..
			}
			| MemberDetail::Method {
				simulation_access, ..
			} => simulation_access,
			_ => false,
		}
	}

	pub fn types(&self) -> Vec<&ApiType> {
		let mut types = Vec::new();
		if let Some(ty) = self.value_type() {
			types.push(ty);
		}
		types.extend(self.parameters().iter().map(|parameter| &parameter.ty));
		types.extend(self.returns());
		types.into_iter().flat_map(ApiType::flatten).collect()
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeUsage {
	pub class_name: String,
	pub member_name: String,
	#[serde(serialize_with = "member_kind")]
	pub kind: MemberKind,
	pub role: TypeRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum TypeRole {
	Value,
	Parameter,
	Return,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub history: Option<ItemHistory>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub superclass: Option<String>,
	pub memory_category: String,
	pub tags: Vec<String>,
	pub members: Vec<Member>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub removed_members: Option<Vec<Removed<Member>>>,
	pub subclasses: Vec<String>,
	pub used_by: Vec<TypeUsage>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Doc>,
	#[serde(skip)]
	pub sample: Option<CodeSample>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumItem {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub history: Option<ItemHistory>,
	pub value: i64,
	pub tags: Vec<String>,
	pub legacy_names: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Doc>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub history: Option<ItemHistory>,
	pub tags: Vec<String>,
	pub items: Vec<EnumItem>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub removed_items: Option<Vec<Removed<EnumItem>>>,
	pub used_by: Vec<TypeUsage>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Doc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum DataTypeCategory {
	Primitive,
	Group,
	DataType,
}

impl DataTypeCategory {
	pub const ALL: [Self; 3] = [Self::Primitive, Self::Group, Self::DataType];

	pub const fn name(self) -> &'static str {
		match self {
			Self::Primitive => "Primitive",
			Self::Group => "Group",
			Self::DataType => "DataType",
		}
	}

	pub const fn type_category(self) -> TypeCategory {
		match self {
			Self::Primitive => TypeCategory::Primitive,
			Self::Group => TypeCategory::Group,
			Self::DataType => TypeCategory::DataType,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataType {
	pub name: String,
	pub category: DataTypeCategory,
	pub used_by: Vec<TypeUsage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum FacetAccess {
	Read,
	Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UseKind {
	Class,
	Enum,
	EnumItem,
	Property,
	Method,
	Event,
	Callback,
}

impl UseKind {
	pub const ALL: [Self; 7] = [
		Self::Class,
		Self::Enum,
		Self::EnumItem,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
	];

	pub const fn name(self) -> &'static str {
		match self {
			Self::Class => "Class",
			Self::Enum => "Enum",
			Self::EnumItem => "EnumItem",
			Self::Property => "Property",
			Self::Method => "Method",
			Self::Event => "Event",
			Self::Callback => "Callback",
		}
	}

	pub const fn member(kind: MemberKind) -> Self {
		match kind {
			MemberKind::Property => Self::Property,
			MemberKind::Method => Self::Method,
			MemberKind::Event => Self::Event,
			MemberKind::Callback => Self::Callback,
		}
	}

	pub const fn member_kind(self) -> Option<MemberKind> {
		match self {
			Self::Property => Some(MemberKind::Property),
			Self::Method => Some(MemberKind::Method),
			Self::Event => Some(MemberKind::Event),
			Self::Callback => Some(MemberKind::Callback),
			_ => None,
		}
	}
}

impl Serialize for UseKind {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(self.name())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FacetUse {
	pub kind: UseKind,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub owner: String,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub access: Option<FacetAccess>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Facet {
	#[serde(serialize_with = "facet_kind")]
	pub kind: FacetKind,
	pub name: String,
	pub uses: Vec<FacetUse>,
}

pub fn facet_kind<S: Serializer>(kind: &FacetKind, serializer: S) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(kind.key())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Facets(pub [IndexMap<String, Facet>; 6]);

impl Facets {
	pub fn of(&self, kind: FacetKind) -> &IndexMap<String, Facet> {
		&self.0[kind.index()]
	}

	pub fn of_mut(&mut self, kind: FacetKind) -> &mut IndexMap<String, Facet> {
		&mut self.0[kind.index()]
	}
}

impl Serialize for Facets {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_map(FACET_KINDS.iter().map(|kind| (kind.key(), self.of(*kind))))
	}
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct RemovedOwners {
	pub classes: Vec<RemovedOwner>,
	pub enums: Vec<RemovedOwner>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
	pub version: String,
	pub hash: String,
	pub channel: String,
	pub published: String,
	pub classes: IndexMap<String, Class>,
	pub enums: IndexMap<String, Enum>,
	pub data_types: IndexMap<String, DataType>,
	pub facets: Facets,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub history_start: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub removed: Option<RemovedOwners>,
}

impl Api {
	pub fn removed_classes(&self) -> &[RemovedOwner] {
		self.removed
			.as_ref()
			.map_or(&[], |removed| &removed.classes)
	}

	pub fn removed_enums(&self) -> &[RemovedOwner] {
		self.removed.as_ref().map_or(&[], |removed| &removed.enums)
	}

	pub fn superclass_of(&self, item: &Class) -> Option<&Class> {
		item.superclass
			.as_ref()
			.and_then(|name| self.classes.get(name))
	}

	pub fn ancestors_of(&self, item: &Class) -> Vec<&Class> {
		let mut ancestors = Vec::new();
		let mut current = self.superclass_of(item);
		while let Some(ancestor) = current {
			ancestors.push(ancestor);
			current = self.superclass_of(ancestor);
		}
		ancestors
	}
}
