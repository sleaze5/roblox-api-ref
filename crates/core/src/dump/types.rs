use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpType {
	pub category: Option<String>,
	pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DumpTag {
	Name(String),
	Detail(DumpTagDetail),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpTagDetail {
	pub preferred_descriptor_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DumpCapabilities {
	List(Vec<String>),
	Split {
		#[serde(rename = "Read")]
		read: Option<Vec<String>>,
		#[serde(rename = "Write")]
		write: Option<Vec<String>>,
	},
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DumpSecurity {
	Single(String),
	Split {
		#[serde(rename = "Read")]
		read: String,
		#[serde(rename = "Write")]
		write: String,
	},
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DumpReturns {
	One(DumpType),
	Many(Vec<DumpType>),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpParameter {
	pub name: String,
	#[serde(rename = "Type")]
	pub ty: DumpType,
	pub default: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpSerialization {
	pub can_load: bool,
	pub can_save: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpMember {
	pub member_type: String,
	pub name: String,
	pub tags: Option<Vec<DumpTag>>,
	pub thread_safety: Option<String>,
	pub capabilities: Option<DumpCapabilities>,
	pub simulation_access: Option<bool>,
	pub category: Option<String>,
	pub default: Option<String>,
	pub value_type: Option<DumpType>,
	pub security: Option<DumpSecurity>,
	pub serialization: Option<DumpSerialization>,
	pub parameters: Option<Vec<DumpParameter>>,
	pub return_type: Option<DumpReturns>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClass {
	pub name: String,
	pub superclass: String,
	pub memory_category: String,
	pub tags: Option<Vec<DumpTag>>,
	pub members: Vec<DumpMember>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnumItem {
	pub name: String,
	pub value: i64,
	pub tags: Option<Vec<DumpTag>>,
	pub legacy_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnum {
	pub name: String,
	pub tags: Option<Vec<DumpTag>>,
	pub items: Vec<DumpEnumItem>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
	pub classes: Vec<DumpClass>,
	pub enums: Vec<DumpEnum>,
}
