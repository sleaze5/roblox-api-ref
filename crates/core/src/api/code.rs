#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CodeTone {
	SelfName,
	Type,
	Constant,
	String,
	Function,
	Name,
	Parameter,
	Punctuation,
	Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeToken {
	pub text: String,
	pub tone: CodeTone,
	pub href: Option<String>,
	pub title: Option<String>,
	pub swatch: Option<String>,
}

pub fn token(text: impl Into<String>, tone: CodeTone) -> CodeToken {
	CodeToken {
		text: text.into(),
		tone,
		href: None,
		title: None,
		swatch: None,
	}
}

pub fn linked_token(text: impl Into<String>, tone: CodeTone, href: String) -> CodeToken {
	CodeToken {
		href: Some(href),
		..token(text, tone)
	}
}

pub fn linked(tokens: Vec<CodeToken>, href: &str, title: Option<&str>) -> Vec<CodeToken> {
	tokens
		.into_iter()
		.map(|part| CodeToken {
			href: Some(href.to_owned()),
			title: title.map_or(part.title, |title| Some(title.to_owned())),
			..part
		})
		.collect()
}

pub fn text_length(tokens: &[CodeToken]) -> usize {
	tokens
		.iter()
		.map(|part| part.text.encode_utf16().count())
		.sum()
}

pub fn unlinked(tokens: &[CodeToken]) -> Vec<CodeToken> {
	tokens
		.iter()
		.map(|part| CodeToken {
			href: None,
			..part.clone()
		})
		.collect()
}

pub fn tokens_text(tokens: &[CodeToken]) -> String {
	tokens.iter().map(|part| part.text.as_str()).collect()
}
