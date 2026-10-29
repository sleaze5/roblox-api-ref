//! Descriptions from the API docs.

use maud::{Markup, html};
use rapi_core::model::{Doc, DocElement, DocLink, DocNode, DocTag};
use rapi_routes::{MemberKind, class_href, enum_href, enum_item_href, member_href, type_href};

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

pub const fn doc_code_tone(link: &DocLink) -> &'static str {
	match link {
		DocLink::EnumItem { .. } => "constant",
		DocLink::Member {
			member: MemberKind::Method,
			..
		} => "function",
		DocLink::Member { .. } => "member",
		_ => "type",
	}
}

fn children(nodes: &[DocNode]) -> Markup {
	html! {
		@for node in nodes {
			@match node {
				DocNode::Text(text) => (text),
				DocNode::Element(element) => (element_markup(element)),
			}
		}
	}
}

fn anchor(element: &DocElement) -> Markup {
	html! {
		@match &element.link {
			None => (children(&element.children)),
			Some(DocLink::External { url }) => a href=(url) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL) {
				(children(&element.children))
				span class="sr-only" { " (opens in a new tab)" }
			},
			Some(link) => a href=(doc_link_href(link)) { (children(&element.children)) },
		}
	}
}

fn code(element: &DocElement) -> Markup {
	let code = html! { code { (children(&element.children)) } };
	match &element.link {
		None => code,
		Some(link) => {
			html! { a href=(doc_link_href(link)) data-code=(doc_code_tone(link)) { (code) } }
		}
	}
}

fn element_markup(element: &DocElement) -> Markup {
	let inner = || children(&element.children);
	match element.tag {
		DocTag::A => anchor(element),
		DocTag::Br => html! { br; },
		DocTag::Code => code(element),
		DocTag::Em => html! { em { (inner()) } },
		DocTag::Kbd => html! { kbd class="kbd" { (inner()) } },
		DocTag::Li => html! { li { (inner()) } },
		DocTag::Pre => html! { pre { (inner()) } },
		DocTag::Strong => html! { strong { (inner()) } },
		DocTag::Sup => html! { sup { (inner()) } },
		DocTag::Ul => html! { ul { (inner()) } },
	}
}

pub fn doc_text(doc: Option<&Doc>, class: &str) -> Markup {
	html! {
		@if let Some(doc) = doc {
			div class=(classes(&["doc-text", class])) { (children(doc)) }
		}
	}
}
