use rapi_routes::{class_href, enum_href, type_href};

use super::code::{CodeToken, CodeTone, linked, linked_token, token};
use crate::model::{ApiType, TypeCategory};

pub fn luau_name(name: &str) -> &str {
	match name {
		"int" | "int64" | "float" | "double" => "number",
		"bool" => "boolean",
		"Variant" => "any",
		"ContentId" | "ProtectedString" | "BinaryString" => "string",
		"CoordinateFrame" | "OptionalCoordinateFrame" => "CFrame",
		other => other,
	}
}

pub fn is_number_type(ty: &ApiType) -> bool {
	matches!(
		ty.name.as_str(),
		"int" | "int64" | "float" | "double" | "number"
	)
}

pub fn is_string_type(ty: &ApiType) -> bool {
	matches!(
		ty.name.as_str(),
		"string" | "ContentId" | "ProtectedString" | "BinaryString"
	)
}

pub fn is_boolean_type(ty: &ApiType) -> bool {
	ty.name == "bool"
}

fn group_tokens(name: &str) -> Option<Vec<CodeToken>> {
	use CodeTone::{Punctuation, Type};
	Some(match name {
		"null" | "void" => vec![token("()", Punctuation)],
		"Array" => vec![
			token("{ ", Punctuation),
			token("any", Type),
			token(" }", Punctuation),
		],
		"Dictionary" | "Map" => vec![
			token("{ [", Punctuation),
			token(
				if name == "Dictionary" {
					"string"
				} else {
					"any"
				},
				Type,
			),
			token("]: ", Punctuation),
			token("any", Type),
			token(" }", Punctuation),
		],
		"Objects" | "Instances" => vec![
			token("{ ", Punctuation),
			token("Instance", Type),
			token(" }", Punctuation),
		],
		"Function" => vec![
			token("(...", Punctuation),
			token("any", Type),
			token(") -> ...", Punctuation),
			token("any", Type),
		],
		_ => return None,
	})
}

fn tuple_tokens(ty: &ApiType) -> Vec<CodeToken> {
	let members = ty.members.as_deref().unwrap_or_default();
	if members.is_empty() {
		return linked(
			vec![
				token("...", CodeTone::Punctuation),
				token("any", CodeTone::Type),
			],
			&type_href("Tuple"),
			None,
		);
	}
	let mut tokens = vec![token("(", CodeTone::Punctuation)];
	for (index, member) in members.iter().enumerate() {
		if index > 0 {
			tokens.push(token(", ", CodeTone::Punctuation));
		}
		tokens.extend(type_tokens(member));
	}
	tokens.push(token(")", CodeTone::Punctuation));
	tokens
}

fn body_tokens(ty: &ApiType) -> Vec<CodeToken> {
	if ty.name == "Tuple" {
		return tuple_tokens(ty);
	}
	match ty.category {
		TypeCategory::Class => {
			return vec![linked_token(&ty.name, CodeTone::Type, class_href(&ty.name))];
		}
		TypeCategory::Enum => {
			return linked(
				vec![
					token("Enum.", CodeTone::Punctuation),
					token(&ty.name, CodeTone::Type),
				],
				&enum_href(&ty.name),
				None,
			);
		}
		_ => {}
	}
	let luau = luau_name(&ty.name);
	let title = (luau != ty.name).then_some(ty.name.as_str());
	if let Some(group) = group_tokens(&ty.name) {
		return linked(group, &type_href(&ty.name), Some(&ty.name));
	}
	linked(
		vec![token(luau, CodeTone::Type)],
		&type_href(&ty.name),
		title,
	)
}

pub fn type_tokens(ty: &ApiType) -> Vec<CodeToken> {
	let optional = ty.optional || ty.name == "OptionalCoordinateFrame";
	let body = body_tokens(ty);
	if !optional {
		return body;
	}
	if ty.name == "Function" {
		let mut tokens = vec![token("(", CodeTone::Punctuation)];
		tokens.extend(body);
		tokens.push(token(")?", CodeTone::Punctuation));
		return tokens;
	}
	let mut tokens = body;
	tokens.push(token("?", CodeTone::Punctuation));
	tokens
}
