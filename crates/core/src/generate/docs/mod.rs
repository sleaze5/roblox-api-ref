mod html;
mod links;
mod to_doc;

pub use html::{HtmlNode, parse_html};
pub use links::{ApiNames, Resolver, resolve_href};
pub use to_doc::{LinkResolver, normalize_text, to_doc};

use indexmap::IndexMap;

use crate::dump::{ApiDocs, ApiDocsEntry};
use crate::model::{Class, Doc, Enum, MemberDetail};

pub fn describe_html(names: &ApiNames<'_>, html: &str) -> Option<Doc> {
	to_doc(&parse_html(html), &Resolver { names })
}

fn describe(names: &ApiNames<'_>, docs: &ApiDocs, key: Option<&str>) -> Option<Doc> {
	let html = key
		.and_then(|key| docs.get(key))
		.and_then(|entry| entry.documentation.as_deref())?;
	describe_html(names, html)
}

struct Descriptions {
	description: Option<Doc>,
	parameters: Vec<Option<Doc>>,
	returns: Option<Doc>,
}

fn member_descriptions(
	names: &ApiNames<'_>,
	docs: &ApiDocs,
	class_name: &str,
	member: &crate::model::Member,
) -> Descriptions {
	let key = format!("@roblox/globaltype/{class_name}.{}", member.name);
	let description = describe(names, docs, Some(&key));
	if !matches!(
		member.detail,
		MemberDetail::Method { .. } | MemberDetail::Callback { .. }
	) {
		return Descriptions {
			description,
			parameters: Vec::new(),
			returns: None,
		};
	}
	let entry = docs
		.get(&key)
		.cloned()
		.unwrap_or_else(ApiDocsEntry::default);
	let params: std::collections::HashMap<&str, &str> = entry
		.params
		.iter()
		.flatten()
		.map(|param| (param.name.as_str(), param.documentation.as_str()))
		.collect();
	let parameters = member
		.parameters()
		.iter()
		.map(|parameter| {
			describe(names, docs, params.get(parameter.name.as_str()).copied())
		})
		.collect();
	let returns = describe(
		names,
		docs,
		entry
			.returns
			.as_ref()
			.and_then(|returns| returns.first())
			.map(String::as_str),
	);
	Descriptions {
		description,
		parameters,
		returns,
	}
}

pub fn apply_api_docs(
	classes: &mut IndexMap<String, Class>,
	enums: &mut IndexMap<String, Enum>,
	data_types: &IndexMap<String, crate::model::DataType>,
	docs: &ApiDocs,
) {
	let (class_docs, enum_docs) = {
		let names = ApiNames {
			classes,
			enums,
			data_types,
		};
		let class_docs: Vec<(Option<Doc>, Vec<Descriptions>)> = classes
			.values()
			.map(|item| {
				let description = describe(
					&names,
					docs,
					Some(&format!("@roblox/globaltype/{}", item.name)),
				);
				let members = item
					.members
					.iter()
					.map(|member| member_descriptions(&names, docs, &item.name, member))
					.collect();
				(description, members)
			})
			.collect();
		let enum_docs: Vec<(Option<Doc>, Vec<Option<Doc>>)> = enums
			.values()
			.map(|item| {
				let description = describe(
					&names,
					docs,
					Some(&format!("@roblox/global/Enum.{}", item.name)),
				);
				let items = item
					.items
					.iter()
					.map(|entry| {
						describe(
							&names,
							docs,
							Some(&format!("@roblox/enum/{}.{}", item.name, entry.name)),
						)
					})
					.collect();
				(description, items)
			})
			.collect();
		(class_docs, enum_docs)
	};
	for (item, (description, members)) in classes.values_mut().zip(class_docs) {
		item.description = description;
		for (member, found) in item.members.iter_mut().zip(members) {
			member.description = found.description;
			for (parameter, description) in member.parameters_mut().iter_mut().zip(found.parameters)
			{
				parameter.description = description;
			}
			match &mut member.detail {
				MemberDetail::Method {
					return_description, ..
				}
				| MemberDetail::Callback {
					return_description, ..
				} => *return_description = found.returns,
				_ => {}
			}
		}
	}
	for (item, (description, items)) in enums.values_mut().zip(enum_docs) {
		item.description = description;
		for (entry, description) in item.items.iter_mut().zip(items) {
			entry.description = description;
		}
	}
}
