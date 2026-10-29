use maud::{Markup, html};
use rapi_core::api::stat_ref::stat_ref_tokens;
use rapi_core::api::stats_text::{count_label, ref_noun, unit_text};
use rapi_core::js::js_number;
use rapi_core::model::{KindTable, StatRecord, StatRef, has_items};

use super::tables::{ROW_HINT_CLASS, column, data_table, number, text, update_cell};
use crate::components::code::code_tokens;
use crate::context::Context;

pub const SHOWN_ITEMS: usize = 8;

pub fn item_list(items: &[StatRef]) -> Markup {
	html! {
		ul class="flex flex-wrap gap-x-4 gap-y-1" {
			@for item in items { li class="min-w-0 break-all" { code { (code_tokens(&stat_ref_tokens(item))) } } }
		}
	}
}

pub fn record_table(records: &[StatRecord]) -> Markup {
	let listed = records.iter().any(has_items);
	html! {
		div class="stat-table" {
			table data-records=[listed.then_some("")] {
				@if listed { colgroup { col data-label=""; col data-value=""; col; } }
				thead {
					tr {
						th scope="col" { "Statistic" }
						th scope="col" data-number="" { "Value" }
						@if listed { th scope="col" { "Items" } }
					}
				}
				tbody {
					@for record in records {
						tr id=(record.id) {
							th scope="row" data-wrap="" { span data-tooltip=(record.hint) class=(ROW_HINT_CLASS) { (record.label) } }
							td data-number="" {
								(js_number(record.value))
								@if let Some(unit) = &record.unit { span class="text-fg-faint" { " " (unit_text(record.value, unit)) } }
							}
							@if listed { td data-wrap="" {
								@if let Some(ties) = &record.ties {
									span class="text-fg-muted" { (ties.count) " " (ref_noun(&ties.kind)) }
								} @else if let Some(first) = record.items.first() {
									@if record.items.len() <= SHOWN_ITEMS {
										(item_list(&record.items))
									} @else {
										details class="change-previous" {
											summary { (record.items.len()) " " (ref_noun(first.kind_name())) }
											(item_list(&record.items))
										}
									}
								}
							} }
						}
					}
				}
			}
		}
	}
}

fn kind_cell(value: Option<f64>, unit: Option<&str>) -> Markup {
	html! {
		@match value {
			None => span class="text-fg-faint" { "–" },
			Some(value) => {
				(js_number(value))
				@if let Some(unit) = unit { span class="text-fg-faint" { " " (unit_text(value, unit)) } }
			},
		}
	}
}

pub fn kind_table(caption: &str, table: &KindTable) -> Markup {
	let mut columns = vec![column("Statistic")];
	columns.extend(table.kinds.iter().map(|kind| number(count_label(*kind))));
	let rows: Vec<(String, Vec<Markup>)> = table
		.rows
		.iter()
		.map(|row| {
			let mut cells =
				vec![html! { span data-tooltip=(row.hint) class=(ROW_HINT_CLASS) { (row.label) } }];
			cells.extend(
				row.values
					.iter()
					.map(|value| kind_cell(*value, row.unit.as_deref())),
			);
			(row.id.clone(), cells)
		})
		.collect();
	data_table(caption, &columns, &rows)
}

pub fn comeback_table(cx: &Context) -> Markup {
	let rows: Vec<(String, Vec<Markup>)> = cx
		.stats
		.comebacks
		.iter()
		.map(|batch| {
			(
				format!(
					"comeback-{}-{}",
					batch.removed.version, batch.returned.version
				),
				vec![
					update_cell(Some(&batch.removed)),
					update_cell(Some(&batch.returned)),
					text(batch.count),
					text(batch.days),
				],
			)
		})
		.collect();
	data_table(
		"Comebacks",
		&[
			column("Removed in"),
			column("Added back in"),
			number("Items"),
			number("Days gone"),
		],
		&rows,
	)
}
