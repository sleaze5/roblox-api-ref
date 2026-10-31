use rapi_core::js::js_trim;
use rapi_core::model::{Doc, DocElement, DocLink, DocNode, DocTag, doc_text};

use super::text::{escape_text, inline_code, link};
use super::urls::{class_markdown_url, enum_markdown_url, type_markdown_url};
use crate::context::Context;

fn link_url(cx: &Context, target: &DocLink) -> String {
	match target {
		DocLink::Class { name } => class_markdown_url(cx, name),
		DocLink::Member { owner, .. } => class_markdown_url(cx, owner),
		DocLink::Enum { name } => enum_markdown_url(cx, name),
		DocLink::EnumItem { owner, .. } => enum_markdown_url(cx, owner),
		DocLink::Type { name } => type_markdown_url(cx, name),
		DocLink::External { url } => url.clone(),
	}
}

fn linked(cx: &Context, element: &DocElement, text: &str) -> String {
	element.link.as_ref().map_or_else(
		|| text.to_owned(),
		|target| link(text, &link_url(cx, target)),
	)
}

fn list_items(cx: &Context, element: &DocElement) -> String {
	let items: Vec<String> = element
		.children
		.iter()
		.filter_map(|child| match child {
			DocNode::Element(child) => Some(format!(
				"- {}",
				js_trim(&nodes_markdown(cx, &child.children))
			)),
			DocNode::Text(_) => None,
		})
		.collect();
	format!("\n\n{}\n\n", items.join("\n"))
}

fn element_markdown(cx: &Context, element: &DocElement) -> String {
	let children = || nodes_markdown(cx, &element.children);
	match element.tag {
		DocTag::A => linked(cx, element, &children()),
		DocTag::Br => "\n\n".to_owned(),
		DocTag::Code | DocTag::Kbd => inline_code(&doc_text(&element.children)),
		DocTag::Em => format!("*{}*", children()),
		DocTag::Strong => format!("**{}**", children()),
		DocTag::Sup => children(),
		DocTag::Li => format!("\n- {}", js_trim(&children())),
		DocTag::Pre => format!(
			"\n\n```luau\n{}\n```\n\n",
			js_trim(&doc_text(&element.children))
		),
		DocTag::Ul => list_items(cx, element),
	}
}

fn nodes_markdown(cx: &Context, nodes: &[DocNode]) -> String {
	nodes
		.iter()
		.map(|node| match node {
			DocNode::Text(text) => escape_text(text),
			DocNode::Element(element) => element_markdown(cx, element),
		})
		.collect()
}

fn tidy(text: &str) -> String {
	let mut lines: Vec<&str> = Vec::new();
	for line in text.split('\n') {
		lines.push(line.trim_end_matches([' ', '\t']));
	}
	let mut tidy = String::with_capacity(text.len());
	let mut breaks = 0;
	for (index, line) in lines.iter().enumerate() {
		if index > 0 {
			breaks += 1;
		}
		if !line.is_empty() || index == lines.len() - 1 {
			for _ in 0..breaks.min(2) {
				tidy.push('\n');
			}
			breaks = 0;
		}
		tidy.push_str(line);
	}
	tidy
}

pub fn doc_markdown(cx: &Context, doc: Option<&Doc>) -> String {
	doc.map_or_else(String::new, |doc| {
		js_trim(&tidy(&nodes_markdown(cx, doc))).to_owned()
	})
}
