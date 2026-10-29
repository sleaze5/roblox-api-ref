use maud::{Markup, PreEscaped, html};
use rapi_core::icons::Icon;
use rapi_routes::{
	ABOUT, CLASSES, CONTACT, ENUMS, HOME, INDEX, PRIVACY, STATS, UPDATES, upload_name,
};

use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::context::Context;
use crate::html::{EXTERNAL_REL, EXTERNAL_TARGET, IconSize, classes, icon};
use crate::meta::REPOSITORY_URL;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Section {
	Home,
	Classes,
	Enums,
	Index,
	Updates,
	Stats,
}

const SECTION_LINKS: [(Section, &str, &str); 5] = [
	(Section::Classes, "Classes", CLASSES),
	(Section::Enums, "Enums", ENUMS),
	(Section::Index, "Index", INDEX),
	(Section::Updates, "Updates", UPDATES),
	(Section::Stats, "Statistics", STATS),
];

const DRAWER_CLASS: &str = "fixed inset-y-0 left-0 m-0 motion-drawer h-dvh w-sidebar max-w-drawer overflow-x-hidden overflow-y-auto overscroll-contain border-r border-line bg-surface p-3 text-sm text-fg";

const SCROLL_TO_CURRENT: &str = "{const n=document.getElementById('sidebar'),c=n&&n.querySelector('[data-tree] [aria-current=page]');if(c)n.scrollTop=c.getBoundingClientRect().top-n.getBoundingClientRect().top-n.clientHeight/3}";

pub struct Shell {
	pub section: Option<Section>,
	pub sidebar: Option<Markup>,
	pub rail: Option<Markup>,
}

impl Shell {
	pub const fn new(section: Option<Section>) -> Self {
		Self {
			section,
			sidebar: None,
			rail: None,
		}
	}

	#[must_use]
	pub fn sidebar(mut self, sidebar: Markup) -> Self {
		self.sidebar = Some(sidebar);
		self
	}

	#[must_use]
	pub fn rail(mut self, rail: Markup) -> Self {
		self.rail = Some(rail);
		self
	}

	pub fn render(self, cx: &Context, children: &Markup) -> Markup {
		let has_sidebar = self.sidebar.is_some();
		html! {
			a href="#main" class="sr-only rounded-sm bg-raised px-3 py-2 text-fg focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-30" { "Skip to content" }
			(header(cx, self.section))
			div class="flex" {
				nav id="sidebar" popover="auto" aria-label="Navigation"
					data-drawer-only=[(!has_sidebar).then_some("")]
					class=(classes(&[DRAWER_CLASS, if has_sidebar { "md:sticky md:top-header md:h-below-header md:max-w-none md:shrink-0" } else { "" }])) {
					(drawer_sections(self.section))
					@if let Some(sidebar) = &self.sidebar { (sidebar) }
				}
				@if has_sidebar {
					script { (PreEscaped(SCROLL_TO_CURRENT)) }
				}
				@if let Some(rail) = &self.rail {
					aside class="fixed top-header right-0 hidden h-below-header w-rail overflow-y-auto overscroll-contain py-8 pr-4 lg:block" { (rail) }
				}
				div class=(classes(&["flex min-h-below-header min-w-0 flex-1 flex-col", if self.rail.is_some() { "lg:rail-gutter" } else { "" }])) {
					main id="main" class="flex-1 px-4 py-6 md:px-8 md:py-8" {
						div class="mx-auto max-w-content" { (children) }
					}
					(footer())
				}
			}
			div data-search-root="" {}
			div id="tooltip" popover="manual" aria-hidden="true" class="tooltip" {}
			div id="announcer" aria-live="polite" class="sr-only" {}
		}
	}
}

fn current(section: Option<Section>, link: Section) -> Option<&'static str> {
	(section == Some(link)).then_some("page")
}

