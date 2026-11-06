mod html;
mod links;
mod to_doc;

pub use html::{HtmlNode, parse_html};
pub use links::{ApiNames, Resolver, resolve_href};
pub use to_doc::{LinkResolver, normalize_text, to_doc};

use indexmap::IndexMap;

use crate::dump::{
	ApiDocs, ApiDocsEntry, class_docs_key, documentation, enum_docs_key, enum_item_docs_key,
	member_docs_key,
};
use crate::model::{Class, Doc, Enum, MemberDetail};

pub fn describe_html(names: &ApiNames<'_>, html: &str) -> Option<Doc> {
	to_doc(&parse_html(html), &Resolver { names })
}

fn describe(names: &ApiNames<'_>, docs: &ApiDocs, key: Option<&str>) -> Option<Doc> {
	describe_html(names, documentation(docs, key)?)
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
	let key = member_docs_key(class_name, &member.name);
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
	let entry = docs.get(&key);
	let parameters = member
		.parameters()
		.iter()
		.map(|parameter| {
			describe(
				names,
				docs,
				entry.and_then(|entry| entry.parameter(&parameter.name)),
			)
		})
		.collect();
	let returns = describe(names, docs, entry.and_then(ApiDocsEntry::first_return));
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
				let description = describe(&names, docs, Some(&class_docs_key(&item.name)));
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
				let description = describe(&names, docs, Some(&enum_docs_key(&item.name)));
				let items = item
					.items
					.iter()
					.map(|entry| {
						describe(
							&names,
							docs,
							Some(&enum_item_docs_key(&item.name, &entry.name)),
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
