const MINIMUM_FUZZY_SPAN: usize = 16;
const FUZZY_SPAN_PER_CHARACTER: usize = 2;

fn at(name: &[u8], index: usize) -> u8 {
	name.get(index).copied().unwrap_or(0)
}

pub fn is_boundary(name: &[u8], index: usize) -> bool {
	if index == 0 {
		return true;
	}
	let previous = at(name, index - 1);
	let current = at(name, index);
	if previous == b'_' {
		return true;
	}
	if current.is_ascii_uppercase() && !previous.is_ascii_uppercase() {
		return true;
	}
	if current.is_ascii_digit() && !previous.is_ascii_digit() {
		return true;
	}
	current.is_ascii_uppercase()
		&& previous.is_ascii_uppercase()
		&& at(name, index + 1).is_ascii_lowercase()
}

const fn class_bit(byte: u8) -> u64 {
	let class = match byte {
		b'a'..=b'z' => byte - b'a',
		b'A'..=b'Z' => byte - b'A',
		b'0'..=b'9' => byte - b'0' + 26,
		b'_' => 36,
		b'.' => 37,
		b' ' => 38,
		0..=127 => 39 + byte % 16,
		_ => 55 + byte % 8,
	};
	1 << class
}

pub fn character_mask(text: &[u8]) -> u64 {
	text.iter().fold(0, |mask, byte| mask | class_bit(*byte))
}

fn find_from(lower: &str, character: char, from: usize) -> Option<usize> {
	lower
		.get(from..)?
		.find(character)
		.map(|position| position + from)
}

fn subsequence(query: &str, name: &[u8], lower: &str, boundaries: bool) -> Option<Vec<usize>> {
	let mut positions = Vec::with_capacity(query.len());
	let mut from = 0;
	for character in query.chars() {
		let mut found = None;
		if boundaries {
			let mut index = find_from(lower, character, from);
			while let Some(position) = index {
				if is_boundary(name, position) {
					found = Some(position);
					break;
				}
				index = find_from(lower, character, position + 1);
			}
		}
		let position = found.or_else(|| find_from(lower, character, from))?;
		positions.push(position);
		from = position + 1;
	}
	Some(positions)
}

fn fuzzy_positions(query: &str, name: &[u8], lower: &str) -> Option<Vec<usize>> {
	subsequence(query, name, lower, true).or_else(|| subsequence(query, name, lower, false))
}

fn is_too_spread(query: &str, positions: &[usize]) -> bool {
	let span = positions.last().copied().unwrap_or(0) - positions.first().copied().unwrap_or(0) + 1;
	span > MINIMUM_FUZZY_SPAN.max(query.len() * FUZZY_SPAN_PER_CHARACTER)
}

fn small(value: usize) -> i64 {
	i64::try_from(value).unwrap_or(i64::MAX)
}

fn length_difference(lower: &str, query: &str) -> i64 {
	small(lower.len()) - small(query.len())
}

fn fuzzy_score(query: &str, name: &[u8], lower: &str) -> i64 {
	let Some(positions) = fuzzy_positions(query, name, lower) else {
		return 0;
	};
	if is_too_spread(query, &positions) {
		return 0;
	}
	let mut score = 300;
	let mut previous: Option<usize> = None;
	for &position in &positions {
		if is_boundary(name, position) {
			score += 12;
		}
		if let Some(before) = previous
			&& position != before + 1
		{
			score -= small(position - before).min(10) * 3;
		}
		previous = Some(position);
	}
	score -= small(positions.first().copied().unwrap_or(0)).min(20) * 2;
	score -= length_difference(lower, query).min(40);
	score.max(1)
}

pub fn score_name(query: &str, name: &str, lower: &str) -> i64 {
	if query.is_empty() {
		return 1;
	}
	if lower == query {
		return 1000;
	}
	if lower.starts_with(query) {
		return 900 - length_difference(lower, query).min(100);
	}
	let name = name.as_bytes();
	if let Some(index) = lower.find(query)
		&& index > 0
	{
		let base = if is_boundary(name, index) { 700 } else { 500 };
		return base - small(index).min(50) - length_difference(lower, query).min(50);
	}
	fuzzy_score(query, name, lower)
}

pub fn match_positions(query: &str, name: &str) -> Vec<usize> {
	if query.is_empty() {
		return Vec::new();
	}
	let lower = name.to_ascii_lowercase();
	if let Some(index) = lower.find(query) {
		return (index..index + query.len()).collect();
	}
	fuzzy_positions(query, name.as_bytes(), &lower).unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::*;

	fn score(query: &str, name: &str) -> i64 {
		score_name(query, name, &name.to_ascii_lowercase())
	}

	#[test]
	fn ranks_exact_prefix_word_substring_and_fuzzy_matches_in_order() {
		let exact = score("touched", "Touched");
		let prefix = score("touch", "Touched");
		let word = score("touch", "CanTouch");
		let substring = score("ouch", "Touched");
		let fuzzy = score("gtp", "GetTouchingParts");
		assert!(
			exact > prefix && prefix > word && word > substring && substring > fuzzy && fuzzy > 0
		);
	}

	#[test]
	fn rejects_letters_spread_across_a_long_name_and_out_of_order_letters() {
		assert_eq!(
			score("material", "TextScraperClientMessageWithPlayerSignal"),
			0
		);
		assert_eq!(score("xyz", "Touched"), 0);
	}

	#[test]
	fn prefers_word_boundaries_for_acronyms() {
		assert_eq!(match_positions("gtp", "GetTouchingParts"), [0, 3, 11]);
	}

	#[test]
	fn rejects_names_that_lack_a_query_character() {
		let mask = character_mask(b"touched");
		assert_eq!(character_mask(b"tch") & !mask, 0);
		assert_ne!(character_mask(b"tx") & !mask, 0);
	}
}
