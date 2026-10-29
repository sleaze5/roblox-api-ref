use maud::{Markup, html};
use rapi_core::api::access::identities_allowing;
use rapi_core::api::facets::{
	facet_description, facet_label, use_kind_title, use_section_id, use_sections,
};
use rapi_core::api::tags::plain_tags;
use rapi_core::model::{Facet, Security};
use rapi_routes::FacetKind;

use crate::components::facts::{fact, fact_list};
use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::components::index::{
	IndexCurrent, IndexGroup, facet_icon, facet_use_sections, index_sidebar,
};
use crate::components::page::{HeaderIcon, PageHeader, index_group_crumbs, page_lead};
use crate::components::tags::tag_list;
use crate::context::Context;
use crate::layout::{RailLink, Section, Shell, page_rail};

fn kind_facts(facet: &Facet) -> Markup {
	match facet.kind {
		FacetKind::Tag => fact(
			"Shown as",
			Some("How this tag appears on this site."),
			&tag_list(&plain_tags(std::slice::from_ref(&facet.name)), ""),
		),
		FacetKind::Security => {
			let Some(security) = Security::parse(&facet.name) else {
				return html! {};
			};
			let identities: Vec<String> = identities_allowing(security)
				.iter()
				.map(ToString::to_string)
				.collect();
			fact(
				"Usable by identities",
				Some("Script identities that can use members with this security level."),
				&html! { (identities.join(", ")) },
			)
		}
		_ => html! {},
	}
}

fn facet_facts(facet: &Facet) -> Markup {
	let label = facet_label(facet.kind);
	fact_list(&html! {
		(fact("Kind", Some("The Index group that this page belongs to."), &html! { (label) }))
		(fact("Uses", Some(&format!("API items that have this {}.", label.to_lowercase())), &html! { (facet.uses.len()) }))
		(kind_facts(facet))
	})
}

pub fn facet_page(cx: &Context, facet: &Facet) -> Markup {
	let sections = use_sections(&facet.uses);
	let links: Vec<RailLink> = sections
		.iter()
		.map(|(kind, uses)| {
			RailLink::new(use_section_id(*kind), use_kind_title(*kind)).count(uses.len())
		})
		.collect();
	let header = PageHeader {
		crumbs: index_group_crumbs(Some(facet.kind)),
		icon: HeaderIcon::Named(facet_icon(facet.kind)),
		title: html! { (facet.name) },
		code: true,
		docs: None,
	};
	let body = html! {
		article class="flex flex-col gap-6" {
			(header.render(cx, Some(html! {
				(page_lead(&html! { (facet_description(facet)) }))
				(facet_facts(facet))
			})))
			(list_filter(cx, "Filter uses", "facet-uses", "q", ListFilterPlacement::Page, None))
			(facet_use_sections(cx, "facet-uses", &sections))
		}
	};
	let current = IndexCurrent {
		group: IndexGroup::Facet(facet.kind),
		name: Some(&facet.name),
	};
	Shell::new(Some(Section::Index))
		.sidebar(index_sidebar(cx, Some(&current)))
		.rail(page_rail(&links))
		.render(cx, &body)
}
