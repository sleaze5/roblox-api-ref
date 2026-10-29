//! The Classes, Enums, and Index pages and their lists.

use std::collections::HashSet;

use maud::{Markup, html};
use rapi_core::api::facets::{
	INDEX_DESCRIPTION, facet_index_description, facet_title, sorted_facets,
};
use rapi_core::api::text::REMOVED_LEAD;
use rapi_core::icons::Icon;
use rapi_core::js::{compare_names, format_date};
use rapi_core::model::{DataType, DataTypeCategory, Enum, RemovedOwner};
use rapi_routes::{
	FACET_KINDS, FacetKind, OwnerKind, REMOVED, TYPES, enum_href, facet_index_href, owner_anchor,
	type_href, update_href,
};

use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::components::index::{
	IndexCurrent, IndexGroup, data_type_category_title, facet_icon, index_sidebar,
	sorted_data_types,
};
use crate::components::page::{HeaderIcon, PageHeader, index_crumbs, page_lead, root_crumbs};
use crate::components::tree::class_tree;
use crate::context::Context;
use crate::html::{IconSize, classes, icon};
use crate::layout::{RailLink, Section, Shell, page_rail};
use crate::pages::class::class_sidebar;
use crate::pages::enum_page::enum_sidebar;

pub const INDEX_GRID_CLASS: &str = "grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3";

fn simple_header(
	cx: &Context,
	crumbs: Vec<crate::components::page::Crumb>,
	icon: Icon,
	title: &str,
	lead: &Markup,
) -> Markup {
	PageHeader {
		crumbs,
		icon: HeaderIcon::Named(icon),
		title: html! { (title) },
		code: false,
		docs: None,
	}
	.render(cx, Some(page_lead(lead)))
}

pub fn class_index_page(cx: &Context) -> Markup {
	let count = cx.api.classes.len();
	let open: HashSet<&str> = cx.api.classes.keys().map(String::as_str).collect();
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, root_crumbs(), Icon::TreeStructure, "Classes", &html! { "All " (count) " classes, arranged by inheritance." }))
			(list_filter(cx, "Filter classes", "class-tree", "q", ListFilterPlacement::Page, None))
			(class_tree(cx, "class-tree", None, &open, true, false))
		}
	};
	Shell::new(Some(Section::Classes))
		.sidebar(class_sidebar(cx, None, &[]))
		.rail(page_rail(&[
			RailLink::new("class-tree", "Classes").count(count)
		]))
		.render(cx, &body)
}

pub fn enum_index_page(cx: &Context) -> Markup {
	let mut enums: Vec<&Enum> = cx.api.enums.values().collect();
	enums.sort_by(|a, b| compare_names(&a.name, &b.name));
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, root_crumbs(), Icon::ListBullets, "Enums", &html! { "All " (enums.len()) " enums, in alphabetical order." }))
			(list_filter(cx, "Filter enums", "enum-list", "q", ListFilterPlacement::Page, None))
			ul id="enum-list" class="grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3" {
				@for item in &enums {
					@let deprecated = item.tags.iter().any(|tag| tag == "Deprecated");
					li data-name=(item.name) {
						a href=(enum_href(&item.name))
							aria-label=[deprecated.then(|| format!("{} (deprecated)", item.name))]
							class=(classes(&["flex h-7 items-center justify-between gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover", if deprecated { "text-code-type-muted hover:text-code-type focus-visible:text-code-type" } else { "text-code-type" }])) {
							span class="truncate" { (item.name) }
							span class="font-sans text-xs text-fg-faint" { (item.items.len()) }
						}
					}
				}
			}
		}
	};
	Shell::new(Some(Section::Enums))
		.sidebar(enum_sidebar(cx, None))
		.rail(page_rail(&[
			RailLink::new("enum-list", "Enums").count(enums.len())
		]))
		.render(cx, &body)
}

