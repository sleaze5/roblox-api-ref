use maud::{Markup, html};
use rapi_core::api::stats_text::{
	DISTRIBUTIONS_LEAD, RELEASE_DAYS_LEAD, WEEKDAYS, bucket_label, gap_label,
};
use rapi_core::model::{BUCKET_STARTS, Distribution, GapBucket, WeekdayTrend};

use super::tables::{column, data_table, number, stats_section, text};
use crate::context::Context;
use crate::pages::charts::{Column, column_chart};

fn distribution_chart(distribution: &Distribution) -> Markup {
	let total: u32 = distribution.counts.iter().sum();
	let columns: Vec<Column> = distribution
		.counts
		.iter()
		.enumerate()
		.map(|(index, count)| Column {
			label: bucket_label(index),
			value: f64::from(*count),
			hint: format!(
				"{}: {count} with {}",
				distribution.label,
				bucket_label(index)
			),
		})
		.collect();
	column_chart(&distribution.label, &format!("{total} in all"), &columns)
}

pub fn distributions_section(cx: &Context) -> Markup {
	let distributions = &cx.stats.distributions;
	let mut columns = vec![column("Count")];
	columns.extend(distributions.iter().map(|item| number(&item.label)));
	let rows: Vec<(String, Vec<Markup>)> = (0..BUCKET_STARTS.len())
		.map(|index| {
			let mut cells = vec![text(bucket_label(index))];
			cells.extend(distributions.iter().map(|item| text(item.counts[index])));
			(format!("distribution-{}", BUCKET_STARTS[index]), cells)
		})
		.collect();
	stats_section(
		"distributions",
		"Distributions",
		Some(DISTRIBUTIONS_LEAD),
		&html! {
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				@for distribution in distributions { (distribution_chart(distribution)) }
			}
			(data_table("Distributions", &columns, &rows))
		},
	)
}

fn weekday_charts(days: &[WeekdayTrend]) -> Markup {
	let columns = |value: &dyn Fn(&WeekdayTrend) -> u32| -> Vec<Column> {
		days.iter()
			.map(|day| Column {
				label: WEEKDAYS[day.weekday][..1].to_owned(),
				value: f64::from(value(day)),
				hint: format!(
					"{}: {} updates, {} changes",
					WEEKDAYS[day.weekday], day.updates, day.changes
				),
			})
			.collect()
	};
	let busiest = |value: &dyn Fn(&WeekdayTrend) -> u32| {
		days.iter()
			.max_by_key(|day| (value(day), std::cmp::Reverse(day.weekday)))
			.map_or_else(String::new, |day| {
				format!("Most on {}", WEEKDAYS[day.weekday])
			})
	};
	html! {
		(column_chart("Updates by weekday", &busiest(&|day| day.updates), &columns(&|day| day.updates)))
		(column_chart("Changes by weekday", &busiest(&|day| day.changes), &columns(&|day| day.changes)))
	}
}

fn gap_chart(gaps: &[GapBucket]) -> Markup {
	let columns: Vec<Column> = gaps
		.iter()
		.map(|gap| Column {
			label: gap_label(gap.days),
			value: f64::from(gap.updates),
			hint: format!("{} days: {} updates", gap_label(gap.days), gap.updates),
		})
		.collect();
	let common = gaps
		.iter()
		.max_by_key(|gap| (gap.updates, std::cmp::Reverse(gap.days)))
		.map_or_else(String::new, |gap| {
			format!("Most often {} days", gap_label(gap.days))
		});
	column_chart("Days between updates", &common, &columns)
}

pub fn release_days_section(cx: &Context) -> Markup {
	let stats = &cx.stats;
	let weekday_rows: Vec<(String, Vec<Markup>)> = stats
		.weekdays
		.iter()
		.map(|day| {
			(
				format!("weekday-{}", WEEKDAYS[day.weekday].to_lowercase()),
				vec![
					text(WEEKDAYS[day.weekday]),
					text(day.updates),
					text(day.changes),
				],
			)
		})
		.collect();
	let gap_rows: Vec<(String, Vec<Markup>)> = stats
		.gaps
		.iter()
		.map(|gap| {
			(
				format!("gap-{}", gap.days),
				vec![text(gap_label(gap.days)), text(gap.updates)],
			)
		})
		.collect();
	stats_section(
		"release-days",
		"Release days",
		Some(RELEASE_DAYS_LEAD),
		&html! {
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				(weekday_charts(&stats.weekdays))
			}
			(gap_chart(&stats.gaps))
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				(data_table("Updates by weekday", &[column("Weekday"), number("Updates"), number("Changes")], &weekday_rows))
				(data_table("Days between updates", &[column("Days"), number("Updates")], &gap_rows))
			}
		},
	)
}
