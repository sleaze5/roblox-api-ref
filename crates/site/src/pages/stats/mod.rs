mod counts;
mod records;
mod sections;
mod shapes;
mod tables;
mod trends;

use maud::{Markup, html};
use rapi_core::api::stats_text::{
	CAPABILITIES_LEAD, COMEBACKS_LEAD, TAGS_LEAD, removed_note, stats_lead,
};
use rapi_core::icons::Icon;
use rapi_routes::FacetKind;

use self::counts::count_table;
use self::records::{comeback_table, kind_table, record_table};
use self::sections::{access_section, docs_section, growth_section, tag_section};
use self::shapes::{distributions_section, release_days_section};
use self::tables::stats_section;
use self::trends::stats_trends;
use crate::components::facts::{fact, fact_list};
use crate::components::page::{HeaderIcon, PageHeader, page_lead, root_crumbs};
use crate::context::Context;
use crate::layout::{RailLink, Section, Shell, page_rail};

const EXTRA_SECTIONS: [(&str, &str); 7] = [
	("access", "Access"),
	("tags", "Tags"),
	("capabilities", "Capabilities"),
	("descriptions", "Descriptions"),
	("growth", "Growth by class"),
	("distributions", "Distributions"),
	("release-days", "Release days"),
];

const TREND_SECTIONS: [(&str, &str); 5] = [
	("api-size", "API size"),
	("members-per-update", "Added and removed"),
	("changes-by-year", "Changes by year"),
	("changes-by-month", "Changes by month"),
	("changes-by-update", "Changes by update"),
];

pub fn stats_page(cx: &Context) -> Markup {
	let stats = &cx.stats;
	let mut links = vec![RailLink::new("counts", "Counts")];
	links.extend(
		stats
			.records
			.iter()
			.map(|group| RailLink::new(&group.id, &group.title)),
	);
	links.extend(
		EXTRA_SECTIONS
			.iter()
			.map(|(id, label)| RailLink::new(*id, *label)),
	);
	links.extend(
		TREND_SECTIONS
			.iter()
			.map(|(id, label)| RailLink::new(*id, *label)),
	);
	let header = PageHeader {
		crumbs: root_crumbs(),
		icon: HeaderIcon::Named(Icon::ChartBar),
		title: html! { "Statistics" },
		code: false,
		docs: None,
	};
	let body = html! {
		article class="flex flex-col gap-10" {
			(header.render(cx, Some(html! {
				(page_lead(&html! { (stats_lead(&cx.api.version)) }))
				(fact_list(&html! {
					(fact("Version", Some("The Roblox Studio version that the current numbers describe."), &html! { span class="font-mono" { (cx.api.version) } }))
					(fact("Tracked since", Some("The oldest version in the update history."), &html! { span class="font-mono" { (stats.history_start) } }))
					(fact("Updates", Some("Updates in the tracked history."), &html! { (stats.updates.len()) }))
				}))
			})))
			(stats_section("counts", "Counts", Some(&removed_note(&stats.history_start)), &count_table(cx, &stats.counts, &stats.history_start)))
			@for group in &stats.records {
				@let comebacks = group.id == "comeback-records";
				(stats_section(&group.id, &group.title, comebacks.then_some(COMEBACKS_LEAD), &html! {
					@if let Some(table) = &group.table { (kind_table(&group.title, table)) }
					(record_table(&group.records))
					@if comebacks { (comeback_table(cx)) }
				}))
			}
			(access_section(cx))
			(tag_section(cx, "tags", "Tags", TAGS_LEAD, FacetKind::Tag, "Tag", &stats.tags))
			(tag_section(cx, "capabilities", "Capabilities", CAPABILITIES_LEAD, FacetKind::Capability, "Capability", &stats.capabilities))
			(docs_section(cx))
			(growth_section(cx))
			(distributions_section(cx))
			(release_days_section(cx))
			(stats_trends(cx))
		}
	};
	Shell::new(Some(Section::Stats))
		.rail(page_rail(&links))
		.render(cx, &body)
}
