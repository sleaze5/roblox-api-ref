use crate::RenderedPage;
use crate::context::Context;
use crate::head::{escape_html, head_tags};
use crate::meta::SITE_DESCRIPTION;

pub fn minify_config() -> minify_html::Cfg {
	minify_html::Cfg {
		minify_css: true,
		remove_bangs: true,
		remove_processing_instructions: true,
		..minify_html::Cfg::default()
	}
}

pub fn minify(html: &str) -> Vec<u8> {
	minify_html::minify(html.as_bytes(), &minify_config())
}

const PLACEHOLDERS: [&str; 4] = [
	"<!--app-title-->",
	"<!--app-description-->",
	"<!--app-head-->",
	"<!--app-body-->",
];

pub fn minify_script(source: &str) -> Result<String, String> {
	use oxc_minifier::{CompressOptions, MangleOptions, Minifier, MinifierOptions};

	let allocator = oxc_allocator::Allocator::default();
	let parsed = oxc_parser::Parser::new(&allocator, source, oxc_span::SourceType::cjs()).parse();
	if let Some(error) = parsed.errors.first() {
		return Err(format!(
			"A script in index.html does not parse: {error}"
		));
	}
	let mut program = parsed.program;
	let minified = Minifier::new(MinifierOptions {
		mangle: Some(MangleOptions::default()),
		compress: Some(CompressOptions::default()),
	})
	.minify(&allocator, &mut program);
	Ok(oxc_codegen::Codegen::new()
		.with_options(oxc_codegen::CodegenOptions::minify())
		.with_scoping(minified.scoping)
		.build(&program)
		.code)
}

fn script_body(open: &str, body: &str) -> Result<String, String> {
	match open {
		"<script>" => minify_script(body),
		"<script type=\"speculationrules\">" => serde_json::from_str::<serde_json::Value>(body)
			.map(|rules| rules.to_string())
			.map_err(|error| format!("The speculation rules in index.html are not JSON: {error}")),
		_ => Ok(body.to_owned()),
	}
}

pub fn prepare_template(template: &str) -> Result<String, String> {
	if let Some(missing) = PLACEHOLDERS
		.iter()
		.find(|placeholder| !template.contains(*placeholder))
	{
		return Err(format!("index.html has no {missing} placeholder."));
	}
	let mut out = String::with_capacity(template.len());
	let mut rest = template;
	while let Some(start) = rest.find("<script") {
		out.push_str(&rest[..start]);
		let tail = &rest[start..];
		let (Some(open_end), Some(close)) = (tail.find('>'), tail.find("</script>")) else {
			return Err("index.html has an unclosed <script> tag.".to_owned());
		};
		let open = &tail[..=open_end];
		out.push_str(open);
		out.push_str(&script_body(open, &tail[open_end + 1..close])?);
		out.push_str("</script>");
		rest = &tail[close + "</script>".len()..];
	}
	out.push_str(rest);
	Ok(out)
}

pub fn fill_template(cx: &Context, template: &str, page: &RenderedPage) -> String {
	let mut html = template.replacen("<!--app-title-->", &escape_html(&page.title), 1);
	html = html.replacen("<!--app-description-->", &escape_html(SITE_DESCRIPTION), 1);
	html = html.replacen("<!--app-head-->", &head_tags(cx, page), 1);
	html.replacen("<!--app-body-->", &page.body, 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mangles_block_scoped_names_in_inline_scripts() {
		let minified = minify_script(
			"try {\n\tconst params = new URLSearchParams(location.search)\n\tdocument.title = params.get(\"q\") ?? \"\"\n} catch {}",
		)
		.expect("the script parses");
		assert!(!minified.contains("params"), "{minified}");
		assert!(
			minified.contains("URLSearchParams(location.search)"),
			"{minified}"
		);
	}

	#[test]
	fn compacts_speculation_rules_and_keeps_other_scripts() {
		let placeholders = PLACEHOLDERS.concat();
		let template = format!(
			"<script type=\"speculationrules\">\n{{ \"prerender\": [] }}\n</script><script type=\"module\" src=\"/a.js\"></script>{placeholders}"
		);
		assert_eq!(
			prepare_template(&template),
			Ok(format!(
				"<script type=\"speculationrules\">{{\"prerender\":[]}}</script><script type=\"module\" src=\"/a.js\"></script>{placeholders}"
			))
		);
	}

	#[test]
	fn rejects_a_template_that_would_render_broken_pages() {
		let placeholders = PLACEHOLDERS.concat();
		assert!(prepare_template("<!--app-title--><!--app-head-->").is_err());
		assert!(prepare_template(&format!("<script>let = </script>{placeholders}")).is_err());
		assert!(
			prepare_template(&format!(
				"<script type=\"speculationrules\">{{</script>{placeholders}"
			))
			.is_err()
		);
	}
}
