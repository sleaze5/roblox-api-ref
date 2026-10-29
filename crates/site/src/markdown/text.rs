use rapi_core::js::is_js_space;

pub fn escape_text(text: &str) -> String {
	let mut escaped = String::with_capacity(text.len());
	for character in text.chars() {
		if matches!(character, '`' | '*' | '_' | '[' | ']' | '<') {
			escaped.push('\\');
		}
		escaped.push(character);
	}
	escaped
}

pub fn inline_code(text: &str) -> String {
	let longest = text
		.split(|character| character != '`')
		.map(str::len)
		.max()
		.unwrap_or(0);
	let fence = "`".repeat(longest + 1);
	if text.starts_with('`') || text.ends_with('`') {
		format!("{fence} {text} {fence}")
	} else {
		format!("{fence}{text}{fence}")
	}
}

pub fn link(text: &str, url: &str) -> String {
	format!("[{text}]({})", url.replace(' ', "%20"))
}

pub fn join_lines(text: &str) -> String {
	let mut joined = String::with_capacity(text.len());
	let mut chars = text.chars().peekable();
	while let Some(character) = chars.next() {
		if is_js_space(character) {
			let mut run = String::from(character);
			while let Some(next) = chars.peek().copied().filter(|next| is_js_space(*next)) {
				run.push(next);
				chars.next();
			}
			if run.contains('\n') {
				joined.push(' ');
			} else {
				joined.push_str(&run);
			}
		} else {
			joined.push(character);
		}
	}
	joined
}

pub fn table_cell(markdown: &str) -> String {
	join_lines(markdown).replace('|', "\\|")
}

pub fn fact_list(facts: &[(&str, Option<String>)]) -> String {
	facts
		.iter()
		.filter_map(|(label, value)| {
			value
				.as_deref()
				.filter(|value| !value.is_empty())
				.map(|value| format!("- {label}: {value}"))
		})
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn join_blocks(blocks: impl IntoIterator<Item = Option<String>>) -> String {
	let blocks: Vec<String> = blocks
		.into_iter()
		.flatten()
		.filter(|block| !block.is_empty())
		.collect();
	format!("{}\n", blocks.join("\n\n"))
}

pub fn markdown_table(headings: &[&str], rows: &[Vec<String>]) -> String {
	let mut lines = vec![
		format!("| {} |", headings.join(" | ")),
		format!(
			"| {} |",
			headings
				.iter()
				.map(|_| "---")
				.collect::<Vec<_>>()
				.join(" | ")
		),
	];
	lines.extend(rows.iter().map(|row| {
		format!(
			"| {} |",
			row.iter()
				.map(|cell| table_cell(cell))
				.collect::<Vec<_>>()
				.join(" | ")
		)
	}));
	lines.join("\n")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn escapes_markdown_syntax_in_text() {
		assert_eq!(
			escape_text("Get_Name *x* [a] <b> `c`"),
			"Get\\_Name \\*x\\* \\[a\\] \\<b> \\`c\\`"
		);
	}

	#[test]
	fn keeps_a_table_cell_on_one_line_and_escapes_pipes() {
		assert_eq!(table_cell("a  \n\n  b | c"), "a b \\| c");
		assert_eq!(table_cell("a  b"), "a  b");
	}

	#[test]
	fn fences_code_that_has_backticks() {
		assert_eq!(inline_code("a`b"), "``a`b``");
		assert_eq!(inline_code("`"), "`` ` ``");
		assert_eq!(inline_code("a``b"), "```a``b```");
		assert_eq!(link("A B", "https://x/a b"), "[A B](https://x/a%20b)");
	}
}
