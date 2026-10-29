use maud::{Markup, html};
use rapi_core::icons::Icon;

use crate::context::Context;
use crate::html::{IconSize, classes, icon};

pub fn filter_field(
	cx: &Context,
	label: &str,
	placeholder: &str,
	count: bool,
	class: &str,
) -> Markup {
	html! {
		div class=(classes(&["relative flex items-center", class])) {
			label class="flex min-w-0 flex-1 items-center" {
				span class="sr-only" { (label) }
				(icon(cx, Icon::MagnifyingGlass, IconSize::S14, "pointer-events-none absolute left-3 text-fg-muted"))
				input type="search" name="text" placeholder=(placeholder) autocomplete="off" spellcheck="false"
					class=(classes(&["field field-icon", if count { "field-count" } else { "" }]));
			}
			@if count {
				output data-filter-summary="" aria-live="polite" class="pointer-events-none absolute right-3 text-xs text-fg-faint" {}
			}
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ListFilterPlacement {
	Page,
	Sidebar,
}

impl ListFilterPlacement {
	const fn class(self) -> &'static str {
		match self {
			Self::Page => "sticky top-header z-10 -mx-3 border-b border-line bg-canvas px-3 py-2",
			Self::Sidebar => "sticky -top-3 z-20 -mx-3 -mt-3 bg-surface p-3",
		}
	}
}

pub fn list_filter(
	cx: &Context,
	label: &str,
	target: &str,
	param: &str,
	placement: ListFilterPlacement,
	noun: Option<&str>,
) -> Markup {
	html! {
		form data-list-filter=(target) data-param=(param) data-noun=[noun] role="search" aria-label=(label) class=(placement.class()) {
			(filter_field(cx, label, label, placement == ListFilterPlacement::Page, ""))
		}
	}
}
