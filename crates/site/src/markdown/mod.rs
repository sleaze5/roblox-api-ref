//! The Markdown version of every page.

use rapi_routes::{
	CLASSES, ENUMS, INDEX, LLMS_TXT, NOT_FOUND_MARKDOWN, REMOVED, STATS, TYPES, UPDATES,
	class_href, enum_href, facet_index_href, markdown_href, type_href, update_href,
};

use crate::context::Context;
use crate::routes::{IndexList, Route};
use crate::site_pages::site_page;

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

pub fn page_markdown(cx: &Context, route: &Route) -> MarkdownPage {
	MarkdownPage {
		href: markdown_path(cx, route),
		content: String::new(),
	}
}
