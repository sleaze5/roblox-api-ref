use maud::{Markup, html};

use crate::html::classes;

pub fn fact(label: &str, hint: Option<&str>, children: &Markup) -> Markup {
	html! {
		div class="flex items-center gap-2" {
			dt data-tooltip=[hint] class=(classes(&["shrink-0 whitespace-nowrap text-fg-faint", if hint.is_some() { "cursor-help underline decoration-line-strong decoration-dotted underline-offset-4" } else { "" }])) {
				(label)
			}
			dd class="min-w-0 text-fg-muted" { (children) }
		}
	}
}

pub fn fact_list(children: &Markup) -> Markup {
	html! {
		dl class="flex flex-wrap gap-x-6 gap-y-1" { (children) }
	}
}
