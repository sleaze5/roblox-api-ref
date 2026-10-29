use indexmap::IndexMap;
use rapi_routes::decode_uri_component;
use url::Url;

use super::to_doc::LinkResolver;
use crate::js::js_trim;
use crate::model::{Class, DataType, DocLink, Enum};

pub struct ApiNames<'a> {
	pub classes: &'a IndexMap<String, Class>,
	pub enums: &'a IndexMap<String, Enum>,
	pub data_types: &'a IndexMap<String, DataType>,
}

impl ApiNames<'_> {
	pub fn member_link(&self, class_name: &str, member_name: &str) -> Option<DocLink> {
		let mut current = self.classes.get(class_name);
		while let Some(item) = current {
			if let Some(member) = item
				.members
				.iter()
				.find(|member| member.name == member_name)
			{
				return Some(DocLink::Member {
					owner: item.name.clone(),
					member: member.kind(),
					name: member.name.clone(),
				});
			}
			current = item
				.superclass
				.as_ref()
				.and_then(|name| self.classes.get(name));
		}
		None
	}

	pub fn enum_link(&self, enum_name: &str, item_name: Option<&str>) -> Option<DocLink> {
		let item = self.enums.get(enum_name)?;
		let Some(item_name) = item_name else {
			return Some(DocLink::Enum {
				name: enum_name.to_owned(),
			});
		};
		item.items
			.iter()
			.any(|entry| entry.name == item_name)
			.then(|| DocLink::EnumItem {
				owner: enum_name.to_owned(),
				name: item_name.to_owned(),
			})
	}

	pub fn type_link(&self, name: &str) -> Option<DocLink> {
		if self.classes.contains_key(name) {
			return Some(DocLink::Class {
				name: name.to_owned(),
			});
		}
		self.data_types.contains_key(name).then(|| DocLink::Type {
			name: name.to_owned(),
		})
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeContext<'a> {
	Class(&'a str),
	Enum(&'a str),
}

fn is_word(text: &str) -> bool {
	!text.is_empty()
		&& text
			.bytes()
			.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn context_link(
	names: &ApiNames<'_>,
	name: &str,
	call: bool,
	context: Option<CodeContext<'_>>,
) -> Option<DocLink> {
	match context {
		Some(CodeContext::Enum(owner)) => {
			if call {
				None
			} else {
				names.enum_link(owner, Some(name))
			}
		}
		Some(CodeContext::Class(owner)) => {
			let link = names.member_link(owner, name);
			if call
				&& matches!(&link, Some(DocLink::Member { member, .. }) if *member != rapi_routes::MemberKind::Method)
			{
				None
			} else {
				link
			}
		}
		None => None,
	}
}

pub fn resolve_code(
	names: &ApiNames<'_>,
	text: &str,
	context: Option<CodeContext<'_>>,
) -> Option<DocLink> {
	let trimmed = js_trim(text);
	let call = trimmed.ends_with("()");
	let name = trimmed.strip_suffix("()").unwrap_or(trimmed);
	if let Some(rest) = name.strip_prefix("Enum.") {
		let (enum_name, item) = rest
			.split_once('.')
			.map_or((rest, None), |(a, b)| (a, Some(b)));
		if is_word(enum_name) && item.is_none_or(is_word) {
			return names.enum_link(enum_name, item);
		}
	}
	if let Some(index) = name.find(['.', ':']) {
		let (owner, member) = (&name[..index], &name[index + 1..]);
		if is_word(owner) && is_word(member) {
			return names.member_link(owner, member);
		}
	}
	if !is_word(name) {
		return None;
	}
	names
		.type_link(name)
		.or_else(|| context_link(names, name, call, context))
}

const DOCS_ORIGIN: &str = "https://create.roblox.com";
const REFERENCE_BASE: &str = "https://create.roblox.com/docs/reference/engine/classes/";

fn absolute_url(href: &str) -> Option<Url> {
	let source = if href.starts_with('/') && !href.starts_with("//") {
		format!("/docs{href}")
	} else {
		href.to_owned()
	};
	let base = Url::parse(REFERENCE_BASE).ok()?;
	let url = base.join(&source).ok()?;
	matches!(url.scheme(), "https" | "http").then_some(url)
}

fn docs_path(pathname: &str) -> String {
	let path = pathname
		.strip_prefix("/docs/en-us/")
		.map_or_else(|| pathname.to_owned(), |rest| format!("/docs/{rest}"));
	let path = path.strip_suffix(".md").unwrap_or(&path);
	path.strip_suffix("/index").unwrap_or(path).to_owned()
}

fn reference_link(
	names: &ApiNames<'_>,
	section: &str,
	name: &str,
	anchor: &str,
) -> Option<DocLink> {
	match section {
		"classes" => (!anchor.is_empty())
			.then(|| names.member_link(name, anchor))
			.flatten()
			.or_else(|| names.type_link(name)),
		"enums" => names
			.enum_link(name, (!anchor.is_empty()).then_some(anchor))
			.or_else(|| names.enum_link(name, None)),
		_ => names.data_types.contains_key(name).then(|| DocLink::Type {
			name: name.to_owned(),
		}),
	}
}

fn reference_match(pathname: &str) -> Option<(&str, &str)> {
	let rest = pathname.strip_prefix("/docs/reference/engine/")?;
	let (section, name) = rest.split_once('/')?;
	(matches!(section, "classes" | "enums" | "datatypes") && is_word(name))
		.then_some((section, name))
}

pub fn resolve_href(names: &ApiNames<'_>, href: &str) -> Option<DocLink> {
	let mut url = absolute_url(js_trim(href))?;
	if url.origin().ascii_serialization() == DOCS_ORIGIN {
		let path = docs_path(url.path());
		url.set_path(&path);
		let internal = reference_match(url.path()).and_then(|(section, name)| {
			let hash = url.fragment().unwrap_or_default();
			let anchor = decode_uri_component(hash).unwrap_or_else(|| hash.to_owned());
			reference_link(names, section, name, &anchor)
		});
		if internal.is_some() {
			return internal;
		}
	}
	Some(DocLink::External {
		url: url.as_str().to_owned(),
	})
}

pub struct Resolver<'a> {
	pub names: &'a ApiNames<'a>,
	pub context: Option<CodeContext<'a>>,
}

impl LinkResolver for Resolver<'_> {
	fn href(&self, href: &str) -> Option<DocLink> {
		resolve_href(self.names, href)
	}

	fn code(&self, text: &str) -> Option<DocLink> {
		resolve_code(self.names, text, self.context)
	}
}
