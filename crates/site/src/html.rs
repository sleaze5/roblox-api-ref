//! Small helpers shared by the page templates.

use maud::{Markup, html};
use rapi_core::icons::Icon;

use crate::context::Context;

/// Joins the non-empty class names with spaces, like a Solid class array.
pub fn classes(parts: &[&str]) -> String {
	let mut joined = String::new();
	for part in parts.iter().filter(|part| !part.is_empty()) {
		if !joined.is_empty() {
			joined.push(' ');
		}
		joined.push_str(part);
	}
	joined
}

pub const EXTERNAL_TARGET: &str = "_blank";
pub const EXTERNAL_REL: &str = "external noopener noreferrer";

/// The icon sizes that the design uses, in pixels.
#[derive(Clone, Copy)]
pub enum IconSize {
	S14,
	S16,
	S20,
	S28,
}

impl IconSize {
	const fn class(self) -> &'static str {
		match self {
			Self::S14 => "size-3.5",
			Self::S16 => "size-4",
			Self::S20 => "size-5",
			Self::S28 => "size-7",
		}
	}
}

/// An icon from the sprite, sized with a design token.
pub fn icon(cx: &Context, name: Icon, size: IconSize, class: &str) -> Markup {
	html! {
		svg class=(classes(&["shrink-0", size.class(), class])) aria-hidden="true" {
			use href={ (cx.sprite) "#" (name.name()) } {}
		}
	}
}

/// An icon from the sprite whose size comes from `class`.
pub fn sprite_icon(cx: &Context, name: Icon, class: &str) -> Markup {
	html! {
		svg class=(class) aria-hidden="true" {
			use href={ (cx.sprite) "#" (name.name()) } {}
		}
	}
}

pub fn class_icon_style(icon: Option<u32>) -> Option<String> {
	icon.map(|icon| format!("--icon: {icon}"))
}

pub fn class_icon(icon: u32, class: &str) -> Markup {
	html! {
		span aria-hidden="true" style=(format!("--icon: {icon}")) class=(classes(&["shrink-0 class-icon", class])) {}
	}
}

pub fn external_link(href: &str, content: &Markup) -> Markup {
	html! {
		a href=(href) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL) {
			(content)
			span class="sr-only" { " (opens in a new tab)" }
		}
	}
}

pub fn creator_hub_link(href: &str, name: &str) -> Markup {
	html! {
		a href=(href) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL)
			data-tooltip="Open on Creator Hub"
			aria-label={ "Open " (name) " on Creator Hub (opens in a new tab)" }
			class="docs-link" {}
	}
}
