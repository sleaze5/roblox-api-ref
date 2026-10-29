use crate::engine::Engine;
use crate::format::{Index, RemovedData};
use crate::query::{ParsedQuery, is_js_space};
use crate::score::score_name;

fn is_hash(text: &str) -> bool {
	(6..=16).contains(&text.len())
		&& text
			.bytes()
			.all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_version(text: &str) -> bool {
	let mut parts = text.split('.');
	let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
	let (Some(major), Some(minor)) = (parts.next(), parts.next()) else {
		return false;
	};
	digits(major)
		&& digits(minor)
		&& parts.all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
}

pub fn match_updates(index: &Index, query: &str, limit: usize) -> Vec<usize> {
	let lower = query.trim_matches(is_js_space).to_lowercase();
	let text = lower.strip_prefix("version-").unwrap_or(&lower);
	let matches: Box<dyn Fn(usize) -> bool> = if is_hash(text) {
		Box::new(|position| index.update_hash(position).starts_with(text))
	} else if is_version(text) {
		Box::new(|position| index.update_version(position).starts_with(text))
	} else {
		return Vec::new();
	};
	(0..index.update_count())
		.filter(|position| matches(*position))
		.take(limit)
		.collect()
}

fn score_removed(entry: &RemovedData, name: &str, parent: &str, query: &ParsedQuery) -> i64 {
	if !query.kinds.is_empty() && !query.kinds.contains(&entry.kind) {
		return 0;
	}
	if let Some(scope) = &query.scope {
		let same_parent = entry.parent.is_some() && parent == scope.to_lowercase();
		if !same_parent || (query.enum_scope && entry.kind != crate::EntryKind::EnumItem) {
			return 0;
		}
	}
	let Some(word) = query.words.first() else {
		return i64::from(query.scope.is_some());
	};
	let word = word.to_lowercase();
	if query.exact {
		return i64::from(name == word);
	}
	score_name(&word, &entry.name, name)
}

pub fn match_removed(engine: &Engine, query: &ParsedQuery, limit: usize) -> Vec<RemovedData> {
	let mut scored: Vec<(&RemovedData, i64)> = engine
		.removed()
		.iter()
		.map(|(entry, name, parent)| (entry, score_removed(entry, name, parent, query)))
		.filter(|(_, score)| *score > 0)
		.collect();
	scored.sort_by(|a, b| {
		b.1.cmp(&a.1)
			.then_with(|| b.0.published.cmp(&a.0.published))
	});
	scored
		.into_iter()
		.take(limit)
		.map(|(entry, _)| entry.clone())
		.collect()
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::format::{EntryKind, IndexData, UpdateData};
	use crate::query::parse_query;

	fn index() -> Index {
		Index::new(&index_data().encode()).unwrap()
	}

	fn index_data() -> IndexData {
		let update = |version: &str, hash: &str| UpdateData {
			version: version.into(),
			hash: hash.into(),
			published: version.into(),
			date: String::new(),
		};
		let removed = |kind, name: &str, parent: Option<&str>, version: &str| RemovedData {
			kind,
			name: name.into(),
			parent: parent.map(str::to_owned),
			version: version.into(),
			published: version.into(),
		};
		IndexData {
			updates: vec![
				update("0.745.0.7451137", "7062aec9dbe74b14"),
				update("0.744.0.7441077", "7aaa6838d36c4448"),
				update("0.74.0.740001", "0123456789abcdef"),
			],
			removed: vec![
				removed(
					EntryKind::Class,
					"BulkImportService",
					None,
					"0.745.0.7451137",
				),
				removed(
					EntryKind::Method,
					"isA",
					Some("Instance"),
					"0.744.0.7441077",
				),
				removed(
					EntryKind::EnumItem,
					"Old",
					Some("Material"),
					"0.74.0.740001",
				),
			],
			..IndexData::default()
		}
	}

	fn versions(query: &str) -> Vec<String> {
		let index = index();
		match_updates(&index, query, 8)
			.into_iter()
			.map(|position| index.update_version(position).to_owned())
			.collect()
	}

	fn removed(query: &str) -> Vec<String> {
		let engine = Engine::new(&index_data().encode()).unwrap();
		match_removed(&engine, &parse_query(query), 8)
			.into_iter()
			.map(|entry| entry.name)
			.collect()
	}

	#[test]
	fn finds_an_update_by_its_hash_with_or_without_the_version_prefix() {
		assert_eq!(versions("7062aec9dbe74b14"), ["0.745.0.7451137"]);
		assert_eq!(versions(" version-7AAA6838D36C4448 "), ["0.744.0.7441077"]);
		assert_eq!(versions("7aaa68"), ["0.744.0.7441077"]);
	}

	#[test]
	fn finds_updates_by_the_start_of_their_version() {
		assert_eq!(versions("0.745"), ["0.745.0.7451137"]);
		assert_eq!(versions("0.74").len(), 3);
		assert_eq!(versions("0.74."), ["0.74.0.740001"]);
	}

	#[test]
	fn ignores_short_hashes_and_ordinary_names() {
		assert!(versions("7062").is_empty());
		assert!(versions("Part").is_empty());
	}

	#[test]
	fn finds_removed_classes_members_and_enum_items_by_name() {
		assert_eq!(removed("BulkImport"), ["BulkImportService"]);
		assert_eq!(removed("Instance:isA"), ["isA"]);
		assert_eq!(removed("Enum.Material.Old"), ["Old"]);
		assert_eq!(removed("Instance."), ["isA"]);
		assert!(removed("is:class isA").is_empty());
	}
}
