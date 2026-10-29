use maud::{Markup, html};
use rapi_core::api::code::unlinked;
use rapi_core::api::docs_links::data_type_docs_href;
use rapi_core::api::luau::type_tokens;
use rapi_core::icons::Icon;
use rapi_core::model::{ApiType, DataType};

use crate::components::code::code_tokens;
use crate::components::facts::{fact, fact_list};
use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::components::index::{IndexCurrent, IndexGroup, data_type_category_title, index_sidebar};
use crate::components::page::{HeaderIcon, PageHeader, index_group_crumbs};
use crate::components::usage::{ROLE_SECTIONS, usage_count, usage_sections};
use crate::context::Context;
use crate::layout::{RailLink, Section, Shell, page_rail};

pub fn type_page(cx: &Context, item: &DataType) -> Markup {
	let docs = data_type_docs_href(&item.name);
	let luau = type_tokens(&ApiType {
		category: item.category.type_category(),
		name: item.name.clone(),
		optional: false,
		members: None,
	});
	let links: Vec<RailLink> = ROLE_SECTIONS
		.iter()
		.map(|section| {
			RailLink::new(section.id, section.title).count(usage_count(&item.used_by, section.role))
		})
		.filter(|link| link.count.unwrap_or_default() > 0)
		.collect();
	let header = PageHeader {
		crumbs: index_group_crumbs(None),
		icon: HeaderIcon::Named(Icon::BracketsCurly),
		title: html! { (item.name) },
		code: true,
		docs: docs.as_ref().map(|href| (href.clone(), item.name.as_str())),
	};
	let body = html! {
		article class="flex flex-col gap-6" {
			(header.render(cx, Some(html! {
				(fact_list(&html! {
					(fact("Kind", Some("The category that the API dump gives this type."), &html! { (data_type_category_title(item.category)) }))
					(fact("Luau type", Some("How this type is written in Luau code."), &html! { code class="text-fg" { (code_tokens(&unlinked(&luau))) } }))
					(fact("Uses", Some("Properties, parameters, and return values that use this type."), &html! { (item.used_by.len()) }))
				}))
				@if docs.is_none() {
					p class="text-fg-muted" { "Creator Hub has no known page for this type." }
				}
			})))
			(list_filter(cx, "Filter uses", "type-uses", "q", ListFilterPlacement::Page, None))
			div id="type-uses" class="flex flex-col gap-8" {
				(usage_sections(cx, &item.used_by, &item.name))
			}
		}
	};
	let current = IndexCurrent {
		group: IndexGroup::Type,
		name: Some(&item.name),
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, Some(&current)))
		.rail(page_rail(&links))
		.render(cx, &body)
}
