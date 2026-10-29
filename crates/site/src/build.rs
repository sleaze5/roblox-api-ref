//! Writes the prerendered site into Vite's output directory.

use std::path::{Path, PathBuf};

use rapi_routes::decode_uri_component;
use rayon::prelude::*;

use crate::context::Context;
use crate::head::escape_html;
use crate::meta::SITE_DESCRIPTION;
use crate::render_page;
use crate::routes::route_paths;
use crate::template::{fill_template, minify, prepare_template};

/// The source path of the icon sprite, as Vite's manifest names it.
pub const SPRITE_SOURCE: &str = "src/generated/icons.svg";

/// The URL of the icon sprite on the Vite dev server.
pub const DEV_SPRITE: &str = "/src/generated/icons.svg";

/// The public URL of the icon sprite in a Vite manifest.
pub fn sprite_url(manifest: &serde_json::Value) -> Option<String> {
	let file = manifest.get(SPRITE_SOURCE)?.get("file")?.as_str()?;
	Some(format!("/{file}"))
}

fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<(), String> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent)
			.map_err(|error| format!("{}: {error}", parent.display()))?;
	}
	std::fs::write(path, contents).map_err(|error| format!("{}: {error}", path.display()))
}

fn output_file(out_dir: &Path, route: &str) -> PathBuf {
	if route == "/404" {
		return out_dir.join("404.html");
	}
	let decoded = decode_uri_component(route).unwrap_or_else(|| route.to_owned());
	out_dir
		.join(decoded.trim_start_matches('/'))
		.join("index.html")
}

fn markdown_file(out_dir: &Path, href: &str) -> PathBuf {
	let decoded = decode_uri_component(href).unwrap_or_else(|| href.to_owned());
	out_dir.join(decoded.trim_start_matches('/'))
}

pub fn robots_txt(cx: &Context) -> String {
	format!(
		"User-agent: *\nAllow: /\n\nSitemap: {}\n",
		cx.absolute_url("/sitemap.xml")
	)
}

pub fn sitemap_xml(cx: &Context, routes: &[String]) -> String {
	let mut xml = String::from(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">",
	);
	for route in routes.iter().filter(|route| *route != "/404") {
		xml.push_str("<url><loc>");
		xml.push_str(&escape_html(&cx.absolute_url(route)));
		xml.push_str("</loc><lastmod>");
		xml.push_str(&cx.api.published);
		xml.push_str("</lastmod></url>");
	}
	xml.push_str("</urlset>\n");
	xml
}

pub fn opensearch_xml(cx: &Context) -> String {
	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?><OpenSearchDescription xmlns=\"http://a9.com/-/spec/opensearch/1.1/\"><ShortName>Roblox API</ShortName><Description>{SITE_DESCRIPTION}</Description><InputEncoding>UTF-8</InputEncoding><Url type=\"text/html\" template=\"{}?search={{searchTerms}}\"/></OpenSearchDescription>\n",
		escape_html(&cx.absolute_url("/"))
	)
}

/// Renders every route into `out_dir`, replacing the template at `out_dir/index.html`.
pub fn write_site(
	cx: &Context,
	out_dir: &Path,
	routes: Option<&[String]>,
) -> Result<usize, String> {
	let template_file = out_dir.join("index.html");
	let template = std::fs::read_to_string(&template_file)
		.map_err(|error| format!("{}: {error}", template_file.display()))?;
	let template = prepare_template(&template);
	let all_routes = route_paths(cx);
	let routes = routes.unwrap_or(&all_routes);
	write(&out_dir.join("robots.txt"), robots_txt(cx))?;
	write(&out_dir.join("sitemap.xml"), sitemap_xml(cx, &all_routes))?;
	write(&out_dir.join("opensearch.xml"), opensearch_xml(cx))?;
	write(&out_dir.join("updates.xml"), crate::feed::updates_feed(cx))?;
	routes.par_iter().try_for_each(|route| {
		let page = render_page(cx, route);
		write(
			&markdown_file(out_dir, &page.markdown.href),
			&page.markdown.content,
		)?;
		let html = fill_template(cx, &template, &page, route);
		write(&output_file(out_dir, route), minify(&html))
	})?;
	Ok(routes.len())
}
