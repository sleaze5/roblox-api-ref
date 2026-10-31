use maud::{Markup, html};
use rapi_core::api::access::{access_tier_hint, access_tier_label};
use rapi_core::api::stats_text::{
	DOCS_LEAD, GROWTH_LEAD, access_lead, coverage_label, percent_text,
};
use rapi_core::model::{AccessTier, ChangeStatus, CoverageKind, TagTrend};
use rapi_routes::{FacetKind, OwnerKind};

use super::tables::{
	ROW_HINT_CLASS, column, data_table, hinted, number, stats_section, text, update_cell,
};
use crate::components::changes::{inline_mark, status_count};
use crate::context::Context;
use crate::pages::charts::{Share, share_chart};

pub const fn tier_name(tier: AccessTier) -> &'static str {
	match tier {
		AccessTier::Game => "game",
		AccessTier::Plugin => "plugin",
		AccessTier::Command => "command",
		AccessTier::Roblox => "roblox",
	}
}

pub const fn coverage_name(kind: CoverageKind) -> &'static str {
	match kind {
		CoverageKind::Class => "Class",
		CoverageKind::Property => "Property",
		CoverageKind::Method => "Method",
		CoverageKind::Event => "Event",
		CoverageKind::Callback => "Callback",
		CoverageKind::Parameter => "Parameter",
		CoverageKind::Return => "Return",
		CoverageKind::Enum => "Enum",
		CoverageKind::EnumItem => "EnumItem",
	}
}

pub fn access_section(cx: &Context) -> Markup {
	let access = &cx.stats.access;
	let tiers: Vec<(String, Vec<Markup>)> = access
		.tiers
		.iter()
		.map(|row| {
			let kinds = &row.kinds;
			(
				format!("access-{}", tier_name(row.tier)),
				vec![
					html! { span data-tooltip=(access_tier_hint(row.tier)) class=(ROW_HINT_CLASS) { (access_tier_label(row.tier)) } },
					text(kinds.property),
					text(kinds.method),
					text(kinds.event),
					text(kinds.callback),
					text(kinds.total()),
				],
			)
		})
		.collect();
	let years: Vec<(String, Vec<Markup>)> = access
		.years
		.iter()
		.rev()
		.map(|year| {
			(
				format!("access-{}", year.period),
				vec![
					text(&year.period),
					text(year.added_open),
					text(year.added_restricted),
					text(year.opened),
					text(year.restricted),
					text(year.made_safe),
					text(year.made_unsafe),
				],
			)
		})
		.collect();
	stats_section(
		"access",
		"Access",
		Some(&access_lead(access.now)),
		&html! {
			(share_chart(
				"Members by audience",
				&format!("{} of {} members", access.now.scriptable, access.now.members),
				&access.tiers.iter().map(|row| Share {
					label: access_tier_label(row.tier).to_owned(),
					part: f64::from(row.kinds.total()),
					total: f64::from(access.now.members),
				}).collect::<Vec<_>>(),
			))
			(data_table("Access by audience", &[
				column("Who can use it"),
				number("Properties"),
				number("Methods"),
				number("Events"),
				number("Callbacks"),
				number("All"),
			], &tiers))
			(data_table("Access by year", &[
				column("Year"),
				hinted("Added open", "Members added that game scripts can use.", true),
				hinted("Added restricted", "Members added that need plugin, Roblox, or another security level.", true),
				hinted("Opened", "Members whose security changed so that game scripts can use them.", true),
				hinted("Restricted", "Members whose security changed so that game scripts can no longer use them.", true),
				hinted("Made safe", "Members whose thread safety changed from Unsafe to ReadSafe or Safe.", true),
				hinted("Made unsafe", "Members whose thread safety changed to Unsafe.", true),
			], &years))
		},
	)
}

