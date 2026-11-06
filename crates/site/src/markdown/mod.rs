mod doc;
pub mod items;
pub mod lists;
mod stat_shapes;
pub mod stats;
pub mod text;
pub mod updates;
mod urls;

use rapi_routes::{LLMS_TXT, NOT_FOUND_MARKDOWN, markdown_href};

use crate::context::Context;
use crate::routes::{IndexList, Route};
use crate::site_pages::{page_content, site_page};

pub struct MarkdownPage {
	pub href: String,
	pub content: String,
}

pub fn markdown_path(cx: &Context, route: &Route) -> String {
	match route {
		Route::Home => LLMS_TXT.to_owned(),
		Route::NotFound => NOT_FOUND_MARKDOWN.to_owned(),
		_ => markdown_href(&route.href(cx)),
	}
}

fn markdown_content(cx: &Context, route: &Route) -> String {
	match route {
		Route::Home => lists::llms_txt(cx),
		Route::NotFound => lists::not_found_markdown(cx),
		Route::Class(item) => items::class_markdown(cx, item),
		Route::Enum(item) => items::enum_markdown(cx, item),
		Route::Type(item) => items::type_markdown(cx, item),
		Route::Facet(facet) => items::facet_markdown(cx, facet),
		Route::Update(update) => updates::update_markdown(cx, update),
		Route::Site(name) => {
			let page = site_page(*name);
			lists::site_page_markdown(
				cx,
				page.title,
				&page_content(*name, &cx.api.version, &cx.api.hash),
			)
		}
		Route::Classes => lists::class_list_markdown(cx),
		Route::Enums => lists::enum_list_markdown(cx),
		Route::IndexList(IndexList::Types) => lists::type_list_markdown(cx),
		Route::IndexList(IndexList::Removed) => lists::removed_list_markdown(cx),
		Route::Updates => updates::update_list_markdown(cx),
		Route::Stats => stats::stats_markdown(cx),
		Route::Index => lists::index_markdown(cx),
		Route::FacetIndex(kind) => lists::facet_index_markdown(cx, *kind),
	}
}

pub fn page_markdown(cx: &Context, route: &Route) -> MarkdownPage {
	MarkdownPage {
		href: markdown_path(cx, route),
		content: markdown_content(cx, route),
	}
}
