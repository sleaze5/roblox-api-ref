pub const JS_SPACE: &str = r"\t\n\x0B\x0C\r \u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}\u{feff}";

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

pub fn js_trim(text: &str) -> &str {
	text.trim_matches(is_js_space)
}

pub fn js_trim_start(text: &str) -> &str {
	text.trim_start_matches(is_js_space)
}

pub fn js_trim_end(text: &str) -> &str {
	text.trim_end_matches(is_js_space)
}

pub fn collapse_spaces(text: &str) -> String {
	let mut collapsed = String::with_capacity(text.len());
	let mut in_space = false;
	for character in text.chars() {
		if is_js_space(character) {
			if !in_space {
				collapsed.push(' ');
			}
			in_space = true;
		} else {
			collapsed.push(character);
			in_space = false;
		}
	}
	collapsed
}

pub fn upper_first(text: &str) -> String {
	let mut characters = text.chars();
	characters.next().map_or_else(String::new, |first| {
		first.to_uppercase().chain(characters).collect()
	})
}
