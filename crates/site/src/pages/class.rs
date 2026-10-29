use std::collections::HashSet;

use maud::{Markup, html};
use rapi_core::api::access::{access_tier_hint, access_tier_label, tier_counts};
use rapi_core::api::docs_links::class_docs_href;
use rapi_core::api::filter_tags::{filter_tag_groups, with_all_tags};
use rapi_core::api::history_text::HistoryNoun;
use rapi_core::api::members::{member_kind_title, members_in_page_order};
use rapi_core::api::tags::plain_tags;
use rapi_core::icons::Icon;
use rapi_core::model::{AccessTier, Class, Member};
use rapi_routes::{FacetKind, MemberKind, OwnerKind, member_anchor, owner_anchor};

use crate::components::changes::{ChangedValue, RemovedList, removed_items};
use crate::components::doc::doc_text;
use crate::components::facts::{fact, fact_list};
use crate::components::history::{history_facts, history_list};
use crate::components::members::{member_filter_bar, member_section, section_id};
use crate::components::page::{HeaderIcon, PageHeader, class_crumbs};
use crate::components::tags::{dated_tags, tag_list};
use crate::components::tree::class_tree;
use crate::components::usage::{ROLE_SECTIONS, usage_count, usage_sections};
use crate::context::Context;
use crate::layout::{RailEntry, RailLink, Section, Shell, page_rail, section_sidebar};

fn count_of_kind(classes: &[&Class], kind: MemberKind) -> usize {
	classes
		.iter()
		.map(|item| {
			item.members
				.iter()
				.filter(|member| member.kind() == kind)
				.count()
		})
		.sum()
}

fn sibling_count(cx: &Context, item: &Class) -> usize {
	match &item.superclass {
		None => cx
			.api
			.classes
			.values()
			.filter(|other| other.superclass.is_none())
			.count(),
		Some(superclass) => cx
			.api
			.classes
			.get(superclass)
			.map_or(1, |parent| parent.subclasses.len()),
	}
}

pub fn class_sidebar(cx: &Context, item: Option<&Class>, ancestors: &[&Class]) -> Markup {
	let open: HashSet<&str> = item
		.into_iter()
		.chain(ancestors.iter().copied())
		.map(|item| item.name.as_str())
		.collect();
	section_sidebar(
		cx,
		"Filter classes",
		"sidebar-tree",
		&class_tree(
			cx,
			"sidebar-tree",
			item.map(|item| item.name.as_str()),
			&open,
			false,
			true,
		),
	)
}

fn access_facts(members: &[Member]) -> Markup {
	let counts = tier_counts(members);
	html! {
		@if !members.is_empty() {
			(fact_list(&html! {
				@for tier in AccessTier::ALL {
					(fact(
						access_tier_label(tier),
						Some(&format!("{} Counts the members this class defines itself.", access_tier_hint(tier))),
						&html! { (counts[tier as usize]) },
					))
				}
			}))
		}
	}
}

fn class_header(cx: &Context, item: &Class, ancestors: &[&Class]) -> Markup {
	let path: Vec<&str> = ancestors
		.iter()
		.rev()
		.map(|ancestor| ancestor.name.as_str())
		.collect();
	let inherited: usize = ancestors
		.iter()
		.map(|ancestor| ancestor.members.len())
		.sum();
	let header = PageHeader {
		crumbs: class_crumbs(path),
		icon: item
			.icon
			.map_or(HeaderIcon::Named(Icon::Cube), HeaderIcon::Class),
		title: html! { (item.name) },
		code: true,
		docs: Some((class_docs_href(&item.name, None), &item.name)),
	};
	let siblings_hint = if item.superclass.is_none() {
		"Classes without a superclass, including this class."
	} else {
		"Classes with the same superclass, including this class."
	};
	header.render(cx, Some(html! {
		(doc_text(item.description.as_ref(), "max-w-prose text-base text-fg"))
		(tag_list(&dated_tags(plain_tags(&item.tags), item.history.as_ref()), ""))
		(fact_list(&html! {
			(fact("Memory category", Some("The memory category that the API dump assigns to this class."), &html! {
				a href=(cx.facet_href(FacetKind::MemoryCategory, &item.memory_category)) class="font-mono text-fg underline decoration-line-strong hover:decoration-fg" { (item.memory_category) }
			}))
			(fact("Own members", Some("Members that this class defines itself."), &html! { (item.members.len()) }))
			(fact("Inherited members", Some("Members that this class inherits from its superclasses."), &html! { (inherited) }))
			(fact("Subclasses", Some("Classes that inherit directly from this class."), &html! { (item.subclasses.len()) }))
			(fact("Siblings", Some(siblings_hint), &html! { (sibling_count(cx, item)) }))
			(fact("Uses", Some("Properties, parameters, and return values that use this class as their type."), &html! { (item.used_by.len()) }))
		}))
		(access_facts(&item.members))
		(fact_list(&history_facts(item.history.as_ref(), OwnerKind::Class, &item.name, cx.api.history_start.as_deref())))
		(history_list(item.history.as_ref(), &owner_anchor(OwnerKind::Class, &item.name), HistoryNoun::Member, ""))
	}))
}

pub fn class_page(cx: &Context, item: &Class) -> Markup {
	let ancestors = cx.api.ancestors_of(item);
	let mut lineage = vec![item];
	lineage.extend(ancestors.iter().copied());
	let kinds: Vec<MemberKind> = MemberKind::ALL
		.into_iter()
		.filter(|kind| count_of_kind(&lineage, *kind) > 0)
		.collect();
	let mut links: Vec<RailLink> = kinds
		.iter()
		.map(|kind| {
			let title = member_kind_title(*kind);
			RailLink::new(section_id(*kind), title)
				.count(count_of_kind(&[item], *kind))
				.count_hint(format!("{title} that this class defines itself."))
				.entries(
					members_in_page_order(&item.members, *kind)
						.into_iter()
						.map(|member| RailEntry {
							id: member_anchor(*kind, &member.name),
							label: member.name.clone(),
						})
						.collect(),
				)
		})
		.collect();
	for section in &ROLE_SECTIONS {
		let count = usage_count(&item.used_by, section.role);
		if count > 0 {
			links.push(RailLink::new(section.id, section.title).count(count));
		}
	}
	if let Some(removed) = &item.removed_members {
		links.push(RailLink::new("removed-members", "Removed members").count(removed.len()));
	}
	let body = html! {
		article class="flex flex-col gap-6" {
			(class_header(cx, item, &ancestors))
			@if kinds.is_empty() {
				p class="text-fg-muted" { "This class has no members." }
			} @else {
				(member_filter_bar(cx, &kinds, &with_all_tags(&filter_tag_groups(lineage.iter().flat_map(|item| item.members.iter())), cx.all_filter_tags())))
				@for kind in &kinds { (member_section(cx, *kind, item, &ancestors)) }
			}
			@if !item.used_by.is_empty() {
				(usage_sections(cx, &item.used_by, &item.name))
			}
			(removed_items(
				cx,
				&RemovedList { id: "removed-members", title: "Removed members", kind: OwnerKind::Class, owner: &item.name },
				item.removed_members.as_deref(),
				ChangedValue::Member,
			))
		}
	};
	Shell::new(Some(Section::Classes))
		.sidebar(class_sidebar(cx, Some(item), &ancestors))
		.rail(page_rail(&links))
		.render(cx, &body)
}
