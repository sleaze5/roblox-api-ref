mod globals;
mod lexical;
mod roles;
mod semantic;
mod ty;

use full_moon::LuaVersion;
use full_moon::tokenizer::{Lexer, LexerResult};
use full_moon::visitors::Visitor;

use self::lexical::token_tone;
use self::roles::Roles;
use self::semantic::Semantic;
use crate::api::code::CodeTone;
use crate::generate::docs::ApiNames;
use crate::model::SampleToken;

fn semantic_roles(names: &ApiNames<'_>, source: &str) -> Roles {
	let Ok(ast) = full_moon::parse_fallible(source, LuaVersion::luau()).into_result() else {
		return Roles::default();
	};
	let mut semantic = Semantic::new(names);
	semantic.visit_ast(&ast);
	semantic.roles
}

fn push(tokens: &mut Vec<SampleToken>, token: SampleToken) {
	match tokens.last_mut() {
		Some(last) if last.tone == token.tone => {
			last.text.push_str(&token.text);
		}
		_ => tokens.push(token),
	}
}

pub fn highlight(names: &ApiNames<'_>, source: &str) -> Vec<SampleToken> {
	let plain = || {
		vec![SampleToken {
			text: source.to_owned(),
			tone: CodeTone::Plain,
		}]
	};
	let LexerResult::Ok(lexed) = Lexer::new(source, LuaVersion::luau()).collect() else {
		return plain();
	};
	let mut roles = semantic_roles(names, source);
	let mut tokens = Vec::new();
	for token in lexed {
		let text = token.to_string();
		if text.is_empty() {
			continue;
		}
		let role = roles.take(token.start_position().bytes());
		let tone = if text.trim().is_empty() {
			CodeTone::Plain
		} else {
			role.unwrap_or_else(|| token_tone(token.token_type()))
		};
		push(&mut tokens, SampleToken { text, tone });
	}
	tokens
}

#[cfg(test)]
mod tests {
	use indexmap::IndexMap;

	use super::*;
	use crate::model::{DataType, DataTypeCategory, Enum};

	fn tones(source: &str) -> Vec<(String, CodeTone)> {
		let classes = IndexMap::new();
		let mut enums = IndexMap::new();
		enums.insert(
			"KeyCode".to_owned(),
			Enum {
				name: "KeyCode".to_owned(),
				history: None,
				tags: Vec::new(),
				items: Vec::new(),
				removed_items: None,
				used_by: Vec::new(),
				description: None,
			},
		);
		let mut data_types = IndexMap::new();
		data_types.insert(
			"Vector3".to_owned(),
			DataType {
				name: "Vector3".to_owned(),
				category: DataTypeCategory::DataType,
				used_by: Vec::new(),
			},
		);
		let names = ApiNames {
			classes: &classes,
			enums: &enums,
			data_types: &data_types,
		};
		highlight(&names, source)
			.into_iter()
			.filter(|token| token.tone != CodeTone::Plain)
			.map(|token| (token.text, token.tone))
			.collect()
	}

	fn tone(text: &str, tone: CodeTone) -> (String, CodeTone) {
		(text.to_owned(), tone)
	}

	#[test]
	fn colors_keywords_literals_and_comments() {
		assert_eq!(
			tones("local x = \"a\" .. 1 -- note\nif x == nil then return end"),
			vec![
				tone("local", CodeTone::Keyword),
				tone("=", CodeTone::Keyword),
				tone("\"a\"", CodeTone::String),
				tone("..", CodeTone::Keyword),
				tone("1", CodeTone::Constant),
				tone("-- note", CodeTone::Comment),
				tone("if", CodeTone::Keyword),
				tone("==", CodeTone::Keyword),
				tone("nil", CodeTone::Constant),
				tone("then", CodeTone::Keyword),
				tone("return", CodeTone::Keyword),
				tone("end", CodeTone::Keyword),
			]
		);
	}

	#[test]
	fn colors_calls_globals_data_types_and_enums() {
		assert_eq!(
			tones("game:GetService(\"Players\")\nprint(Vector3.new(), Enum.KeyCode.E)"),
			vec![
				tone("game", CodeTone::Constant),
				tone("GetService", CodeTone::Function),
				tone("\"Players\"", CodeTone::String),
				tone("print", CodeTone::Function),
				tone("Vector3", CodeTone::Type),
				tone("new", CodeTone::Function),
				tone("KeyCode", CodeTone::Type),
				tone("E", CodeTone::Constant),
			]
		);
	}

	#[test]
	fn colors_declarations_and_annotations() {
		assert_eq!(
			tones("type Id = number\nlocal function go(id: Id): string end\ncontinue"),
			vec![
				tone("type", CodeTone::Keyword),
				tone("Id", CodeTone::Type),
				tone("=", CodeTone::Keyword),
				tone("number", CodeTone::Type),
				tone("local", CodeTone::Keyword),
				tone("function", CodeTone::Keyword),
				tone("go", CodeTone::Function),
				tone("Id", CodeTone::Type),
				tone("string", CodeTone::Type),
				tone("end", CodeTone::Keyword),
				tone("continue", CodeTone::Keyword),
			]
		);
	}

	#[test]
	fn local_names_shadow_globals() {
		assert_eq!(
			tones("local game = 1\nprint(game)"),
			vec![
				tone("local", CodeTone::Keyword),
				tone("=", CodeTone::Keyword),
				tone("1", CodeTone::Constant),
				tone("print", CodeTone::Function),
			]
		);
	}
}
