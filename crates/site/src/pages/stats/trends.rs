use maud::{Markup, html};
use rapi_core::api::stats_text::{
	MONTHS_NOTE, PER_UPDATE_KINDS, PER_UPDATE_NOTE, TRENDS_NOTE, kind_tally, month_label,
	per_update_totals,
};
use rapi_core::js::{format_date, js_number, js_round};
use rapi_core::model::{
	ChangeCounts, ChangeStatus, ChangeTally, PeriodTrend, Tallies, TrendKind, UpdateTrend,
};
use rapi_routes::update_href;

use super::tables::{column, data_table, number, stats_section, text};
use crate::components::changes::{change_counts, status_count, status_count_or_zero};
use crate::context::Context;
use crate::pages::charts::{
	BarColumn, Column, LinePoint, column_chart, diverging_bar_chart, line_chart, year_ticks,
};

pub fn tally_counts(tally: &ChangeTally) -> ChangeCounts {
	ChangeCounts {
		added: tally.added,
		removed: tally.removed,
		changed: tally.changed,
	}
}

pub fn tally_cell(tally: &ChangeTally, changes: bool) -> Markup {
	let empty = tally.added == 0 && tally.removed == 0 && (!changes || tally.changed == 0);
	html! {
		td {
			@if empty {
				span class="text-fg-faint" { "0" }
			} @else {
				(change_counts(&tally_counts(tally), changes))
			}
		}
	}
}

pub fn trend_table(
	caption: &str,
	heading_label: &str,
	detail_label: &str,
	rows: &[(String, Markup, Markup, &Tallies)],
) -> Markup {
	html! {
		div class="stat-table" {
			table {
				caption class="sr-only" { (caption) }
				thead {
					tr {
						th scope="col" { (heading_label) }
						th scope="col" { (detail_label) }
						th scope="col" { "Classes" }
						th scope="col" { "Properties" }
						th scope="col" { "Methods" }
						th scope="col" { "Events" }
						th scope="col" { "Callbacks" }
						th scope="col" { "Enums" }
						th scope="col" { "Enum items" }
						th scope="col" data-number="" { "Deprecated" }
					}
				}
				tbody {
					@for (key, heading, detail, tallies) in rows {
						@let deprecated = tallies.deprecated_total();
						tr id=(key) {
							th scope="row" { (heading) }
							td { (detail) }
							(tally_cell(&tallies.class, false))
							(tally_cell(&tallies.property, true))
							(tally_cell(&tallies.method, true))
							(tally_cell(&tallies.event, true))
							(tally_cell(&tallies.callback, true))
							(tally_cell(&tallies.enumeration, false))
							(tally_cell(&tallies.enum_item, true))
							td data-number="" class=(if deprecated == 0 { "text-fg-faint" } else { "text-warning" }) { (deprecated) }
						}
					}
				}
			}
		}
	}
}

pub fn point_hint(trend: &UpdateTrend, value: u32, noun: &str) -> String {
	format!(
		"{} ({}): {value} {noun}",
		trend.version,
		format_date(&trend.published)
	)
}

pub fn size_chart(updates: &[UpdateTrend], classes: bool) -> Markup {
	let (Some(first), Some(last)) = (updates.first(), updates.last()) else {
		return html! {};
	};
	let value = |trend: &UpdateTrend| if classes { trend.classes } else { trend.enums };
	let noun = if classes { "classes" } else { "enums" };
	let points: Vec<LinePoint> = updates
		.iter()
		.map(|trend| LinePoint {
			value: f64::from(value(trend)),
			hint: point_hint(trend, value(trend), noun),
		})
		.collect();
	line_chart(
		if classes { "Classes" } else { "Enums" },
		&format!("{} to {}", value(first), value(last)),
		&points,
		&year_ticks(updates.iter().map(|trend| trend.published.as_str())),
	)
}

