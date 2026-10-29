use rapi_routes::{enum_href, enum_item_href, type_href};

use super::code::{CodeToken, CodeTone, linked, token};
use super::luau::{is_boolean_type, is_number_type, is_string_type, luau_name};
use crate::js::{js_number, js_round, json_string};
use crate::model::{ApiType, TypeCategory};

const CONSTRUCTOR_TYPES: [&str; 11] = [
	"Vector2",
	"Vector3",
	"Vector2int16",
	"Vector3int16",
	"Color3",
	"Rect",
	"UDim",
	"UDim2",
	"CFrame",
	"CoordinateFrame",
	"NumberRange",
];

const IDENTITY_CFRAME: &str = "0,0,0,1,0,0,0,1,0,0,0,1";

fn unquote(value: &str) -> &str {
	if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
		&value[1..value.len() - 1]
	} else {
		value
	}
}

fn numbers_in(value: &str) -> Vec<String> {
	value
		.replace(['{', '}'], " ")
		.split(|character: char| character.is_whitespace() || character == ',')
		.filter(|part| !part.is_empty())
		.map(str::to_owned)
		.collect()
}

pub fn js_parse_number(text: &str) -> Option<f64> {
	let trimmed = text.trim_matches(|character: char| character.is_whitespace());
	if trimmed.is_empty() {
		return Some(0.0);
	}
	if let Some(hex) = trimmed
		.strip_prefix("0x")
		.or_else(|| trimmed.strip_prefix("0X"))
	{
		#[expect(clippy::cast_precision_loss, reason = "matches Number()")]
		return u64::from_str_radix(hex, 16).ok().map(|value| value as f64);
	}
	match trimmed {
		"Infinity" | "+Infinity" => return Some(f64::INFINITY),
		"-Infinity" => return Some(f64::NEG_INFINITY),
		_ => {}
	}
	let valid = trimmed
		.bytes()
		.all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'e' | b'E' | b'+' | b'-'));
	if !valid || trimmed.contains("inf") {
		return None;
	}
	trimmed.parse::<f64>().ok()
}

fn type_name_token(type_name: &str) -> CodeToken {
	CodeToken {
		href: Some(type_href(type_name)),
		..token(luau_name(type_name), CodeTone::Type)
	}
}

fn constructor_tokens(type_name: &str, args: &[String]) -> Vec<CodeToken> {
	let mut tokens = vec![
		type_name_token(type_name),
		token(".", CodeTone::Punctuation),
		token("new", CodeTone::Function),
		token("(", CodeTone::Punctuation),
	];
	for (index, arg) in args.iter().enumerate() {
		if index > 0 {
			tokens.push(token(", ", CodeTone::Punctuation));
		}
		tokens.push(token(arg, CodeTone::Constant));
	}
	tokens.push(token(")", CodeTone::Punctuation));
	tokens
}

fn static_tokens(type_name: &str, member: &str) -> Vec<CodeToken> {
	vec![
		type_name_token(type_name),
		token(".", CodeTone::Punctuation),
		token(member, CodeTone::Name),
	]
}

fn color_swatch(args: &[String]) -> Option<String> {
	if args.len() != 3 {
		return None;
	}
	let channels = args
		.iter()
		.map(|arg| js_parse_number(arg).map(|value| js_number(js_round(value * 255.0))))
		.collect::<Option<Vec<_>>>()?;
	Some(format!("rgb({})", channels.join(" ")))
}

fn is_vector_type(type_name: &str) -> bool {
	type_name == "Vector3" || type_name == "Vector2"
}

fn shorthand_tokens(type_name: &str, args: &[String]) -> Option<Vec<CodeToken>> {
	if luau_name(type_name) == "CFrame" && args.join(",") == IDENTITY_CFRAME {
		return Some(static_tokens(type_name, "identity"));
	}
	if is_vector_type(type_name) && args.iter().all(|arg| arg == "0") {
		return Some(static_tokens(type_name, "zero"));
	}
	if is_vector_type(type_name) && args.iter().all(|arg| arg == "1") {
		return Some(static_tokens(type_name, "one"));
	}
	if type_name == "NumberRange" && args.len() == 2 && args[0] == args[1] {
		return Some(constructor_tokens(type_name, &args[..1]));
	}
	None
}

