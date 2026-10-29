use std::cmp::Ordering;
use std::sync::LazyLock;

use icu_collator::preferences::CollationNumericOrdering;
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};

static COLLATOR: LazyLock<CollatorBorrowed<'static>> = LazyLock::new(|| {
	let mut preferences = CollatorPreferences::default();
	preferences.numeric_ordering = Some(CollationNumericOrdering::True);
	Collator::try_new(preferences, Default::default()).expect("the baked collation data loads")
});

pub fn compare_names(a: &str, b: &str) -> Ordering {
	COLLATOR.compare(a, b)
}

pub fn compare_code_units(a: &str, b: &str) -> Ordering {
	if a.is_ascii() && b.is_ascii() {
		return a.cmp(b);
	}
	a.encode_utf16().cmp(b.encode_utf16())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sorts_names_like_intl_collator_with_numeric_ordering() {
		let mut names = vec!["Vector3int16", "vector2", "Vector2", "a10", "a2", "_Hidden", "Zed", "ab"];
		names.sort_by(|a, b| compare_names(a, b));
		assert_eq!(
			names,
			["_Hidden", "a2", "a10", "ab", "vector2", "Vector2", "Vector3int16", "Zed"]
		);
	}

	#[test]
	fn sorts_code_units_like_array_sort() {
		let mut names = vec!["b", "B", "a", "\u{1F600}", "\u{FF5E}"];
		names.sort_by(|a, b| compare_code_units(a, b));
		assert_eq!(names, ["B", "a", "b", "\u{1F600}", "\u{FF5E}"]);
	}
}