pub fn tag_section(
	cx: &Context,
	id: &str,
	title: &str,
	lead: &str,
	kind: FacetKind,
	label: &str,
	rows: &[TagTrend],
) -> Markup {
	let rows: Vec<(String, Vec<Markup>)> = rows
		.iter()
		.map(|row| {
			let name = if cx.api.facets.of(kind).contains_key(&row.name) {
				html! { a href=(cx.facet_href(kind, &row.name)) class="font-mono font-normal text-fg hover:text-fg-bright hover:underline" { (row.name) } }
			} else {
				html! { span class="font-mono font-normal text-fg-muted" { (row.name) } }
			};
			(
				format!("{id}-{}", row.name),
				vec![name, text(row.current), text(row.gained), text(row.lost), update_cell(row.first.as_ref())],
			)
		})
		.collect();
	stats_section(
		id,
		title,
		Some(lead),
		&data_table(
			title,
			&[
				column(label),
				hinted("Current", "Items that have it now.", true),
				hinted(
					"Gained",
					"Times an update added an item with it or gave it to an item.",
					true,
				),
				hinted(
					"Lost",
					"Times an update removed an item with it or took it from an item.",
					true,
				),
				hinted(
					"First gained",
					"The first tracked update that gave it to an item.",
					false,
				),
			],
			&rows,
		),
	)
}

pub fn docs_section(cx: &Context) -> Markup {
	let docs = &cx.stats.docs;
	let described: u32 = docs.coverage.iter().map(|row| row.described).sum();
	let total: u32 = docs.coverage.iter().map(|row| row.total).sum();
	let coverage: Vec<(String, Vec<Markup>)> = docs
		.coverage
		.iter()
		.map(|row| {
			(
				format!("coverage-{}", coverage_name(row.kind)),
				vec![
					text(coverage_label(row.kind)),
					text(row.described),
					text(row.total),
					text(percent_text(row.described, row.total)),
				],
			)
		})
		.collect();
	let years: Vec<(String, Vec<Markup>)> = docs
		.years
		.iter()
		.rev()
		.map(|year| {
			(
				format!("descriptions-{}", year.period),
				vec![
					text(&year.period),
					text(year.added),
					text(year.changed),
					text(year.removed),
				],
			)
		})
		.collect();
	stats_section(
		"descriptions",
		"Descriptions",
		Some(DOCS_LEAD),
		&html! {
			(share_chart(
				"Described",
				&format!("{} of the current items", percent_text(described, total)),
				&docs.coverage.iter().map(|row| Share {
					label: coverage_label(row.kind).to_owned(),
					part: f64::from(row.described),
					total: f64::from(row.total),
				}).collect::<Vec<_>>(),
			))
			(data_table("Description coverage", &[column("Kind"), number("Described"), number("Total"), number("Share")], &coverage))
			(data_table("Description changes by year", &[
				column("Year"),
				hinted("Added", "Descriptions written where there was none.", true),
				hinted("Changed", "Descriptions rewritten.", true),
				hinted("Removed", "Descriptions taken away.", true),
			], &years))
		},
	)
}

pub fn growth_section(cx: &Context) -> Markup {
	let rows: Vec<(String, Vec<Markup>)> = cx
		.stats
		.growth
		.iter()
		.rev()
		.map(|year| {
			(
				format!("growth-{}", year.period),
				vec![
					text(&year.period),
					html! {
						ul class="flex flex-wrap items-baseline gap-x-4 gap-y-1 whitespace-normal" {
							@for item in &year.classes {
								li class="flex items-baseline gap-1" {
									@if cx.is_removed_owner(OwnerKind::Class, &item.name) {
										(inline_mark(ChangeStatus::Removed))
									}
									code {
										@match cx.owner_href(OwnerKind::Class, &item.name) {
											Some(href) if cx.api.classes.contains_key(&item.name) => a href=(href) class="text-code-type hover:underline" { (item.name) },
											Some(href) => a href=(href) class="text-fg-muted hover:underline" { (item.name) },
											None => span class="text-fg-muted" { (item.name) },
										}
									}
									(status_count(ChangeStatus::Added, item.added))
								}
							}
						}
					},
				],
			)
		})
		.collect();
	stats_section(
		"growth",
		"Growth by class",
		Some(GROWTH_LEAD),
		&data_table(
			"Growth by class",
			&[column("Year"), column("Classes that grew the most")],
			&rows,
		),
	)
}
