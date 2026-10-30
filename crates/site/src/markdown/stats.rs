use rapi_core::api::access::access_tier_label;
use rapi_core::api::stats_text::{
	CAPABILITIES_LEAD, COMEBACKS_LEAD, DOCS_LEAD, GROWTH_LEAD, MONTHS_NOTE, PER_UPDATE_TABLE_NOTE,
	SERVICES_HINT, TAGS_LEAD, TRENDS_NOTE, access_lead, count_label, coverage_label, month_label,
	per_update_totals, percent_text, removed_note, stats_lead,
};
use rapi_core::js::format_date;
use rapi_core::model::{ChangeTally, RecordGroup, TagTrend, Tallies, UpdateEvent, has_items};
use rapi_routes::update_href;

use super::stat_shapes::{kind_table, record_items, record_value, shape_blocks};
use super::text::{fact_list, inline_code, join_blocks, link, markdown_table};
use super::urls::markdown_url;
use crate::context::Context;

fn update_text(update: Option<&UpdateEvent>) -> String {
	update.map_or_else(
		|| "Not recorded".to_owned(),
		|update| format!("{} ({})", update.version, format_date(&update.published)),
	)
}

fn tally_text(tally: &ChangeTally, changes: bool) -> String {
	let mut parts = Vec::new();
	if tally.added > 0 {
		parts.push(format!("+{}", tally.added));
	}
	if tally.removed > 0 {
		parts.push(format!("−{}", tally.removed));
	}
	if changes && tally.changed > 0 {
		parts.push(format!("~{}", tally.changed));
	}
	if parts.is_empty() {
		"0".to_owned()
	} else {
		parts.join(" ")
	}
}

fn tally_cells(tallies: &Tallies) -> Vec<String> {
	vec![
		tally_text(&tallies.class, false),
		tally_text(&tallies.property, true),
		tally_text(&tallies.method, true),
		tally_text(&tallies.event, true),
		tally_text(&tallies.callback, true),
		tally_text(&tallies.enumeration, false),
		tally_text(&tallies.enum_item, true),
		tallies.deprecated_total().to_string(),
	]
}

const TALLY_HEADINGS: [&str; 8] = [
	"Classes",
	"Properties",
	"Methods",
	"Events",
	"Callbacks",
	"Enums",
	"Enum items",
	"Deprecated",
];

fn record_blocks(cx: &Context, group: &RecordGroup) -> Vec<String> {
	let listed = group.records.iter().any(has_items);
	let rows: Vec<Vec<String>> = group
		.records
		.iter()
		.map(|record| {
			let mut cells = vec![record.label.clone(), record_value(record)];
			if listed {
				cells.push(record_items(record));
			}
			cells
		})
		.collect();
	let mut blocks = vec![format!("## {}", group.title)];
	if let Some(table) = &group.table {
		blocks.push(kind_table(table));
	}
	let headings: &[&str] = if listed {
		&["Statistic", "Value", "Items"]
	} else {
		&["Statistic", "Value"]
	};
	blocks.push(markdown_table(headings, &rows));
	if group.id == "comeback-records" {
		blocks.push(COMEBACKS_LEAD.to_owned());
		let rows: Vec<Vec<String>> = cx
			.stats
			.comebacks
			.iter()
			.map(|batch| {
				vec![
					update_text(Some(&batch.removed)),
					update_text(Some(&batch.returned)),
					batch.count.to_string(),
					batch.days.to_string(),
				]
			})
			.collect();
		blocks.push(markdown_table(
			&["Removed in", "Added back in", "Items", "Days gone"],
			&rows,
		));
	}
	blocks
}

fn tag_table(label: &str, rows: &[TagTrend]) -> String {
	let rows: Vec<Vec<String>> = rows
		.iter()
		.map(|row| {
			vec![
				inline_code(&row.name),
				row.current.to_string(),
				row.gained.to_string(),
				row.lost.to_string(),
				update_text(row.first.as_ref()),
			]
		})
		.collect();
	markdown_table(&[label, "Current", "Gained", "Lost", "First gained"], &rows)
}

