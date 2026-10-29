use maud::{Markup, html};
use rapi_core::api::code::{CodeTone, token};
use rapi_core::api::docs_links::enum_docs_href;
use rapi_core::api::history_text::HistoryNoun;
use rapi_core::api::tags::plain_tags;
use rapi_core::icons::Icon;
use rapi_core::js::compare_names;
use rapi_core::model::{Enum, EnumItem};
use rapi_routes::{OwnerKind, change_anchor, enum_href, enum_item_anchor, owner_anchor};

use crate::components::changes::{ChangedValue, RemovedList, removed_items};
use crate::components::code::code_tokens;
use crate::components::doc::doc_text;
use crate::components::facts::{fact, fact_list};
use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::components::history::{history_facts, history_list};
use crate::components::page::{HeaderIcon, PageHeader, enum_crumbs};
use crate::components::tags::{dated_tags, tag_list};
use crate::components::tree::{FlatTreeIcon, FlatTreeItem, flat_tree};
use crate::components::usage::{ROLE_SECTIONS, usage_count, usage_sections};
use crate::context::Context;
use crate::layout::{RailLink, Section, Shell, page_rail, section_sidebar};

pub fn copy_link(name: &str, href: &str) -> Markup {
	html! {
		a href=(href) data-copy-link="" data-tooltip="Copy link" aria-label={ "Copy link to " (name) } class="copy-link" {}
	}
}

fn enum_item_row(owner: &Enum, item: &EnumItem) -> Markup {
	let anchor = enum_item_anchor(&item.name);
	let tokens = [
		token("Enum.", CodeTone::Punctuation),
		token(&owner.name, CodeTone::Type),
		token(".", CodeTone::Punctuation),
		token(&item.name, CodeTone::Constant),
		token(" = ", CodeTone::Punctuation),
		token(item.value.to_string(), CodeTone::Constant),
	];
	html! {
		li id=(anchor) data-name=(item.name) class="member-card" {
			div class="member-head" {
				code class="member-code" { (code_tokens(&tokens)) }
				div class="member-actions" { (copy_link(&item.name, &format!("#{anchor}"))) }
			}
			(doc_text(item.description.as_ref(), "member-description"))
			(tag_list(&dated_tags(plain_tags(&item.tags), item.history.as_ref()), ""))
			@if !item.legacy_names.is_empty() {
				p class="text-fg-muted" {
					"Legacy names: "
					code class="text-fg" { (item.legacy_names.join(", ")) }
				}
			}
			(history_list(item.history.as_ref(), &change_anchor(OwnerKind::Enum, &owner.name, &item.name), HistoryNoun::Member, ""))
		}
	}
}

pub fn enum_sidebar(cx: &Context, current: Option<&str>) -> Markup {
	let mut enums: Vec<&Enum> = cx.api.enums.values().collect();
	enums.sort_by(|a, b| compare_names(&a.name, &b.name));
	let items: Vec<FlatTreeItem> = enums
		.into_iter()
		.map(|item| FlatTreeItem {
			name: item.name.clone(),
			href: enum_href(&item.name),
			deprecated: item.tags.iter().any(|tag| tag == "Deprecated"),
			label: None,
		})
		.collect();
	section_sidebar(
		cx,
		"Filter enums",
		"sidebar-tree",
		&flat_tree("sidebar-tree", &items, FlatTreeIcon::Enum, current),
	)
}

pub fn enum_page(cx: &Context, item: &Enum) -> Markup {
	let mut links = vec![RailLink::new("items", "Items").count(item.items.len())];
	for section in &ROLE_SECTIONS {
		let count = usage_count(&item.used_by, section.role);
		if count > 0 {
			links.push(RailLink::new(section.id, section.title).count(count));
		}
	}
	if let Some(removed) = &item.removed_items {
		links.push(RailLink::new("removed-items", "Removed items").count(removed.len()));
	}
	let docs_name = format!("Enum.{}", item.name);
	let header = PageHeader {
		crumbs: enum_crumbs(),
		icon: HeaderIcon::Named(Icon::ListBullets),
		title: html! { span class="text-fg-muted" { "Enum." } (item.name) },
		code: true,
		docs: Some((enum_docs_href(&item.name), &docs_name)),
	};
	let body = html! {
		article class="flex flex-col gap-6" {
			(header.render(cx, Some(html! {
				(doc_text(item.description.as_ref(), "max-w-prose text-base text-fg"))
				(tag_list(&dated_tags(plain_tags(&item.tags), item.history.as_ref()), ""))
				(fact_list(&html! {
					(fact("Items", Some("Enum items that this enum defines."), &html! { (item.items.len()) }))
					(fact("Uses", Some("Properties, parameters, and return values that use this enum."), &html! { (item.used_by.len()) }))
				}))
				(fact_list(&history_facts(item.history.as_ref(), OwnerKind::Enum, &item.name, cx.api.history_start.as_deref())))
				(history_list(item.history.as_ref(), &owner_anchor(OwnerKind::Enum, &item.name), HistoryNoun::Item, ""))
			})))
			(list_filter(cx, "Filter items and uses", "enum-content", "q", ListFilterPlacement::Page, None))
			div id="enum-content" class="flex flex-col gap-8" {
				section id="items" data-list-section="" aria-labelledby="items-title" class="flex flex-col gap-3" {
					h2 id="items-title" class="flex items-center gap-2 text-lg font-semibold" {
						(crate::html::icon(cx, Icon::Hash, crate::html::IconSize::S20, "text-fg-muted"))
						"Items"
						span class="text-sm font-normal text-fg-faint" { (item.items.len()) }
					}
					ul class="flex flex-col gap-4" {
						@for entry in &item.items { (enum_item_row(item, entry)) }
					}
				}
				(usage_sections(cx, &item.used_by, &docs_name))
			}
			(removed_items(
				cx,
				&RemovedList { id: "removed-items", title: "Removed items", kind: OwnerKind::Enum, owner: &item.name },
				item.removed_items.as_deref(),
				ChangedValue::EnumItem,
			))
		}
	};
	Shell::new(Some(Section::Enums))
		.sidebar(enum_sidebar(cx, Some(&item.name)))
		.rail(page_rail(&links))
		.render(cx, &body)
}
