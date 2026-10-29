//! Fills and minifies the HTML template that Vite builds from `index.html`.

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

/// Compresses and mangles a classic script, or returns it unchanged when it
/// does not parse.
pub fn minify_script(source: &str) -> String {
	use oxc_minifier::{CompressOptions, MangleOptions, Minifier, MinifierOptions};

	let allocator = oxc_allocator::Allocator::default();
	let parsed = oxc_parser::Parser::new(&allocator, source, oxc_span::SourceType::cjs()).parse();
	if !parsed.errors.is_empty() {
		return source.trim().to_owned();
	}
	let mut program = parsed.program;
	let minified = Minifier::new(MinifierOptions {
		mangle: Some(MangleOptions::default()),
		compress: Some(CompressOptions::default()),
	})
	.minify(&allocator, &mut program);
	oxc_codegen::Codegen::new()
		.with_options(oxc_codegen::CodegenOptions::minify())
		.with_scoping(minified.scoping)
		.build(&program)
		.code
}

/// Minifies the template's classic inline scripts and compacts its speculation rules.
pub fn prepare_template(template: &str) -> String {
	let mut out = String::with_capacity(template.len());
	let mut rest = template;
	while let Some(start) = rest.find("<script") {
		out.push_str(&rest[..start]);
		let tail = &rest[start..];
		let Some(open_end) = tail.find('>') else {
			break;
		};
		let open = &tail[..=open_end];
		let Some(close) = tail.find("</script>") else {
			break;
		};
		let body = &tail[open_end + 1..close];
		out.push_str(open);
		if open == "<script>" {
			out.push_str(&minify_script(body));
		} else if open == "<script type=\"speculationrules\">" {
			match serde_json::from_str::<serde_json::Value>(body) {
				Ok(rules) => out.push_str(&rules.to_string()),
				Err(_) => out.push_str(body),
			}
		} else {
			out.push_str(body);
		}
		out.push_str("</script>");
		rest = &tail[close + "</script>".len()..];
	}
	out.push_str(rest);
	out
}

pub fn fill_template(cx: &Context, template: &str, page: &RenderedPage, route: &str) -> String {
	let mut html = template.replacen("<!--app-title-->", &escape_html(&page.title), 1);
	html = html.replacen("<!--app-description-->", &escape_html(SITE_DESCRIPTION), 1);
	html = html.replacen("<!--app-head-->", &head_tags(cx, page, route), 1);
	html.replacen("<!--app-body-->", &page.body, 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mangles_block_scoped_names_in_inline_scripts() {
		let minified = minify_script(
			"try {\n\tconst params = new URLSearchParams(location.search)\n\tdocument.title = params.get(\"q\") ?? \"\"\n} catch {}",
		);
		assert!(!minified.contains("params"), "{minified}");
		assert!(
			minified.contains("URLSearchParams(location.search)"),
			"{minified}"
		);
	}

	#[test]
	fn compacts_speculation_rules_and_keeps_other_scripts() {
		let template = "<script type=\"speculationrules\">\n{ \"prerender\": [] }\n</script><script type=\"module\" src=\"/a.js\"></script>";
		assert_eq!(
			prepare_template(template),
			"<script type=\"speculationrules\">{\"prerender\":[]}</script><script type=\"module\" src=\"/a.js\"></script>"
		);
	}
}
