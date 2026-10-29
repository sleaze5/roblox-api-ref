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
		Route::Enum(item) => pages::enum_page::enum_page(cx, item),
		Route::Type(item) => pages::type_page::type_page(cx, item),
		Route::Facet(facet) => pages::facet::facet_page(cx, facet),
		Route::Home => pages::home::home_page(cx),
		Route::Updates => pages::updates::updates_page(cx),
		Route::Stats => pages::stats::stats_page(cx),
		Route::Update(update) => pages::updates::update_page(cx, update),
		Route::NotFound => pages::home::not_found_page(cx),
		Route::Classes => pages::index_pages::class_index_page(cx),
		Route::Enums => pages::index_pages::enum_index_page(cx),
		Route::Index => pages::index_pages::index_page(cx),
		Route::FacetIndex(kind) => pages::index_pages::facet_index_page(cx, *kind),
		Route::IndexList(routes::IndexList::Types) => pages::index_pages::type_index_page(cx),
		Route::IndexList(routes::IndexList::Removed) => pages::index_pages::removed_index_page(cx),
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

pub fn page_index(cx: &Context) -> Vec<(String, String)> {
	routes::route_paths(cx)
		.into_iter()
		.map(|path| {
			let markdown = markdown::markdown_path(cx, &match_route(cx, &path));
			(path, markdown)
		})
		.collect()
}
