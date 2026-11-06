use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ApiDocsParam {
	pub name: String,
	pub documentation: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ApiDocsEntry {
	pub documentation: Option<String>,
	pub params: Option<Vec<ApiDocsParam>>,
	pub returns: Option<Vec<String>>,
	pub code_sample: Option<String>,
}

impl ApiDocsEntry {
	pub fn parameter(&self, name: &str) -> Option<&str> {
		self.params
			.iter()
			.flatten()
			.rfind(|param| param.name == name)
			.map(|param| param.documentation.as_str())
	}

	pub fn first_return(&self) -> Option<&str> {
		self.returns.as_ref()?.first().map(String::as_str)
	}
}

pub type ApiDocs = HashMap<String, ApiDocsEntry>;

pub fn class_docs_key(class: &str) -> String {
	format!("@roblox/globaltype/{class}")
}

pub fn member_docs_key(class: &str, member: &str) -> String {
	format!("@roblox/globaltype/{class}.{member}")
}

pub fn enum_docs_key(name: &str) -> String {
	format!("@roblox/global/Enum.{name}")
}

pub fn enum_item_docs_key(name: &str, item: &str) -> String {
	format!("@roblox/enum/{name}.{item}")
}

pub fn documentation<'a>(docs: &'a ApiDocs, key: Option<&str>) -> Option<&'a str> {
	key.and_then(|key| docs.get(key))
		.and_then(|entry| entry.documentation.as_deref())
		.filter(|text| !text.is_empty())
}
