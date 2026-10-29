use crate::score::{character_mask, score_name};

pub struct Matcher {
	names: Vec<(String, String, u64)>,
}

impl Matcher {
	pub fn new(names: &str) -> Self {
		Self {
			names: names
				.split('\n')
				.map(|name| {
					let lower = name.to_ascii_lowercase();
					let mask = character_mask(lower.as_bytes());
					(name.to_owned(), lower, mask)
				})
				.collect(),
		}
	}

	pub fn matches(&self, query: &str) -> Vec<u8> {
		let mask = character_mask(query.as_bytes());
		self.names
			.iter()
			.map(|(name, lower, name_mask)| {
				u8::from(
					query.is_empty()
						|| (mask & !name_mask == 0 && score_name(query, name, lower) > 0),
				)
			})
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn matches_each_name_like_the_list_filters() {
		let matcher = Matcher::new("Part\nGetTouchingParts\nModel");
		assert_eq!(matcher.matches("gtp"), [0, 1, 0]);
		assert_eq!(matcher.matches("part"), [1, 1, 0]);
		assert_eq!(matcher.matches(""), [1, 1, 1]);
	}
}
