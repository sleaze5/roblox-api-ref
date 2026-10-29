use crate::format::{EntryKind, flags};

pub const KIND_ALIASES: [(&str, &[EntryKind]); 10] = [
	("class", &[EntryKind::Class]),
	("enum", &[EntryKind::Enum]),
	("type", &[EntryKind::DataType]),
	("property", &[EntryKind::Property]),
	("method", &[EntryKind::Method]),
	("function", &[EntryKind::Method]),
	("event", &[EntryKind::Event]),
	("callback", &[EntryKind::Callback]),
	("item", &[EntryKind::EnumItem]),
	(
		"member",
		&[
			EntryKind::Property,
			EntryKind::Method,
			EntryKind::Event,
			EntryKind::Callback,
		],
	),
];

pub const TAG_ALIASES: [(&str, u16); 9] = [
	("deprecated", flags::DEPRECATED),
	("hidden", flags::HIDDEN),
	("internal", flags::INTERNAL),
	("service", flags::SERVICE),
	("readonly", flags::READ_ONLY),
	("yields", flags::YIELDS),
	("notcreatable", flags::NOT_CREATABLE),
	("notreplicated", flags::NOT_REPLICATED),
	("notscriptable", flags::NOT_SCRIPTABLE),
];

pub const ACCESS_FILTERS: [(&str, u8); 3] = [("game", 0), ("plugin", 1), ("command", 2)];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedQuery {
	pub words: Vec<String>,
	pub scope: Option<String>,
	pub exact: bool,
	pub kinds: Vec<EntryKind>,
	pub include: u16,
	pub exclude: u16,
	pub value_type: Option<String>,
	pub uses: Option<String>,
	pub enum_scope: bool,
	pub access: Option<u8>,
}

impl ParsedQuery {
	pub fn has_kind(&self, kind: EntryKind) -> bool {
		self.kinds.is_empty() || self.kinds.contains(&kind)
	}

	pub const fn has_filters(&self) -> bool {
		!self.kinds.is_empty()
			|| self.include != 0
			|| self.value_type.is_some()
			|| self.uses.is_some()
			|| self.access.is_some()
	}

	fn add_kind(&mut self, kind: EntryKind) {
		if !self.kinds.contains(&kind) {
			self.kinds.push(kind);
		}
	}
}

pub const fn is_js_space(character: char) -> bool {
	matches!(
		character,
		'\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
			..='\u{200a}'
				| '\u{2028}' | '\u{2029}'
				| '\u{202f}' | '\u{205f}'
				| '\u{3000}' | '\u{feff}'
	)
}

fn apply_filter(query: &mut ParsedQuery, negated: bool, field: &str, value: &str) -> bool {
	let lower = value.to_lowercase();
	match field {
		"type" | "uses" => {
			if value.is_empty() {
				return false;
			}
			if field == "type" {
				query.value_type = Some(value.to_owned());
			} else {
				query.uses = Some(value.to_owned());
			}
			true
		}
		"access" => ACCESS_FILTERS
			.iter()
			.find(|(name, _)| *name == lower)
			.map(|(_, level)| query.access = Some(*level))
			.is_some(),
		"is" => KIND_ALIASES
			.iter()
			.find(|(name, _)| *name == lower)
			.map(|(_, kinds)| {
				for kind in *kinds {
					query.add_kind(*kind);
				}
			})
			.is_some(),
		_ => TAG_ALIASES
			.iter()
			.find(|(name, _)| *name == lower)
			.map(|(_, flag)| {
				if negated {
					query.exclude |= flag;
				} else {
					query.include |= flag;
				}
			})
			.is_some(),
	}
}

fn parse_filter(part: &str) -> Option<(bool, String, &str)> {
	let (negated, rest) = match part.as_bytes().first() {
		Some(b'!' | b'-') => (true, &part[1..]),
		_ => (false, part),
	};
	let colon = rest.find(':')?;
	let field = rest[..colon].to_ascii_lowercase();
	if !matches!(field.as_str(), "is" | "tag" | "type" | "uses" | "access")
		|| !rest[..colon].is_ascii()
		|| rest[colon + 1..].contains(['\n', '\r', '\u{2028}', '\u{2029}'])
	{
		return None;
	}
	Some((negated, field, &rest[colon + 1..]))
}

fn add_enum_word(query: &mut ParsedQuery, word: &str) {
	let Some(dot) = word.find('.') else {
		query.add_kind(EntryKind::Enum);
		if !word.is_empty() {
			query.words.push(word.to_owned());
		}
		return;
	};
	query.scope = Some(word[..dot].to_owned());
	query.enum_scope = true;
	let item = &word[dot + 1..];
	if !item.is_empty() {
		query.words.push(item.to_owned());
	}
}

fn add_word(query: &mut ParsedQuery, part: &str) {
	let mut word = part;
	if word.encode_utf16().count() >= 2 && word.starts_with('"') && word.ends_with('"') {
		query.exact = true;
		word = &word[1..word.len() - 1];
	}
	if query.scope.is_none() && word.to_lowercase().starts_with("enum.") {
		add_enum_word(query, &word["enum.".len()..]);
		return;
	}
	if query.scope.is_none()
		&& let Some(dot) = word.find(['.', ':'])
	{
		query.scope = Some(word[..dot].to_owned());
		word = &word[dot + 1..];
	}
	if !word.is_empty() {
		query.words.push(word.to_owned());
	}
}

pub fn parse_query(input: &str) -> ParsedQuery {
	let mut query = ParsedQuery::default();
	for part in input.split(is_js_space).filter(|part| !part.is_empty()) {
		if let Some((negated, field, value)) = parse_filter(part)
			&& apply_filter(&mut query, negated, &field, value)
		{
			continue;
		}
		add_word(&mut query, part);
	}
	query
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn splits_a_class_scope_from_the_member_word() {
		let query = parse_query("Part.Touch");
		assert_eq!(query.scope.as_deref(), Some("Part"));
		assert_eq!(query.words, ["Touch"]);
	}

	#[test]
	fn reads_kind_tag_type_and_access_filters() {
		let query = parse_query("is:method tag:yields !tag:deprecated type:Color3 access:game");
		assert_eq!(query.kinds, [EntryKind::Method]);
		assert_eq!(query.include, flags::YIELDS);
		assert_eq!(query.exclude, flags::DEPRECATED);
		assert_eq!(query.value_type.as_deref(), Some("Color3"));
		assert_eq!(query.access, Some(0));
	}

	#[test]
	fn reads_an_unknown_filter_as_a_class_member_search() {
		let query = parse_query("tag:nothing");
		assert_eq!(query.scope.as_deref(), Some("tag"));
		assert_eq!(query.words, ["nothing"]);
	}

	#[test]
	fn marks_quoted_words_as_exact() {
		let query = parse_query("\"Name\"");
		assert!(query.exact);
		assert_eq!(query.words, ["Name"]);
	}

	#[test]
	fn reads_uses_and_enum_scopes() {
		assert_eq!(
			parse_query("uses:Enum.Material").uses.as_deref(),
			Some("Enum.Material")
		);
		let query = parse_query("Enum.Mat");
		assert_eq!(query.kinds, [EntryKind::Enum]);
		assert_eq!(query.words, ["Mat"]);
		let query = parse_query("Enum.Material.Plas");
		assert_eq!(query.scope.as_deref(), Some("Material"));
		assert!(query.enum_scope);
		assert_eq!(query.words, ["Plas"]);
	}
}
