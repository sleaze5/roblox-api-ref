use stylua_lib::{
	BlockNewlineGaps, CallParenType, CollapseSimpleStatement, Config, IndentType, LineEndings,
	LuaVersion, OutputVerification, QuoteStyle, SortRequiresConfig, SpaceAfterFunctionNames,
	format_code,
};

const COLUMN_WIDTH: usize = 120;

fn config() -> Config {
	Config {
		block_newline_gaps: BlockNewlineGaps::Never,
		call_parentheses: CallParenType::Always,
		collapse_simple_statement: CollapseSimpleStatement::Never,
		column_width: COLUMN_WIDTH,
		indent_type: IndentType::Tabs,
		indent_width: 4,
		line_endings: LineEndings::Unix,
		quote_style: QuoteStyle::ForceDouble,
		sort_requires: SortRequiresConfig { enabled: false },
		space_after_function_names: SpaceAfterFunctionNames::Never,
		syntax: LuaVersion::All,
		..Config::default()
	}
}

fn limit_blank_lines(code: &str) -> String {
	let mut text = String::with_capacity(code.len());
	let mut newlines = 0;
	for character in code.chars() {
		if character == '\n' {
			newlines += 1;
			if newlines > 2 {
				continue;
			}
		} else {
			newlines = 0;
		}
		text.push(character);
	}
	text
}

fn tidy(code: &str) -> String {
	let unix = code.replace("\r\n", "\n").replace('\r', "\n");
	let lines: Vec<&str> = unix.lines().map(str::trim_end).collect();
	limit_blank_lines(&lines.join("\n")).trim().to_owned()
}

pub fn format_sample(code: &str) -> Option<String> {
	let source = tidy(code);
	if source.is_empty() {
		return None;
	}
	Some(
		format_code(&source, config(), None, OutputVerification::None)
			.map_or(source, |formatted| tidy(&formatted)),
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formats_with_tabs_double_quotes_and_call_parentheses() {
		assert_eq!(
			format_sample("local   x = require 'Module'\nif x then   return   end").as_deref(),
			Some("local x = require(\"Module\")\nif x then\n\treturn\nend")
		);
	}

	#[test]
	fn removes_blank_lines_at_block_boundaries() {
		assert_eq!(
			format_sample("if ready then\n\n\twork()\n\nend").as_deref(),
			Some("if ready then\n\twork()\nend")
		);
	}

	#[test]
	fn keeps_at_most_one_blank_line_and_trims_the_edges() {
		assert_eq!(
			format_sample("\n\nlocal a = 1\n\n\n\n-- note\nlocal b = 2\n\n").as_deref(),
			Some("local a = 1\n\n-- note\nlocal b = 2")
		);
	}

	#[test]
	fn keeps_code_that_does_not_parse() {
		assert_eq!(
			format_sample("local = = 1\n\n\n\nfoo(").as_deref(),
			Some("local = = 1\n\nfoo(")
		);
	}

	#[test]
	fn skips_empty_samples() {
		assert_eq!(format_sample(" \n\t\n"), None);
	}
}
