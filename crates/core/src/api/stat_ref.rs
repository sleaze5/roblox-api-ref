use rapi_routes::{
	MemberKind, class_href, enum_href, enum_item_href, member_href, type_href, update_href,
};

use super::code::{CodeTone, CodeToken, linked, token};
use crate::model::StatRef;

pub fn stat_ref_label(item: &StatRef) -> String {
	match item {
		StatRef::Class { name } | StatRef::DataType { name } => name.clone(),
		StatRef::Enum { name } => format!("Enum.{name}"),
		StatRef::EnumItem { owner, name } => format!("Enum.{owner}.{name}"),
		StatRef::Parameter {
			owner,
			member,
			member_kind,
			name,
		} => format!("{owner}{}{member}({name})", member_kind.separator()),
		StatRef::Update { version, .. } => version.clone(),
		StatRef::Removed { label, .. } => label.clone(),
		StatRef::Member { kind, owner, name } => format!("{owner}{}{name}", kind.separator()),
	}
}

pub fn stat_ref_href(item: &StatRef) -> String {
	match item {
		StatRef::Class { name } => class_href(name),
		StatRef::DataType { name } => type_href(name),
		StatRef::Enum { name } => enum_href(name),
		StatRef::EnumItem { owner, name } => enum_item_href(owner, name),
		StatRef::Parameter {
			owner,
			member,
			member_kind,
			..
		} => member_href(owner, *member_kind, member),
		StatRef::Update { version, .. } => update_href(version),
		StatRef::Removed {
			version, anchor, ..
		} => format!("{}#{anchor}", update_href(version)),
		StatRef::Member { kind, owner, name } => member_href(owner, *kind, name),
	}
}

fn member_tokens(owner: &str, kind: MemberKind, name: &str) -> Vec<CodeToken> {
	vec![
		token(owner, CodeTone::SelfName),
		token(kind.separator().to_string(), CodeTone::Punctuation),
		token(
			name,
			if kind == MemberKind::Method {
				CodeTone::Function
			} else {
				CodeTone::Name
			},
		),
	]
}

fn unlinked_tokens(item: &StatRef) -> Vec<CodeToken> {
	match item {
		StatRef::Class { name } | StatRef::DataType { name } => vec![token(name, CodeTone::Type)],
		StatRef::Enum { name } => {
			vec![token("Enum.", CodeTone::Punctuation), token(name, CodeTone::Type)]
		}
		StatRef::EnumItem { owner, name } => vec![
			token("Enum.", CodeTone::Punctuation),
			token(owner, CodeTone::Type),
			token(".", CodeTone::Punctuation),
			token(name, CodeTone::Constant),
		],
		StatRef::Parameter {
			owner,
			member,
			member_kind,
			name,
		} => {
			let mut tokens = member_tokens(owner, *member_kind, member);
			tokens.push(token("(", CodeTone::Punctuation));
			tokens.push(token(name, CodeTone::Parameter));
			tokens.push(token(")", CodeTone::Punctuation));
			tokens
		}
		StatRef::Update { version, .. } => vec![token(version, CodeTone::Plain)],
		StatRef::Removed { label, .. } => vec![token(label, CodeTone::SelfName)],
		StatRef::Member { kind, owner, name } => member_tokens(owner, *kind, name),
	}
}

pub fn stat_ref_tokens(item: &StatRef) -> Vec<CodeToken> {
	linked(unlinked_tokens(item), &stat_ref_href(item), None)
}
