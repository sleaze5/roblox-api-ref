use rapi_search::is_js_space;

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
