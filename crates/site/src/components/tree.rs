//! Navigation trees of classes and of flat lists.

use std::collections::HashSet;

use maud::{Markup, html};
use rapi_core::api::members::is_deprecated;
use rapi_core::js::compare_names;
use rapi_core::model::Class;
use rapi_routes::class_href;

use crate::context::Context;
use crate::html::{class_icon_style, classes};

struct TreeOptions<'a> {
	cx: &'a Context,
	current: Option<&'a str>,
	open: &'a HashSet<&'a str>,
	counts: bool,
}

fn node_link(item: &Class, options: &TreeOptions) -> Markup {
	html! {
		a href=(class_href(&item.name)) style=[class_icon_style(item.icon)]
			aria-current=[(options.current == Some(item.name.as_str())).then_some("page")]
			data-deprecated=[is_deprecated(&item.tags).then_some("")] {
			span { (item.name) }
		}
	}
}

fn class_node(item: &Class, options: &TreeOptions, depth: usize) -> Markup {
	let children: Vec<&Class> = item
		.subclasses
		.iter()
		.filter_map(|name| options.cx.api.classes.get(name))
		.collect();
	html! {
		li {
			@if children.is_empty() {
				(node_link(item, options))
			} @else {
				details open[options.open.contains(item.name.as_str())] {
					summary style={ "--depth: " (depth) } {
						(node_link(item, options))
						@if options.counts { small { (children.len()) } }
					}
					ul {
						@for child in &children { (class_node(child, options, depth + 1)) }
					}
				}
			}
		}
	}
}

pub fn class_tree(
	cx: &Context,
	id: &str,
	current: Option<&str>,
	open: &HashSet<&str>,
	counts: bool,
	sticky: bool,
) -> Markup {
	let mut roots: Vec<&Class> = cx
		.api
		.classes
		.values()
		.filter(|item| item.superclass.is_none())
		.collect();
	roots.sort_by(|a, b| compare_names(&a.name, &b.name));
	let options = TreeOptions {
		cx,
		current,
		open,
		counts,
	};
	html! {
		ul id=(id) data-tree="" data-sticky-tree=[sticky.then_some("")] class=(classes(&["class-tree tree-class-icons", if sticky { "tree-sticky" } else { "" }])) {
			@for item in roots { (class_node(item, &options, 0)) }
		}
	}
}

pub struct FlatTreeItem {
	pub name: String,
	pub href: String,
	pub deprecated: bool,
	pub label: Option<String>,
}

#[derive(Clone, Copy)]
pub enum FlatTreeIcon {
	Enum,
	Type,
	Update,
}

impl FlatTreeIcon {
	const fn class(self) -> &'static str {
		match self {
			Self::Enum => "tree-enum-icons",
			Self::Type => "tree-type-icons",
			Self::Update => "tree-update-icons",
		}
	}
}

pub fn flat_tree(
	id: &str,
	items: &[FlatTreeItem],
	icon: FlatTreeIcon,
	current: Option<&str>,
) -> Markup {
	html! {
		ul id=(id) data-tree="" class=(classes(&["class-tree", icon.class()])) {
			@for item in items {
				li {
					a href=(item.href) aria-current=[(current == Some(item.name.as_str())).then_some("page")] data-deprecated=[item.deprecated.then_some("")] {
						span { (item.label.as_deref().unwrap_or(&item.name)) }
					}
				}
			}
		}
	}
}