pub fn index_row_markup(name: &str, href: &str, count: usize, is_type: bool) -> Markup {
	html! {
		li data-name=(name) {
			a href=(href) class=(classes(&["flex h-7 items-center justify-between gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover", if is_type { "text-code-type" } else { "text-fg" }])) {
				span class="truncate" { (name) }
				span class="font-sans text-xs text-fg-faint" { (count) }
			}
		}
	}
}

pub fn index_section_markup(
	cx: &Context,
	id: &str,
	title: &str,
	section_icon: Icon,
	count: usize,
	href: Option<&str>,
	children: &Markup,
) -> Markup {
	html! {
		section id=(id) data-list-section="" aria-labelledby={ (id) "-title" } class="flex flex-col gap-3" {
			h2 id={ (id) "-title" } class="flex items-center gap-2 text-lg font-semibold" {
				(icon(cx, section_icon, IconSize::S20, "text-fg-muted"))
				@match href {
					None => (title),
					Some(href) => a href=(href) class="hover:text-fg-bright hover:underline" { (title) },
				}
				span class="text-sm font-normal text-fg-faint" { (count) }
			}
			(children)
		}
	}
}

fn type_rows(types: &[&DataType]) -> Markup {
	html! {
		ul class=(INDEX_GRID_CLASS) {
			@for item in types { (index_row_markup(&item.name, &type_href(&item.name), item.used_by.len(), true)) }
		}
	}
}

fn data_type_groups(cx: &Context, standalone: bool) -> Markup {
	let types = sorted_data_types(cx);
	html! {
		@for category in DataTypeCategory::ALL {
			@let members: Vec<&DataType> = types.iter().copied().filter(|item| item.category == category).collect();
			@let id = category.name();
			@if standalone {
				(index_section_markup(cx, id, data_type_category_title(category), Icon::BracketsCurly, members.len(), None, &type_rows(&members)))
			} @else {
				section id=(id) data-list-section="" aria-labelledby={ (id) "-title" } class="flex flex-col gap-1" {
					h3 id={ (id) "-title" } class="px-2 text-xs font-medium text-fg-muted" { (data_type_category_title(category)) }
					(type_rows(&members))
				}
			}
		}
	}
}

fn removed_row(owner: &RemovedOwner, kind: OwnerKind) -> Markup {
	html! {
		li data-name=(owner.name) class="flex flex-wrap items-baseline gap-x-3 px-2 py-1" {
			code class="text-fg-muted" { (owner.name) }
			span class="text-fg-faint" {
				"Removed in "
				a href={ (update_href(&owner.removed.version)) "#" (owner_anchor(kind, &owner.name)) } class="font-mono text-fg-muted hover:text-fg hover:underline" { (owner.removed.version) }
			}
			span class="text-xs text-fg-faint" { (format_date(&owner.removed.published)) }
		}
	}
}

fn facet_rows(cx: &Context, kind: FacetKind) -> Markup {
	html! {
		@for facet in sorted_facets(&cx.api, kind) {
			(index_row_markup(&facet.name, &cx.facet_href(kind, &facet.name), facet.uses.len(), false))
		}
	}
}

pub fn index_page(cx: &Context) -> Markup {
	let api = &cx.api;
	let removed_count = api.removed_classes().len() + api.removed_enums().len();
	let mut links = vec![RailLink::new("data-types", "Data types").count(api.data_types.len())];
	links.extend(FACET_KINDS.map(|kind| {
		RailLink::new(kind.path(), facet_title(kind)).count(api.facets.of(kind).len())
	}));
	links.push(RailLink::new("removed", "Removed").count(removed_count));
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, root_crumbs(), Icon::Books, "Index", &html! { (INDEX_DESCRIPTION) }))
			(list_filter(cx, "Filter the index", "index-list", "q", ListFilterPlacement::Page, None))
			div id="index-list" class="flex flex-col gap-8" {
				(index_section_markup(cx, "data-types", "Data types", Icon::BracketsCurly, api.data_types.len(), Some(TYPES), &data_type_groups(cx, false)))
				@for kind in FACET_KINDS {
					(index_section_markup(cx, kind.path(), facet_title(kind), facet_icon(kind), api.facets.of(kind).len(), Some(&facet_index_href(kind)), &html! {
						ul class=(INDEX_GRID_CLASS) { (facet_rows(cx, kind)) }
					}))
				}
				(index_section_markup(cx, "removed", "Removed", Icon::ClockCounterClockwise, removed_count, Some(REMOVED), &html! {
					ul class="flex flex-col" {
						@for owner in api.removed_classes() { (removed_row(owner, OwnerKind::Class)) }
						@for owner in api.removed_enums() { (removed_row(owner, OwnerKind::Enum)) }
					}
				}))
			}
		}
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, None))
		.rail(page_rail(&links))
		.render(cx, &body)
}

