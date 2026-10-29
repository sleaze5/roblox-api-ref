use maud::{Markup, html};
use rapi_core::icons::Icon;
use rapi_routes::{CLASSES, ENUMS, HOME, INDEX, TYPES, UPDATES, class_href};

use crate::context::Context;
use crate::html::{IconSize, class_icon, classes, creator_hub_link, icon};

#[derive(Clone)]
pub struct Crumb {
	pub label: String,
	pub href: String,
	pub code: bool,
}

pub fn crumb(label: impl Into<String>, href: impl Into<String>) -> Crumb {
	Crumb {
		label: label.into(),
		href: href.into(),
		code: false,
	}
}

pub fn root_crumbs() -> Vec<Crumb> {
	vec![crumb("Home", HOME)]
}

pub fn class_crumbs<'a>(ancestors: impl IntoIterator<Item = &'a str>) -> Vec<Crumb> {
	let mut crumbs = vec![crumb("Home", HOME), crumb("Classes", CLASSES)];
	crumbs.extend(ancestors.into_iter().map(|name| Crumb {
		label: name.to_owned(),
		href: class_href(name),
		code: true,
	}));
	crumbs
}

pub fn enum_crumbs() -> Vec<Crumb> {
	vec![crumb("Home", HOME), crumb("Enums", ENUMS)]
}

pub fn updates_crumbs() -> Vec<Crumb> {
	vec![crumb("Home", HOME), crumb("Updates", UPDATES)]
}

pub fn index_crumbs() -> Vec<Crumb> {
	vec![crumb("Home", HOME), crumb("Index", INDEX)]
}

pub fn index_group_crumbs(group: Option<rapi_routes::FacetKind>) -> Vec<Crumb> {
	let mut crumbs = index_crumbs();
	crumbs.push(match group {
		None => crumb("Data types", TYPES),
		Some(kind) => crumb(
			rapi_core::api::facets::facet_title(kind),
			rapi_routes::facet_index_href(kind),
		),
	});
	crumbs
}

pub fn breadcrumbs(crumbs: &[Crumb]) -> Markup {
	html! {
		nav aria-label="Breadcrumb" {
			ol class="flex flex-wrap items-center gap-x-1 text-fg-muted" {
				@for (index, crumb) in crumbs.iter().enumerate() {
					li class="flex items-center gap-1" {
						@if index > 0 {
							span aria-hidden="true" class="text-fg-faint" { "›" }
						}
						a href=(crumb.href) class=(classes(&["hover:underline", if crumb.code { "font-mono text-code-type" } else { "motion-colors hover:text-fg" }])) {
							(crumb.label)
						}
					}
				}
			}
		}
	}
}

pub enum HeaderIcon {
	Named(Icon),
	Class(u32),
}

pub struct PageHeader<'a> {
	pub crumbs: Vec<Crumb>,
	pub icon: HeaderIcon,
	pub title: Markup,
	pub code: bool,
	pub docs: Option<(String, &'a str)>,
}

impl PageHeader<'_> {
	pub fn render(&self, cx: &Context, children: Option<Markup>) -> Markup {
		html! {
			header class="flex flex-col gap-4" {
				div class="flex flex-col gap-2" {
					(breadcrumbs(&self.crumbs))
					div class="flex items-center gap-3" {
						@match self.icon {
							HeaderIcon::Named(name) => (icon(cx, name, IconSize::S28, "text-fg-muted")),
							HeaderIcon::Class(index) => (class_icon(index, "size-7")),
						}
						h1 class=(classes(&["min-w-0 flex-1 text-2xl font-semibold text-fg-bright", if self.code { "font-mono break-all" } else { "break-words" }])) {
							(self.title)
						}
						@if let Some((href, name)) = &self.docs {
							(creator_hub_link(href, name))
						}
					}
				}
				@if let Some(children) = children { (children) }
			}
		}
	}
}

pub fn page_lead(children: &Markup) -> Markup {
	html! {
		p class="max-w-prose text-base text-fg-muted" { (children) }
	}
}