fn data_type_tokens(type_name: &str, raw: &str) -> Vec<CodeToken> {
	let args = numbers_in(raw);
	if args
		.iter()
		.any(|arg| js_parse_number(arg).is_none_or(f64::is_nan))
	{
		return vec![token(raw, CodeTone::Plain)];
	}
	if let Some(shorthand) = shorthand_tokens(type_name, &args) {
		return shorthand;
	}
	let mut tokens = constructor_tokens(type_name, &args);
	if type_name == "Color3" {
		tokens[0].swatch = color_swatch(&args);
	}
	tokens
}

fn enum_tokens(enum_name: &str, item: &str) -> Vec<CodeToken> {
	let mut tokens = linked(
		vec![
			token("Enum.", CodeTone::Punctuation),
			token(enum_name, CodeTone::Type),
		],
		&enum_href(enum_name),
		None,
	);
	tokens.extend(linked(
		vec![
			token(".", CodeTone::Punctuation),
			token(item, CodeTone::Constant),
		],
		&enum_item_href(enum_name, item),
		None,
	));
	tokens
}

pub fn default_value_tokens(ty: &ApiType, raw: &str) -> Vec<CodeToken> {
	let value = raw.trim();
	if value == "nil" {
		return vec![token("nil", CodeTone::Constant)];
	}
	if ty.category == TypeCategory::Enum {
		return enum_tokens(&ty.name, unquote(value));
	}
	if is_string_type(ty) {
		return vec![token(json_string(unquote(raw)), CodeTone::String)];
	}
	if is_boolean_type(ty) {
		return vec![token(value.to_lowercase(), CodeTone::Constant)];
	}
	if is_number_type(ty) {
		return vec![token(value, CodeTone::Constant)];
	}
	if ty.category == TypeCategory::DataType && CONSTRUCTOR_TYPES.contains(&ty.name.as_str()) {
		return data_type_tokens(&ty.name, value);
	}
	vec![token(value, CodeTone::Plain)]
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::code::tokens_text;

	fn ty(category: TypeCategory, name: &str) -> ApiType {
		ApiType {
			category,
			name: name.to_owned(),
			optional: false,
			members: None,
		}
	}

	fn text(category: TypeCategory, name: &str, raw: &str) -> String {
		tokens_text(&default_value_tokens(&ty(category, name), raw))
	}

	#[test]
	fn writes_enum_defaults_as_luau_enum_items() {
		assert_eq!(
			text(TypeCategory::Enum, "PartType", "Block"),
			"Enum.PartType.Block"
		);
	}

	#[test]
	fn quotes_strings_and_lowercases_booleans() {
		assert_eq!(text(TypeCategory::Primitive, "string", "Part"), "\"Part\"");
		assert_eq!(text(TypeCategory::Primitive, "bool", "False"), "false");
	}

	#[test]
	fn uses_luau_shorthands_for_common_data_type_values() {
		assert_eq!(
			text(TypeCategory::DataType, "Vector3", "0, 0, 0"),
			"Vector3.zero"
		);
		assert_eq!(
			text(
				TypeCategory::DataType,
				"CFrame",
				"0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1"
			),
			"CFrame.identity"
		);
	}

	#[test]
	fn writes_other_data_type_values_as_constructors() {
		assert_eq!(
			text(TypeCategory::DataType, "Color3", "0.5, 0.25, 1"),
			"Color3.new(0.5, 0.25, 1)"
		);
	}

	#[test]
	fn adds_a_swatch_to_color3_values() {
		let tokens = default_value_tokens(&ty(TypeCategory::DataType, "Color3"), "1, 0, 0");
		assert_eq!(tokens[0].swatch.as_deref(), Some("rgb(255 0 0)"));
	}

	#[test]
	fn parses_numbers_like_the_number_function() {
		assert_eq!(js_parse_number("1e3"), Some(1000.0));
		assert_eq!(js_parse_number(" -0.5 "), Some(-0.5));
		assert_eq!(js_parse_number("abc"), None);
		assert_eq!(js_parse_number(""), Some(0.0));
	}
}
