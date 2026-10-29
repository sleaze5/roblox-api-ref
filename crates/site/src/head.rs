//! The tags that each page adds to the template's `<head>`.

use std::fmt::Write as _;

use serde_json::{Value, json};

use crate::RenderedPage;
use crate::context::Context;
use crate::meta::{REPOSITORY_URL, SITE_DESCRIPTION, SITE_NAME, SchemaType};

pub fn escape_html(value: &str) -> String {
	let mut escaped = String::with_capacity(value.len());
	for character in value.chars() {
		match character {
			'&' => escaped.push_str("&amp;"),
			'<' => escaped.push_str("&lt;"),
			'>' => escaped.push_str("&gt;"),
			'"' => escaped.push_str("&quot;"),
			_ => escaped.push(character),
		}
	}
	escaped
}

fn meta(out: &mut String, attribute: &str, key: &str, content: &str) {
	let _ = write!(
		out,
		"<meta {attribute}=\"{key}\" content=\"{}\">",
		escape_html(content)
	);
}

fn link(out: &mut String, attributes: &[(&str, &str)]) {
	out.push_str("<link");
	for (key, value) in attributes {
		let _ = write!(out, " {key}=\"{}\"", escape_html(value));
	}
	out.push('>');
}

fn organization_id(cx: &Context) -> String {
	cx.absolute_url("/#organization")
}

fn organization(cx: &Context) -> Value {
	json!({
		"@type": "Organization",
		"@id": organization_id(cx),
		"name": SITE_NAME,
		"url": cx.absolute_url("/"),
		"logo": cx.absolute_url("/og-image.png"),
		"description": SITE_DESCRIPTION,
		"sameAs": [REPOSITORY_URL],
		"contactPoint": {
			"@type": "ContactPoint",
			"contactType": "customer support",
			"url": cx.absolute_url("/contact/"),
			"availableLanguage": "en",
		},
	})
}

fn website(cx: &Context) -> Value {
	let home = cx.absolute_url("/");
	json!({
		"@type": "WebSite",
		"@id": format!("{home}#website"),
		"name": SITE_NAME,
		"alternateName": "Roblox Engine API Reference",
		"description": SITE_DESCRIPTION,
		"url": home,
		"inLanguage": "en",
		"publisher": { "@id": organization_id(cx) },
		"potentialAction": {
			"@type": "SearchAction",
			"target": {
				"@type": "EntryPoint",
				"urlTemplate": format!("{home}?search={{search_term_string}}"),
			},
			"query-input": "required name=search_term_string",
		},
	})
}

fn web_page(cx: &Context, page: &RenderedPage, schema: SchemaType, url: &str) -> Value {
	let mut node = json!({
		"@type": schema.name(),
		"headline": page.breadcrumbs.last().map_or(page.title.as_str(), |crumb| crumb.name.as_str()),
		"description": SITE_DESCRIPTION,
		"url": url,
		"inLanguage": "en",
		"dateModified": page.modified,
		"isPartOf": { "@type": "WebSite", "name": SITE_NAME, "url": cx.absolute_url("/") },
		"breadcrumb": {
			"@type": "BreadcrumbList",
			"itemListElement": page.breadcrumbs.iter().enumerate().map(|(index, crumb)| json!({
				"@type": "ListItem",
				"position": index + 1,
				"name": crumb.name,
				"item": cx.absolute_url(&crumb.href),
			})).collect::<Vec<_>>(),
		},
	});
	if matches!(schema, SchemaType::AboutPage | SchemaType::ContactPage) {
		node["mainEntity"] = json!({ "@id": organization_id(cx) });
	}
	node
}

fn structured_data(cx: &Context, page: &RenderedPage, url: &str) -> String {
	let Some(schema) = page.schema_type else {
		return String::new();
	};
	let main = if schema == SchemaType::WebSite {
		website(cx)
	} else {
		web_page(cx, page, schema, url)
	};
	let mut graph = serde_json::Map::new();
	graph.insert("@context".to_owned(), json!("https://schema.org"));
	if matches!(
		schema,
		SchemaType::WebSite | SchemaType::AboutPage | SchemaType::ContactPage
	) {
		graph.insert("@graph".to_owned(), json!([main, organization(cx)]));
	} else if let Value::Object(fields) = main {
		graph.extend(fields);
	}
	format!(
		"<script type=\"application/ld+json\">{}</script>",
		Value::Object(graph).to_string().replace('<', "\\u003c")
	)
}

const IMAGE_TYPE: &str = "image/png";
const IMAGE_WIDTH: &str = "1200";
const IMAGE_HEIGHT: &str = "630";

pub fn head_tags(cx: &Context, page: &RenderedPage, route: &str) -> String {
	let url = cx.absolute_url(if route == "/404" { "/404.html" } else { route });
	let image = cx.absolute_url("/og-image.png");
	let not_found = page.status == 404;
	let article = page.schema_type == Some(SchemaType::TechArticle);
	let mut out = String::new();
	meta(&mut out, "name", "application-name", SITE_NAME);
	meta(&mut out, "name", "apple-mobile-web-app-title", SITE_NAME);
	meta(
		&mut out,
		"name",
		"robots",
		if not_found {
			"noindex"
		} else {
			"index, follow, max-image-preview:large"
		},
	);
	meta(&mut out, "name", "is-agentic-site-type", "content");
	meta(
		&mut out,
		"property",
		"og:type",
		if article { "article" } else { "website" },
	);
	meta(&mut out, "property", "og:site_name", SITE_NAME);
	meta(&mut out, "property", "og:title", &page.title);
	meta(&mut out, "property", "og:description", SITE_DESCRIPTION);
	meta(&mut out, "property", "og:url", &url);
	meta(&mut out, "property", "og:locale", "en_US");
	meta(&mut out, "property", "og:image", &image);
	meta(&mut out, "property", "og:image:secure_url", &image);
	meta(&mut out, "property", "og:image:type", IMAGE_TYPE);
	meta(&mut out, "property", "og:image:width", IMAGE_WIDTH);
	meta(&mut out, "property", "og:image:height", IMAGE_HEIGHT);
	meta(&mut out, "property", "og:image:alt", SITE_NAME);
	if article {
		meta(
			&mut out,
			"property",
			"article:modified_time",
			&page.modified,
		);
	}
	meta(&mut out, "name", "twitter:card", "summary_large_image");
	meta(&mut out, "name", "twitter:title", &page.title);
	meta(&mut out, "name", "twitter:description", SITE_DESCRIPTION);
	meta(&mut out, "name", "twitter:image", &image);
	meta(&mut out, "name", "twitter:image:alt", SITE_NAME);
	if !not_found {
		link(&mut out, &[("rel", "canonical"), ("href", &url)]);
		link(
			&mut out,
			&[
				("rel", "alternate"),
				("type", "text/markdown"),
				("href", &page.markdown.href),
			],
		);
		let feed_title = format!("{SITE_NAME} updates");
		link(
			&mut out,
			&[
				("rel", "alternate"),
				("type", "application/atom+xml"),
				("title", &feed_title),
				("href", rapi_routes::FEED),
			],
		);
	}
	link(
		&mut out,
		&[
			("rel", "search"),
			("type", "application/opensearchdescription+xml"),
			("title", SITE_NAME),
			("href", "/opensearch.xml"),
		],
	);
	out.push_str(&structured_data(cx, page, &url));
	out
}
