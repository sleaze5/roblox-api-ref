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
}

pub type ApiDocs = HashMap<String, ApiDocsEntry>;

pub fn documentation<'a>(docs: &'a ApiDocs, key: Option<&str>) -> Option<&'a str> {
	key.and_then(|key| docs.get(key))
		.and_then(|entry| entry.documentation.as_deref())
}
