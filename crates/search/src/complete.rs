use crate::engine::{Engine, SearchResult};
use crate::query::{ACCESS_FILTERS, KIND_ALIASES, TAG_ALIASES};

fn find_option<'a>(options: impl Iterator<Item = &'a str>, value: &str) -> Option<&'a str> {
	options.into_iter().find(|candidate| {
		let lower = candidate.to_lowercase();
		lower.starts_with(value) && lower != value
	})
}

fn complete_filter(word: &str, engine: &Engine) -> Option<String> {
	let negation = if word.starts_with('!') || word.starts_with('-') {
		&word[..1]
	} else {
		""
	};
	let body = word[negation.len()..].to_lowercase();
	for prefix in ["is:", "tag:", "access:", "type:", "uses:"] {
		if let Some(value) = body.strip_prefix(prefix) {
			let option = match prefix {
				"is:" => find_option(KIND_ALIASES.iter().map(|(name, _)| *name), value),
				"tag:" => find_option(TAG_ALIASES.iter().map(|(name, _)| *name), value),
				"access:" => find_option(ACCESS_FILTERS.iter().map(|(name, _)| *name), value),
				_ => find_option(engine.type_names(), value),
			};
			return option.map(|option| format!("{negation}{prefix}{option}"));
		}
		if prefix.starts_with(&body) && !body.is_empty() {
			return Some(format!("{negation}{prefix}"));
		}
	}
	None
}

fn complete_name(word: &str, engine: &Engine, top: Option<&SearchResult>) -> Option<String> {
	let top = top?;
	let name = engine.name(top.entry);
	let Some(dot) = word.find(['.', ':']) else {
		if !engine.is_primary(top.entry) || !name.to_lowercase().starts_with(&word.to_lowercase()) {
			return None;
		}
		return Some(format!("{name}."));
	};
	let scope = &word[..dot];
	let separator = &word[dot..=dot];
	let member = &word[dot + 1..];
	if engine.is_primary(top.entry) || !name.to_lowercase().starts_with(&member.to_lowercase()) {
		return None;
	}
	let scope_name = top.scope.map_or(scope, |scope| engine.name(scope));
	Some(format!("{scope_name}{separator}{name}"))
}

pub fn complete(input: &str, engine: &Engine, top: Option<&SearchResult>) -> Option<String> {
	let start = input.rfind(' ').map_or(0, |space| space + 1);
	let word = &input[start..];
	if word.is_empty() {
		return None;
	}
	let completed = complete_filter(word, engine).or_else(|| complete_name(word, engine, top))?;
	(completed != word).then(|| format!("{}{completed}", &input[..start]))
}
