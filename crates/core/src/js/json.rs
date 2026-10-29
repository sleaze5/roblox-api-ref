use std::fmt::Write;

pub fn json_string(text: &str) -> String {
	let mut json = String::with_capacity(text.len() + 2);
	json.push('"');
	for character in text.chars() {
		match character {
			'"' => json.push_str("\\\""),
			'\\' => json.push_str("\\\\"),
			'\n' => json.push_str("\\n"),
			'\r' => json.push_str("\\r"),
			'\t' => json.push_str("\\t"),
			'\u{8}' => json.push_str("\\b"),
			'\u{c}' => json.push_str("\\f"),
			control if u32::from(control) < 0x20 => {
				let _ = write!(json, "\\u{:04x}", u32::from(control));
			}
			other => json.push(other),
		}
	}
	json.push('"');
	json
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn quotes_like_json_stringify() {
		assert_eq!(json_string("Part"), "\"Part\"");
		assert_eq!(json_string("a\"b\\c\n\u{1}é"), "\"a\\\"b\\\\c\\n\\u0001é\"");
	}
}
