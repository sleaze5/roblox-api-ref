use rapi_routes::{FacetKind, class_href, enum_href, facet_index_href, markdown_href, type_href};

use crate::context::Context;

pub fn markdown_url(cx: &Context, href: &str) -> String {
	cx.absolute_url(&markdown_href(href))
}

pub fn class_markdown_url(cx: &Context, name: &str) -> String {
	markdown_url(cx, &class_href(name))
}

pub fn enum_markdown_url(cx: &Context, name: &str) -> String {
	markdown_url(cx, &enum_href(name))
}

pub fn type_markdown_url(cx: &Context, name: &str) -> String {
	markdown_url(cx, &type_href(name))
}

pub fn facet_markdown_url(cx: &Context, kind: FacetKind, name: &str) -> String {
	markdown_url(cx, &cx.facet_href(kind, name))
}

pub fn facet_index_markdown_url(cx: &Context, kind: FacetKind) -> String {
	markdown_url(cx, &facet_index_href(kind))
}
