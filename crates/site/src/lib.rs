//! Renders every page of the site, with its Markdown version and site files.

pub mod build;
pub mod components;
pub mod context;
pub mod feed;
mod filter_summary;
pub mod head;
pub mod html;
pub mod layout;
pub mod markdown;
pub mod meta;
pub mod pages;
pub mod routes;
pub mod site_pages;
pub mod template;

use maud::Markup;

pub use context::Context;
use markdown::{MarkdownPage, page_markdown};
use meta::{Breadcrumb, SchemaType, breadcrumbs, page_title, schema_type};
use routes::{Route, match_route};

pub struct RenderedPage {
	pub status: u16,
	pub title: String,
	pub body: String,
	pub schema_type: Option<SchemaType>,
	pub breadcrumbs: Vec<Breadcrumb>,
	pub markdown: MarkdownPage,
	pub modified: String,
}

fn page_body(cx: &Context, route: &Route) -> Markup {
	match route {
		Route::Site(name) => pages::site::site_page_body(cx, *name),
		Route::Class(item) => pages::class::class_page(cx, item),
		_ => maud::html! {},
	}
}

pub fn render_page(cx: &Context, path: &str) -> RenderedPage {
	let route = match_route(cx, path);
	RenderedPage {
		status: if matches!(route, Route::NotFound) {
			404
		} else {
			200
		},
		title: page_title(&route),
		body: filter_summary::fill_filter_summaries(&page_body(cx, &route).into_string()),
		schema_type: schema_type(&route),
		breadcrumbs: breadcrumbs(cx, &route),
		markdown: page_markdown(cx, &route),
		modified: cx.api.published.clone(),
	}
}
