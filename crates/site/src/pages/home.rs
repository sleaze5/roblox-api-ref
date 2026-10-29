use maud::{Markup, html};
use rapi_core::api::release::release_text;
use rapi_core::icons::Icon;
use rapi_core::js::compare_names;
use rapi_core::model::Class;
use rapi_routes::{CLASSES, ENUMS, INDEX, STATS, UPDATES, class_href, upload_name};

use crate::components::page::{HeaderIcon, PageHeader, page_lead, root_crumbs};
use crate::context::Context;
use crate::html::{IconSize, icon};
use crate::layout::Shell;
use crate::meta::SITE_SUMMARY;

const EXAMPLES: [&str; 5] = [
	"Part.Touched",
	"TweenService.",
	"is:enum Material",
	"tag:yields data",
	"type:Color3",
];

fn browse_link(
	cx: &Context,
	href: &str,
	label: &str,
	count: Option<usize>,
	link_icon: Icon,
) -> Markup {
	html! {
		a href=(href) class="flex items-center gap-3 rounded-md border border-line px-4 py-3 motion-colors hover:bg-hover" {
			(icon(cx, link_icon, IconSize::S20, "text-fg-muted"))
			span class="font-medium" { (label) }
			@if let Some(count) = count { span class="ml-auto text-fg-faint" { (count) } }
		}
	}
}

pub fn home_page(cx: &Context) -> Markup {
	let api = &cx.api;
	let class_count = api.classes.len();
	let enum_count = api.enums.len();
	let mut services: Vec<&Class> = api
		.classes
		.values()
		.filter(|item| {
			item.tags.iter().any(|tag| tag == "Service")
				&& !item.tags.iter().any(|tag| tag == "Deprecated")
		})
		.collect();
	services.sort_by(|a, b| compare_names(&a.name, &b.name));
	let body = html! {
		div class="flex flex-col gap-12" {
			section class="flex flex-col gap-4 pt-8" {
				div class="flex items-center gap-3" {
					h1 class="text-xl font-semibold text-fg-bright" { "Roblox API Reference" }
					span class="tag-chip" { "Unofficial" }
				}
				p class="max-w-prose text-base text-fg-muted" {
					(SITE_SUMMARY) " This build covers " (class_count) " classes and " (enum_count) " enums from Roblox Studio "
					span class="font-mono text-sm text-fg" { (api.version) }
					" ("
					span class="font-mono text-sm text-fg" { (upload_name(&api.hash)) }
					"), with every default value in the API dump."
				}
				button type="button" data-search-open="" class="flex h-12 w-full min-w-0 items-center gap-3 rounded-md border border-line bg-raised px-4 text-left text-base text-fg-faint motion-colors hover:border-line-strong hover:text-fg-muted" {
					(icon(cx, Icon::MagnifyingGlass, IconSize::S20, ""))
					span class="truncate" { "Search classes, members, enums, and data types" }
					kbd class="ml-auto hidden kbd md:block" { "/" }
				}
				ul class="flex flex-wrap gap-2" aria-label="Example searches" {
					@for example in EXAMPLES {
						li {
							button type="button" data-search-open=(example) aria-label={ "Search for " (example) } class="button font-mono font-normal" { (example) }
						}
					}
				}
			}
			section class="flex flex-col gap-3" aria-labelledby="browse-title" {
				h2 id="browse-title" class="text-lg font-semibold" { "Browse" }
				div class="grid grid-cols-1 gap-3 md:grid-cols-2" {
					(browse_link(cx, CLASSES, "Class tree", Some(class_count), Icon::TreeStructure))
					(browse_link(cx, ENUMS, "Enums", Some(enum_count), Icon::ListBullets))
					(browse_link(cx, INDEX, "Index", None, Icon::Books))
					(browse_link(cx, UPDATES, "Updates", Some(cx.updates.len()), Icon::GitDiff))
					(browse_link(cx, STATS, "Statistics", None, Icon::ChartBar))
				}
			}
			section class="flex flex-col gap-3" aria-labelledby="services-title" {
				h2 id="services-title" class="text-lg font-semibold" { "Services" }
				ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3" {
					@for item in &services {
						li {
							a href=(class_href(&item.name)) class="flex h-7 items-center rounded-sm px-2 font-mono text-code-type motion-colors hover:bg-hover" {
								span class="truncate" { (item.name) }
							}
						}
					}
				}
			}
		}
	};
	Shell::new(None).render(cx, &body)
}

pub fn not_found_page(cx: &Context) -> Markup {
	let header = PageHeader {
		crumbs: root_crumbs(),
		icon: HeaderIcon::Named(Icon::Warning),
		title: html! { "Page not found" },
		code: false,
		docs: None,
	};
	let body = header.render(cx, Some(html! {
		(page_lead(&html! {
			"This page does not exist in " (release_text(&cx.api.version, &cx.api.hash)) ". Search for the class or enum, or browse the "
			a href=(CLASSES) class="text-fg underline decoration-line-strong hover:decoration-fg" { "class tree" }
			"."
		}))
		button type="button" data-search-open="" class="button w-fit" {
			(icon(cx, Icon::MagnifyingGlass, IconSize::S14, ""))
			"Search the API"
		}
	}));
	Shell::new(None).render(cx, &body)
}
