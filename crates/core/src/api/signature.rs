use rapi_routes::{class_href, type_href};

use super::code::{CodeToken, CodeTone, linked, linked_token, token};
use super::default_value::default_value_tokens;
use super::luau::type_tokens;
use crate::model::{ApiType, Member, MemberDetail, Parameter, TypeCategory};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureParts {
	pub owner: Vec<CodeToken>,
	pub head: Vec<CodeToken>,
	pub parameters: Option<Vec<Vec<CodeToken>>>,
	pub tail: Vec<CodeToken>,
}

impl SignatureParts {
	#[must_use]
	pub fn map(&self, map: impl Fn(&[CodeToken]) -> Vec<CodeToken>) -> Self {
		Self {
			owner: map(&self.owner),
			head: map(&self.head),
			parameters: self
				.parameters
				.as_ref()
				.map(|parameters| parameters.iter().map(|parameter| map(parameter)).collect()),
			tail: map(&self.tail),
		}
	}
}

pub fn parameter_tokens(parameter: &Parameter) -> Vec<CodeToken> {
	if parameter.ty.name == "Tuple" {
		return linked(
			vec![
				token("...", CodeTone::Punctuation),
				token(": ", CodeTone::Punctuation),
				token("any", CodeTone::Type),
			],
			&type_href("Tuple"),
			None,
		);
	}
	let ty = if parameter.default.is_none() || parameter.ty.optional {
		parameter.ty.clone()
	} else {
		ApiType {
			optional: true,
			..parameter.ty.clone()
		}
	};
	let mut tokens = vec![
		token(&parameter.name, CodeTone::Parameter),
		token(": ", CodeTone::Punctuation),
	];
	tokens.extend(type_tokens(&ty));
	if let Some(default) = &parameter.default {
		tokens.push(token(" = ", CodeTone::Punctuation));
		tokens.extend(default_value_tokens(&parameter.ty, default));
	}
	tokens
}

pub fn is_nothing(ty: &ApiType) -> bool {
	ty.category == TypeCategory::Primitive && (ty.name == "null" || ty.name == "void")
}

fn owner_tokens(owner: &str, separator: &str) -> Vec<CodeToken> {
	vec![
		linked_token(owner, CodeTone::SelfName, class_href(owner)),
		token(separator, CodeTone::Punctuation),
	]
}

fn named(name: &str, tone: CodeTone, href: Option<&str>) -> CodeToken {
	CodeToken {
		href: href.map(str::to_owned),
		..token(name, tone)
	}
}

pub fn signature_parts(member: &Member, owner: &str, name_href: Option<&str>) -> SignatureParts {
	let parameters = || {
		Some(
			member
				.parameters()
				.iter()
				.map(parameter_tokens)
				.collect::<Vec<_>>(),
		)
	};
	match &member.detail {
		MemberDetail::Property { ty, default, .. } => {
			let mut head = vec![
				named(&member.name, CodeTone::Name, name_href),
				token(": ", CodeTone::Punctuation),
			];
			head.extend(type_tokens(ty));
			let tail = default.as_ref().map_or_else(Vec::new, |default| {
				let mut tail = vec![token(" = ", CodeTone::Punctuation)];
				tail.extend(default_value_tokens(ty, default));
				tail
			});
			SignatureParts {
				owner: owner_tokens(owner, "."),
				head,
				parameters: None,
				tail,
			}
		}
		MemberDetail::Method { returns, .. } => {
			let mut tail = vec![token(")", CodeTone::Punctuation)];
			if !is_nothing(returns) {
				tail.push(token(": ", CodeTone::Punctuation));
				tail.extend(type_tokens(returns));
			}
			SignatureParts {
				owner: owner_tokens(owner, ":"),
				head: vec![
					named(&member.name, CodeTone::Function, name_href),
					token("(", CodeTone::Punctuation),
				],
				parameters: parameters(),
				tail,
			}
		}
		MemberDetail::Event { .. } => SignatureParts {
			owner: owner_tokens(owner, "."),
			head: vec![
				named(&member.name, CodeTone::Name, name_href),
				token(": ", CodeTone::Punctuation),
				linked_token(
					"RBXScriptSignal",
					CodeTone::Type,
					type_href("RBXScriptSignal"),
				),
				token("<(", CodeTone::Punctuation),
			],
			parameters: parameters(),
			tail: vec![token(")>", CodeTone::Punctuation)],
		},
		MemberDetail::Callback { returns, .. } => {
			let mut tail = vec![token(") -> ", CodeTone::Punctuation)];
			tail.extend(type_tokens(returns));
			SignatureParts {
				owner: owner_tokens(owner, "."),
				head: vec![
					named(&member.name, CodeTone::Name, name_href),
					token(": (", CodeTone::Punctuation),
				],
				parameters: parameters(),
				tail,
			}
		}
	}
}

pub fn inline_tokens(parts: &SignatureParts) -> Vec<CodeToken> {
	let mut tokens = parts.owner.clone();
	tokens.extend(parts.head.iter().cloned());
	for (index, parameter) in parts.parameters.iter().flatten().enumerate() {
		if index > 0 {
			tokens.push(token(", ", CodeTone::Punctuation));
		}
		tokens.extend(parameter.iter().cloned());
	}
	tokens.extend(parts.tail.iter().cloned());
	tokens
}

pub fn parts_text(parts: &SignatureParts) -> String {
	super::code::tokens_text(&inline_tokens(parts))
}
