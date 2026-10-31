use maud::{Markup, html};
use rapi_core::api::doc_link_hint::{doc_link_hint, doc_link_kind};
use rapi_core::model::{Doc, DocElement, DocLink, DocNode, DocTag};
use rapi_routes::{class_href, enum_href, enum_item_href, member_href, type_href};

use crate::context::Context;
use crate::html::{EXTERNAL_REL, EXTERNAL_TARGET, classes};

pub fn doc_link_href(link: &DocLink) -> String {
	match link {
		DocLink::Class { name } => class_href(name),
		DocLink::Member {
			owner,
			member,
			name,
		} => member_href(owner, *member, name),
		DocLink::Enum { name } => enum_href(name),
		DocLink::EnumItem { owner, name } => enum_item_href(owner, name),
		DocLink::Type { name } => type_href(name),
		DocLink::External { url } => url.clone(),
	}
}

fn children(cx: &Context, nodes: &[DocNode]) -> Markup {
	html! {
		@for node in nodes {
			@match node {
				DocNode::Text(text) => (text),
				DocNode::Element(element) => (element_markup(cx, element)),
			}
		}
	}
}

pub fn class_icon_index(cx: &Context, link: &DocLink) -> Option<u32> {
	match link {
		DocLink::Class { name } => cx.api.classes.get(name).and_then(|item| item.icon),
		_ => None,
	}
}

fn anchor(cx: &Context, element: &DocElement) -> Markup {
	html! {
		@match &element.link {
			None => (children(cx, &element.children)),
			Some(link @ DocLink::External { url }) => a href=(url) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL) data-tooltip=(doc_link_hint(link)) {
				(children(cx, &element.children))
				span class="sr-only" { " (opens in a new tab)" }
			},
			Some(link) => a href=(doc_link_href(link)) data-tooltip=(doc_link_hint(link)) data-tooltip-kind=[doc_link_kind(link)] data-tooltip-icon=[class_icon_index(cx, link)] { (children(cx, &element.children)) },
		}
	}
}

fn element_markup(cx: &Context, element: &DocElement) -> Markup {
	let inner = || children(cx, &element.children);
	match element.tag {
		DocTag::A => anchor(cx, element),
		DocTag::Br => html! { br; },
		DocTag::Code => html! { code { (inner()) } },
		DocTag::Em => html! { em { (inner()) } },
		DocTag::Kbd => html! { kbd class="kbd" { (inner()) } },
		DocTag::Li => html! { li { (inner()) } },
		DocTag::Pre => html! { pre { (inner()) } },
		DocTag::Strong => html! { strong { (inner()) } },
		DocTag::Sup => html! { sup { (inner()) } },
		DocTag::Ul => html! { ul { (inner()) } },
	}
}

pub fn doc_text(cx: &Context, doc: Option<&Doc>, class: &str) -> Markup {
	html! {
		@if let Some(doc) = doc {
			div class=(classes(&["doc-text", class])) { (children(cx, doc)) }
		}
	}
}
