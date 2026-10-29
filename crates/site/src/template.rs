//! Fills and minifies the HTML template that Vite builds from `index.html`.

use crate::RenderedPage;
use crate::context::Context;
use crate::head::{escape_html, head_tags};
use crate::meta::SITE_DESCRIPTION;

pub fn minify_config(minify_js: bool) -> minify_html::Cfg {
	minify_html::Cfg {
		minify_css: true,
		minify_js,
		remove_bangs: true,
		remove_processing_instructions: true,
		..minify_html::Cfg::default()
	}
}

pub fn minify(html: &str) -> Vec<u8> {
	minify_html::minify(html.as_bytes(), &minify_config(false))
}

fn minify_script(source: &str) -> String {
	let wrapped = format!("<script>{source}</script>");
	let minified = String::from_utf8(minify_html::minify(
		wrapped.as_bytes(),
		&minify_config(true),
	))
	.unwrap_or(wrapped);
	minified
		.strip_prefix("<script>")
		.and_then(|rest| rest.strip_suffix("</script>"))
		.map_or_else(|| source.to_owned(), str::to_owned)
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
