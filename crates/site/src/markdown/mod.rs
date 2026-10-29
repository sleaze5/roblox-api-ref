mod doc;
pub mod items;
pub mod lists;
pub mod stats;
pub mod text;
pub mod updates;
mod urls;

use rapi_routes::{
	CLASSES, ENUMS, INDEX, LLMS_TXT, NOT_FOUND_MARKDOWN, REMOVED, STATS, TYPES, UPDATES,
	class_href, enum_href, facet_index_href, markdown_href, type_href, update_href,
};

use crate::context::Context;
use crate::routes::{IndexList, Route};
use crate::site_pages::{page_content, site_page};

pub struct MarkdownPage {
	pub href: String,
	pub content: String,
}

pub fn markdown_path(cx: &Context, route: &Route) -> String {
	match route {
		Route::Home => return LLMS_TXT.to_owned(),
		Route::NotFound => return NOT_FOUND_MARKDOWN.to_owned(),
		_ => {}
	}
	markdown_href(&match route {
		Route::Class(item) => class_href(&item.name),
		Route::Enum(item) => enum_href(&item.name),
		Route::Type(item) => type_href(&item.name),
		Route::Update(update) => update_href(&update.version),
		Route::Facet(facet) => cx.facet_href(facet.kind, &facet.name),
		Route::Site(name) => site_page(*name).href.to_owned(),
		Route::Classes => CLASSES.to_owned(),
		Route::Enums => ENUMS.to_owned(),
		Route::IndexList(IndexList::Types) => TYPES.to_owned(),
		Route::IndexList(IndexList::Removed) => REMOVED.to_owned(),
		Route::Updates => UPDATES.to_owned(),
		Route::Stats => STATS.to_owned(),
		Route::Index => INDEX.to_owned(),
		Route::FacetIndex(kind) => facet_index_href(*kind),
		Route::Home | Route::NotFound => unreachable!(),
	})
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
