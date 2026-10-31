use crate::model::SampleToken;

pub fn split_lines(tokens: Vec<SampleToken>) -> Vec<Vec<SampleToken>> {
	let mut lines = vec![Vec::new()];
	for token in tokens {
		for (index, part) in token.text.split('\n').enumerate() {
			if index > 0 {
				lines.push(Vec::new());
			}
			if !part.is_empty()
				&& let Some(line) = lines.last_mut()
			{
				line.push(SampleToken {
					text: part.to_owned(),
					tone: token.tone,
				});
			}
		}
	}
	lines
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::code::CodeTone;

	fn token(text: &str, tone: CodeTone) -> SampleToken {
		SampleToken {
			text: text.to_owned(),
			tone,
		}
	}

	#[test]
	fn splits_tokens_at_line_breaks_and_keeps_blank_lines() {
		assert_eq!(
			split_lines(vec![
				token("--[[a\nb]]", CodeTone::Comment),
				token("\n\n", CodeTone::Plain),
				token("end", CodeTone::Keyword),
			]),
			vec![
				vec![token("--[[a", CodeTone::Comment)],
				vec![token("b]]", CodeTone::Comment)],
				Vec::new(),
				vec![token("end", CodeTone::Keyword)],
			]
		);
	}
}