fn header(cx: &Context, section: Option<Section>) -> Markup {
	let label = format!("{} ({})", upload_name(&cx.api.hash), cx.api.version);
	html! {
		header class="sticky top-0 z-20 flex h-12 items-center gap-2 border-b border-line bg-surface px-2 md:gap-3 md:px-4" {
			div class="flex min-w-0 flex-1 items-center gap-2 md:gap-3" {
				button type="button" popovertarget="sidebar" aria-label="Open navigation" class="grid size-10 shrink-0 place-items-center rounded-sm text-fg-muted motion-colors hover:bg-hover hover:text-fg md:hidden" {
					(icon(cx, Icon::List, IconSize::S20, ""))
				}
				a href=(HOME) class="truncate font-semibold whitespace-nowrap text-fg" { "Roblox API Reference" }
				nav aria-label="Sections" class="hidden items-center gap-1 md:flex" {
					@for (link, label, href) in SECTION_LINKS {
						a href=(href) aria-current=[current(section, link)]
							class=(classes(&["inline-flex h-8 items-center rounded-sm px-2 font-medium whitespace-nowrap motion-colors hover:bg-hover hover:text-fg", if section == Some(link) { "text-fg" } else { "text-fg-muted" }])) {
							(label)
						}
					}
				}
			}
			button type="button" data-search-open="" aria-label="Search the API" aria-keyshortcuts="Control+K /" class="ml-auto flex h-10 shrink-0 items-center gap-2 rounded-sm px-2.5 text-fg-muted motion-colors hover:bg-hover hover:text-fg md:ml-0 md:h-8 md:w-80 md:shrink md:border md:border-line md:bg-raised md:text-fg-faint md:hover:border-line-strong md:hover:bg-raised md:hover:text-fg-muted" {
				(icon(cx, Icon::MagnifyingGlass, IconSize::S16, ""))
				span class="hidden md:inline" { "Search the API" }
				kbd class="ml-auto hidden kbd md:block" { "Ctrl K" }
			}
			div class="hidden min-w-0 flex-1 items-center justify-end gap-3 lg:flex" {
				span data-tooltip=(label) class="min-w-0 truncate font-mono text-xs text-fg-faint" { (label) }
			}
		}
	}
}

fn drawer_sections(section: Option<Section>) -> Markup {
	html! {
		ul aria-label="Sections" class="mb-3 flex flex-col border-b border-line pb-3 md:hidden" {
			@for (link, label, href) in SECTION_LINKS {
				li {
					a href=(href) aria-current=[current(section, link)]
						class=(classes(&["flex h-10 items-center rounded-sm px-2 font-medium motion-colors hover:bg-hover hover:text-fg", if section == Some(link) { "bg-active text-fg-bright" } else { "text-fg-muted" }])) {
						(label)
					}
				}
			}
		}
	}
}

const FOOTER_LINK_CLASS: &str = "rounded-sm motion-colors hover:text-fg";

fn footer() -> Markup {
	html! {
		footer class="border-t border-line px-4 py-6 md:px-8" {
			div class="mx-auto flex max-w-content flex-col gap-3 text-sm text-fg-muted md:flex-row md:items-center md:justify-between" {
				nav aria-label="Site" {
					ul class="flex flex-wrap gap-x-4 gap-y-2" {
						@for (label, href) in [("About", ABOUT), ("Contact", CONTACT), ("Privacy policy", PRIVACY)] {
							li { a href=(href) class=(FOOTER_LINK_CLASS) { (label) } }
						}
						li {
							a href=(REPOSITORY_URL) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL) class=(FOOTER_LINK_CLASS) {
								"GitHub"
								span class="sr-only" { " (opens in a new tab)" }
							}
						}
					}
				}
				p class="text-fg-faint" { "This site is not affiliated with Roblox Corporation." }
			}
		}
	}
}

pub fn section_sidebar(cx: &Context, label: &str, target: &str, children: &Markup) -> Markup {
	html! {
		(list_filter(cx, label, target, "nav", ListFilterPlacement::Sidebar, None))
		(children)
	}
}
