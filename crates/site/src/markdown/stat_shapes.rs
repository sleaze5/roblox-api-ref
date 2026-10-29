use rapi_core::api::stat_ref::stat_ref_label;
use rapi_core::api::stats_text::{
	DISTRIBUTIONS_LEAD, RELEASE_DAYS_LEAD, WEEKDAYS, bucket_label, count_label, gap_label,
	ref_noun, unit_text,
};
use rapi_core::js::js_number;
use rapi_core::model::{BUCKET_STARTS, KindTable, StatRecord};

use super::text::{inline_code, markdown_table};
use crate::context::Context;

fn with_unit(value: f64, unit: Option<&str>) -> String {
	let text = js_number(value);
	unit.map_or_else(
		|| text.clone(),
		|unit| format!("{text} {}", unit_text(value, unit)),
	)
}

pub fn record_items(record: &StatRecord) -> String {
	match &record.ties {
		Some(ties) => format!("{} {}", ties.count, ref_noun(&ties.kind)),
		None => record
			.items
			.iter()
			.map(|item| inline_code(&stat_ref_label(item)))
			.collect::<Vec<_>>()
			.join(", "),
	}
}

pub fn record_value(record: &StatRecord) -> String {
	with_unit(record.value, record.unit.as_deref())
}

pub fn kind_table(table: &KindTable) -> String {
	let mut headings = vec!["Statistic"];
	headings.extend(table.kinds.iter().map(|kind| count_label(*kind)));
	let rows: Vec<Vec<String>> = table
		.rows
		.iter()
		.map(|row| {
			let mut cells = vec![row.label.clone()];
			cells.extend(row.values.iter().map(|value| {
				value.map_or_else(
					|| "–".to_owned(),
					|value| with_unit(value, row.unit.as_deref()),
				)
			}));
			cells
		})
		.collect();
	markdown_table(&headings, &rows)
}

pub fn shape_blocks(cx: &Context) -> Vec<String> {
	let stats = &cx.stats;
	let mut headings = vec!["Count"];
	headings.extend(stats.distributions.iter().map(|item| item.label.as_str()));
	let distributions: Vec<Vec<String>> = (0..BUCKET_STARTS.len())
		.map(|index| {
			let mut cells = vec![bucket_label(index)];
			cells.extend(
				stats
					.distributions
					.iter()
					.map(|item| item.counts[index].to_string()),
			);
			cells
		})
		.collect();
	let weekdays: Vec<Vec<String>> = stats
		.weekdays
		.iter()
		.map(|day| {
			vec![
				WEEKDAYS[day.weekday].to_owned(),
				day.updates.to_string(),
				day.changes.to_string(),
			]
		})
		.collect();
	let gaps: Vec<Vec<String>> = stats
		.gaps
		.iter()
		.map(|gap| vec![gap_label(gap.days), gap.updates.to_string()])
		.collect();
	vec![
		"## Distributions".to_owned(),
		DISTRIBUTIONS_LEAD.to_owned(),
		markdown_table(&headings, &distributions),
		"## Release days".to_owned(),
		RELEASE_DAYS_LEAD.to_owned(),
		markdown_table(&["Weekday", "Updates", "Changes"], &weekdays),
		markdown_table(&["Days between updates", "Updates"], &gaps),
	]
}