pub fn list_text(labels: &[String]) -> String {
	match labels {
		[] => String::new(),
		[only] => only.clone(),
		[first, second] => format!("{first} and {second}"),
		[rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
	}
}

pub fn extremes_text(
	periods: &[PeriodTrend],
	label: &dyn Fn(&PeriodTrend) -> String,
	value: impl Fn(&PeriodTrend) -> u32,
) -> String {
	let values: Vec<u32> = periods.iter().map(&value).collect();
	let labels_of = |target: Option<u32>| {
		list_text(
			&periods
				.iter()
				.filter(|period| Some(value(period)) == target)
				.map(label)
				.collect::<Vec<_>>(),
		)
	};
	format!(
		"Most in {}, fewest in {}",
		labels_of(values.iter().copied().max()),
		labels_of(values.iter().copied().min())
	)
}

pub fn period_hint(period: &PeriodTrend, label: &dyn Fn(&PeriodTrend) -> String) -> String {
	let changes = period.tallies.change_total();
	format!(
		"{}: {changes} changes in {} updates, {} per update",
		label(period),
		period.updates,
		js_number(js_round(f64::from(changes) / f64::from(period.updates)))
	)
}

pub fn first_letter(label: &str) -> String {
	label.chars().next().map(String::from).unwrap_or_default()
}

pub fn month_trends(months: &[PeriodTrend]) -> Markup {
	let rows: Vec<(String, Markup, Markup, &Tallies)> = months
		.iter()
		.map(|month| {
			(
				format!("month-{}", month.period),
				text(month_label(&month.period)),
				text(month.updates),
				&month.tallies,
			)
		})
		.collect();
	stats_section(
		"changes-by-month",
		"Changes by month",
		Some(MONTHS_NOTE),
		&html! {
			(period_charts(months, &|month| month_label(&month.period), &|month| first_letter(&month_label(&month.period))))
			(trend_table("Changes by month", "Month", "Updates", &rows))
		},
	)
}

fn added_removed_chart(title: &str, updates: &[UpdateTrend], kind: Option<TrendKind>) -> Markup {
	let tally = |tallies: &Tallies| kind_tally(tallies, kind);
	let added: u32 = updates
		.iter()
		.map(|trend| tally(&trend.tallies).added)
		.sum();
	let removed: u32 = updates
		.iter()
		.map(|trend| tally(&trend.tallies).removed)
		.sum();
	let bars: Vec<BarColumn> = updates
		.iter()
		.map(|trend| {
			let tally = tally(&trend.tallies);
			BarColumn {
				up: f64::from(tally.added),
				down: f64::from(tally.removed),
				hint: format!(
					"{} ({}): {} added, {} removed",
					trend.version,
					format_date(&trend.published),
					tally.added,
					tally.removed
				),
			}
		})
		.collect();
	diverging_bar_chart(
		title,
		&format!("{added} added, {removed} removed"),
		("Added", "Removed"),
		&bars,
		&year_ticks(updates.iter().map(|trend| trend.published.as_str())),
	)
}

fn average_cell(status: ChangeStatus, average: &str) -> Markup {
	if average == "0" {
		html! { span class="text-fg-faint" { "0" } }
	} else {
		status_count(status, average)
	}
}

fn per_update_table(updates: &[UpdateTrend]) -> Markup {
	let rows: Vec<(String, Vec<Markup>)> = per_update_totals(updates)
		.into_iter()
		.map(|row| {
			(
				format!("per-update-{}", row.label.to_lowercase().replace(' ', "-")),
				vec![
					text(row.label),
					status_count_or_zero(ChangeStatus::Added, row.added),
					status_count_or_zero(ChangeStatus::Removed, row.removed),
					average_cell(ChangeStatus::Added, &row.average_added),
					average_cell(ChangeStatus::Removed, &row.average_removed),
				],
			)
		})
		.collect();
	data_table(
		"Added and removed per update",
		&[
			column("Kind"),
			number("Added"),
			number("Removed"),
			number("Average added"),
			number("Average removed"),
		],
		&rows,
	)
}

fn period_charts(
	periods: &[PeriodTrend],
	label: &dyn Fn(&PeriodTrend) -> String,
	column_label: &dyn Fn(&PeriodTrend) -> String,
) -> Markup {
	let columns = |value: &dyn Fn(&PeriodTrend) -> u32| -> Vec<Column> {
		periods
			.iter()
			.map(|period| Column {
				label: column_label(period),
				value: f64::from(value(period)),
				hint: period_hint(period, label),
			})
			.collect()
	};
	let changes = |period: &PeriodTrend| period.tallies.change_total();
	let updates = |period: &PeriodTrend| period.updates;
	html! {
		div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
			(column_chart("Changes", &extremes_text(periods, label, changes), &columns(&changes)))
			(column_chart("Updates", &extremes_text(periods, label, updates), &columns(&updates)))
		}
	}
}

pub fn stats_trends(cx: &Context) -> Markup {
	let stats = &cx.stats;
	let updates = &stats.updates;
	let year_rows: Vec<(String, Markup, Markup, &Tallies)> = stats
		.years
		.iter()
		.rev()
		.map(|year| {
			(
				format!("year-{}", year.period),
				text(&year.period),
				text(year.updates),
				&year.tallies,
			)
		})
		.collect();
	let update_rows: Vec<(String, Markup, Markup, &Tallies)> = updates
		.iter()
		.rev()
		.map(|trend| {
			(
				format!("update-{}", trend.version),
				html! { a href=(update_href(&trend.version)) class="font-mono font-normal hover:text-fg-bright hover:underline" { (trend.version) } },
				text(format_date(&trend.published)),
				&trend.tallies,
			)
		})
		.collect();
	let year_label = |period: &PeriodTrend| period.period.clone();
	html! {
		(stats_section("api-size", "API size", Some("The number of classes and enums after each update."), &html! {
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				(size_chart(updates, true))
				(size_chart(updates, false))
			}
		}))
		(stats_section("members-per-update", "Added and removed per update", Some(PER_UPDATE_NOTE), &html! {
			@let (title, kind) = PER_UPDATE_KINDS[0];
			(added_removed_chart(title, updates, kind))
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				@for (title, kind) in &PER_UPDATE_KINDS[1..] {
					(added_removed_chart(title, updates, *kind))
				}
			}
			(per_update_table(updates))
		}))
		(stats_section("changes-by-year", "Changes by year", Some(TRENDS_NOTE), &html! {
			(period_charts(&stats.years, &year_label, &year_label))
			(trend_table("Changes by year", "Year", "Updates", &year_rows))
		}))
		(month_trends(&stats.months))
		(stats_section("changes-by-update", "Changes by update", Some("Every tracked update, newest first."), &trend_table("Changes by update", "Update", "Published", &update_rows)))
	}
}