pub fn facet_index_page(cx: &Context, kind: FacetKind) -> Markup {
	let title = facet_title(kind);
	let count = cx.api.facets.of(kind).len();
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, index_crumbs(), facet_icon(kind), title, &html! { (facet_index_description(kind)) }))
			(list_filter(cx, &format!("Filter {}", title.to_lowercase()), "facet-list", "q", ListFilterPlacement::Page, None))
			ul id="facet-list" class=(INDEX_GRID_CLASS) { (facet_rows(cx, kind)) }
		}
	};
	let current = IndexCurrent {
		group: IndexGroup::Facet(kind),
		name: None,
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, Some(&current)))
		.rail(page_rail(
			&[RailLink::new("facet-list", title).count(count)],
		))
		.render(cx, &body)
}

pub fn removed_index_page(cx: &Context) -> Markup {
	let groups = [
		(
			OwnerKind::Class,
			"removed-classes",
			"Classes",
			Icon::Cube,
			cx.api.removed_classes(),
		),
		(
			OwnerKind::Enum,
			"removed-enums",
			"Enums",
			Icon::ListBullets,
			cx.api.removed_enums(),
		),
	];
	let links: Vec<RailLink> = groups
		.iter()
		.map(|(_, id, title, _, owners)| RailLink::new(*id, *title).count(owners.len()))
		.collect();
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, index_crumbs(), Icon::ClockCounterClockwise, "Removed", &html! { (REMOVED_LEAD) }))
			(list_filter(cx, "Filter removed", "removed-list", "q", ListFilterPlacement::Page, None))
			div id="removed-list" class="flex flex-col gap-8" {
				@for (kind, id, title, group_icon, owners) in &groups {
					(index_section_markup(cx, id, title, *group_icon, owners.len(), None, &html! {
						ul class="flex flex-col" {
							@for owner in *owners { (removed_row(owner, *kind)) }
						}
					}))
				}
			}
		}
	};
	let current = IndexCurrent {
		group: IndexGroup::Removed,
		name: None,
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, Some(&current)))
		.rail(page_rail(&links))
		.render(cx, &body)
}

pub fn type_index_page(cx: &Context) -> Markup {
	let types = &cx.api.data_types;
	let links: Vec<RailLink> = DataTypeCategory::ALL
		.into_iter()
		.map(|category| {
			RailLink::new(category.name(), data_type_category_title(category)).count(
				types
					.values()
					.filter(|item| item.category == category)
					.count(),
			)
		})
		.collect();
	let body = html! {
		article class="flex flex-col gap-6" {
			(simple_header(cx, index_crumbs(), Icon::BracketsCurly, "Data types", &html! { "All " (types.len()) " data types: primitive types, group types, and Roblox types." }))
			(list_filter(cx, "Filter data types", "type-list", "q", ListFilterPlacement::Page, None))
			div id="type-list" class="flex flex-col gap-8" { (data_type_groups(cx, true)) }
		}
	};
	let current = IndexCurrent {
		group: IndexGroup::Type,
		name: None,
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, Some(&current)))
		.rail(page_rail(&links))
		.render(cx, &body)
}
