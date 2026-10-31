mod calls;
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
		Some(last) if last.tone == token.tone && last.link.is_none() && token.link.is_none() => {
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
			link: None,
		}]
	};
	let lexed = match Lexer::new(source, LuaVersion::luau()).collect() {
		LexerResult::Ok(tokens) => tokens,
		_ => return plain(),
	};
	let mut roles = semantic_roles(names, source);
	let mut tokens = Vec::new();
	for token in lexed {
		let text = token.to_string();
		if text.is_empty() {
			continue;
		}
		let role = roles.take(token.start_position().bytes());
		let tone = role
			.as_ref()
			.map_or_else(|| token_tone(token.token_type()), |role| role.tone);
		let tone = if text.trim().is_empty() {
			CodeTone::Plain
		} else {
			tone
		};
		push(
			&mut tokens,
			SampleToken {
				text,
				tone,
				link: role.and_then(|role| role.link),
			},
		);
	}
	tokens
}