fn extra_blocks(cx: &Context) -> Vec<String> {
	let stats = &cx.stats;
	let tiers: Vec<Vec<String>> = stats
		.access
		.tiers
		.iter()
		.map(|row| {
			let kinds = &row.kinds;
			vec![
				access_tier_label(row.tier).to_owned(),
				kinds.property.to_string(),
				kinds.method.to_string(),
				kinds.event.to_string(),
				kinds.callback.to_string(),
				kinds.total().to_string(),
			]
		})
		.collect();
	let access_years: Vec<Vec<String>> = stats
		.access
		.years
		.iter()
		.rev()
		.map(|year| {
			vec![
				year.period.clone(),
				year.added_open.to_string(),
				year.added_restricted.to_string(),
				year.opened.to_string(),
				year.restricted.to_string(),
				year.made_safe.to_string(),
				year.made_unsafe.to_string(),
			]
		})
		.collect();
	let coverage: Vec<Vec<String>> = stats
		.docs
		.coverage
		.iter()
		.map(|row| {
			vec![
				coverage_label(row.kind).to_owned(),
				row.described.to_string(),
				row.total.to_string(),
				percent_text(row.described, row.total),
			]
		})
		.collect();
	let doc_years: Vec<Vec<String>> = stats
		.docs
		.years
		.iter()
		.rev()
		.map(|year| {
			vec![
				year.period.clone(),
				year.added.to_string(),
				year.changed.to_string(),
				year.removed.to_string(),
			]
		})
		.collect();
	let growth: Vec<Vec<String>> = stats
		.growth
		.iter()
		.rev()
		.map(|year| {
			vec![
				year.period.clone(),
				year.classes
					.iter()
					.map(|item| format!("{} +{}", inline_code(&item.name), item.added))
					.collect::<Vec<_>>()
					.join(", "),
			]
		})
		.collect();
	vec![
		"## Access".to_owned(),
		access_lead(stats.access.now),
		markdown_table(
			&[
				"Who can use it",
				"Properties",
				"Methods",
				"Events",
				"Callbacks",
				"All",
			],
			&tiers,
		),
		markdown_table(
			&[
				"Year",
				"Added open",
				"Added restricted",
				"Opened",
				"Restricted",
				"Made safe",
				"Made unsafe",
			],
			&access_years,
		),
		"## Tags".to_owned(),
		TAGS_LEAD.to_owned(),
		tag_table("Tag", &stats.tags),
		"## Capabilities".to_owned(),
		CAPABILITIES_LEAD.to_owned(),
		tag_table("Capability", &stats.capabilities),
		"## Descriptions".to_owned(),
		DOCS_LEAD.to_owned(),
		markdown_table(&["Kind", "Described", "Total", "Share"], &coverage),
		markdown_table(&["Year", "Added", "Changed", "Removed"], &doc_years),
		"## Growth by class".to_owned(),
		GROWTH_LEAD.to_owned(),
		markdown_table(&["Year", "Classes that grew the most"], &growth),
	]
	.into_iter()
	.chain(shape_blocks(cx))
	.collect()
}

fn headings<'a>(first: &[&'a str]) -> Vec<&'a str> {
	first.iter().copied().chain(TALLY_HEADINGS).collect()
}

pub fn stats_markdown(cx: &Context) -> String {
	let stats = &cx.stats;
	let totals = stats.counts.iter().filter(|row| !row.kind.is_subset());
	let current: u32 = totals.clone().map(|row| row.current).sum();
	let removed: u32 = totals.map(|row| row.removed).sum();
	let mut counts: Vec<Vec<String>> = stats
		.counts
		.iter()
		.map(|row| {
			vec![
				count_label(row.kind).to_owned(),
				row.current.to_string(),
				row.removed.to_string(),
				(row.current + row.removed).to_string(),
			]
		})
		.collect();
	counts.push(vec![
		"All".to_owned(),
		current.to_string(),
		removed.to_string(),
		(current + removed).to_string(),
	]);
	let sizes: Vec<Vec<String>> = stats
		.updates
		.iter()
		.rev()
		.map(|trend| {
			vec![
				trend.version.clone(),
				format_date(&trend.published),
				trend.classes.to_string(),
				trend.enums.to_string(),
			]
		})
		.collect();
	let per_update: Vec<Vec<String>> = per_update_totals(&stats.updates)
		.into_iter()
		.map(|row| {
			vec![
				row.label.to_owned(),
				row.added.to_string(),
				row.removed.to_string(),
				row.average_added,
				row.average_removed,
			]
		})
		.collect();
	let years: Vec<Vec<String>> = stats
		.years
		.iter()
		.rev()
		.map(|year| {
			[
				vec![year.period.clone(), year.updates.to_string()],
				tally_cells(&year.tallies),
			]
			.concat()
		})
		.collect();
	let months: Vec<Vec<String>> = stats
		.months
		.iter()
		.map(|month| {
			[
				vec![
					month_label(&month.period),
					month.updates.to_string(),
					month.tallies.change_total().to_string(),
				],
				tally_cells(&month.tallies),
			]
			.concat()
		})
		.collect();
	let updates: Vec<Vec<String>> = stats
		.updates
		.iter()
		.rev()
		.map(|trend| {
			[
				vec![
					link(
						&trend.version,
						&markdown_url(cx, &update_href(&trend.version)),
					),
					format_date(&trend.published),
				],
				tally_cells(&trend.tallies),
			]
			.concat()
		})
		.collect();
	let mut blocks = vec![
		"# Statistics".to_owned(),
		stats_lead(&cx.api.version),
		fact_list(&[
			("Version", Some(cx.api.version.clone())),
			("Tracked since", Some(stats.history_start.clone())),
			("Updates", Some(stats.updates.len().to_string())),
		]),
		"## Counts".to_owned(),
		format!("{} {SERVICES_HINT}", removed_note(&stats.history_start)),
		markdown_table(&["Kind", "Current", "Removed", "Total"], &counts),
	];
	for group in &stats.records {
		blocks.extend(record_blocks(cx, group));
	}
	blocks.extend(extra_blocks(cx));
	blocks.extend([
		"## API size".to_owned(),
		markdown_table(&["Update", "Published", "Classes", "Enums"], &sizes),
		"## Added and removed per update".to_owned(),
		PER_UPDATE_TABLE_NOTE.to_owned(),
		markdown_table(
			&[
				"Kind",
				"Added",
				"Removed",
				"Average added",
				"Average removed",
			],
			&per_update,
		),
		"## Changes by year".to_owned(),
		format!("{TRENDS_NOTE} + means added, − removed, and ~ changed."),
		markdown_table(&headings(&["Year", "Updates"]), &years),
		"## Changes by month".to_owned(),
		MONTHS_NOTE.to_owned(),
		markdown_table(&headings(&["Month", "Updates", "Changes"]), &months),
		"## Changes by update".to_owned(),
		markdown_table(&headings(&["Update", "Published"]), &updates),
	]);
	join_blocks(blocks.into_iter().map(Some))
}
