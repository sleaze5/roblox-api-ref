use maud::{Markup, html};
use rapi_core::api::access::{access_tier_hint, access_tier_label};
use rapi_core::api::stat_ref::stat_ref_tokens;
use rapi_core::api::stats_text::{
	CAPABILITIES_LEAD, COMEBACKS_LEAD, DOCS_LEAD, GROWTH_LEAD, MONTHS_NOTE, TAGS_LEAD, TRENDS_NOTE,
	access_lead, count_label, coverage_label, month_label, percent_text, ref_noun, removed_note,
	stats_lead,
};
use rapi_core::icons::Icon;
use rapi_core::js::{format_date, js_number, js_round};
use rapi_core::model::{
	AccessTier, ChangeCounts, ChangeTally, CountKind, CountRow, CoverageKind, PeriodTrend,
	StatRecord, StatRef, TagTrend, Tallies, UpdateEvent, UpdateTrend,
};
use rapi_routes::{FacetKind, class_href, update_href};

use crate::components::code::code_tokens;
use crate::components::facts::{fact, fact_list};
use crate::components::page::{HeaderIcon, PageHeader, page_lead, root_crumbs};
use crate::context::Context;
use crate::html::{IconSize, icon};
use crate::layout::{RailLink, Section, Shell, page_rail};
use crate::pages::charts::{
	BarColumn, Column, LinePoint, column_chart, diverging_bar_chart, line_chart, year_ticks,
};
use crate::pages::updates::change_counts;

const EXTRA_SECTIONS: [(&str, &str); 5] = [
	("access", "Access"),
	("tags", "Tags"),
	("capabilities", "Capabilities"),
	("descriptions", "Descriptions"),
	("growth", "Growth by class"),
];

const TREND_SECTIONS: [(&str, &str); 5] = [
	("api-size", "API size"),
	("members-per-update", "Members per update"),
	("changes-by-year", "Changes by year"),
	("changes-by-month", "Changes by month"),
	("changes-by-update", "Changes by update"),
];

const HINT_CLASS: &str =
	"cursor-help underline decoration-line-strong decoration-dotted underline-offset-4";
const ROW_HINT_CLASS: &str =
	"cursor-help font-normal underline decoration-line-strong decoration-dotted underline-offset-4";

fn stats_section(id: &str, title: &str, lead: Option<&str>, children: &Markup) -> Markup {
	html! {
		section id=(id) aria-labelledby={ (id) "-title" } class="flex flex-col gap-3" {
			h2 id={ (id) "-title" } class="text-lg font-semibold" { (title) }
			@if let Some(lead) = lead { p class="max-w-prose text-fg-muted" { (lead) } }
			(children)
		}
	}
}

const fn count_icon(kind: CountKind) -> Icon {
	match kind {
		CountKind::Class => Icon::Cube,
		CountKind::Property => Icon::SlidersHorizontal,
		CountKind::Method => Icon::Function,
		CountKind::Event => Icon::Lightning,
		CountKind::Callback => Icon::ArrowUUpLeft,
		CountKind::Enum => Icon::ListBullets,
		CountKind::EnumItem => Icon::Hash,
		CountKind::DataType => Icon::BracketsCurly,
	}
}

fn count_table(cx: &Context, rows: &[CountRow], history_start: &str) -> Markup {
	let current: u32 = rows.iter().map(|row| row.current).sum();
	let removed: u32 = rows.iter().map(|row| row.removed).sum();
	let header = |label: &str, hint: &str| {
		html! { th scope="col" data-number="" { span data-tooltip=(hint) class=(HINT_CLASS) { (label) } } }
	};
	html! {
		div class="stat-table" {
			table {
				thead {
					tr {
						th scope="col" { "Kind" }
						(header("Current", "Items in the current version."))
						(header("Removed", &format!("Items removed since {history_start} and not added back.")))
						(header("Total", "Current and removed items together."))
					}
				}
				tbody {
					@for row in rows {
						tr {
							th scope="row" {
								span class="flex items-center gap-2" {
									(icon(cx, count_icon(row.kind), IconSize::S14, "text-fg-muted"))
									(count_label(row.kind))
								}
							}
							td data-number="" { (row.current) }
							td data-number="" { (row.removed) }
							td data-number="" { (row.current + row.removed) }
						}
					}
				}
				tfoot {
					tr {
						th scope="row" { "All" }
						td data-number="" { (current) }
						td data-number="" { (removed) }
						td data-number="" { (current + removed) }
					}
				}
			}
		}
	}
}

const SHOWN_ITEMS: usize = 8;

fn item_list(items: &[StatRef]) -> Markup {
	html! {
		ul class="flex flex-wrap gap-x-4 gap-y-1" {
			@for item in items { li class="min-w-0 break-all" { code { (code_tokens(&stat_ref_tokens(item))) } } }
		}
	}
}

fn record_table(records: &[StatRecord]) -> Markup {
	html! {
		div class="stat-table" {
			table data-records="" {
				colgroup { col data-label=""; col data-value=""; col; }
				thead {
					tr {
						th scope="col" { "Statistic" }
						th scope="col" data-number="" { "Value" }
						th scope="col" { "Items" }
					}
				}
				tbody {
					@for record in records {
						tr id=(record.id) {
							th scope="row" data-wrap="" { span data-tooltip=(record.hint) class=(ROW_HINT_CLASS) { (record.label) } }
							td data-number="" {
								(js_number(record.value))
								@if let Some(unit) = &record.unit { span class="text-fg-faint" { " " (unit) } }
							}
							td data-wrap="" {
								@if let Some(first) = record.items.first() {
									@if record.items.len() <= SHOWN_ITEMS {
										(item_list(&record.items))
									} @else {
										details class="change-previous" {
											summary { (record.items.len()) " " (ref_noun(first.kind_name())) }
											(item_list(&record.items))
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

struct DataColumn<'a> {
	label: &'a str,
	hint: Option<&'a str>,
	number: bool,
}

const fn column(label: &str) -> DataColumn<'_> {
	DataColumn {
		label,
		hint: None,
		number: false,
	}
}

const fn number(label: &str) -> DataColumn<'_> {
	DataColumn {
		label,
		hint: None,
		number: true,
	}
}

const fn hinted<'a>(label: &'a str, hint: &'a str, number: bool) -> DataColumn<'a> {
	DataColumn {
		label,
		hint: Some(hint),
		number,
	}
}

fn data_table(caption: &str, columns: &[DataColumn], rows: &[(String, Vec<Markup>)]) -> Markup {
	html! {
		div class="stat-table" {
			table {
				caption class="sr-only" { (caption) }
				thead {
					tr {
						@for column in columns {
							th scope="col" data-number=[column.number.then_some("")] {
								@match column.hint {
									None => (column.label),
									Some(hint) => span data-tooltip=(hint) class=(HINT_CLASS) { (column.label) },
								}
							}
						}
					}
				}
				tbody {
					@for (key, cells) in rows {
						tr id=(key) {
							@for (index, cell) in cells.iter().enumerate() {
								@if index == 0 {
									th scope="row" { (cell) }
								} @else {
									td data-number=[columns.get(index).is_some_and(|column| column.number).then_some("")] { (cell) }
								}
							}
						}
					}
				}
			}
		}
	}
}

fn text(value: impl std::fmt::Display) -> Markup {
	html! { (value.to_string()) }
}

fn update_cell(update: Option<&UpdateEvent>) -> Markup {
	html! {
		@match update {
			None => span class="text-fg-faint" { "Not recorded" },
			Some(update) => span class="flex flex-wrap items-baseline gap-x-2" {
				a href=(update_href(&update.version)) class="font-mono font-normal text-fg hover:text-fg-bright hover:underline" { (update.version) }
				span class="text-xs text-fg-faint" { (format_date(&update.published)) }
			},
		}
	}
}

fn comeback_table(cx: &Context) -> Markup {
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

const fn tier_name(tier: AccessTier) -> &'static str {
	match tier {
		AccessTier::Game => "game",
		AccessTier::Plugin => "plugin",
		AccessTier::Command => "command",
		AccessTier::Roblox => "roblox",
	}
}

const fn coverage_name(kind: CoverageKind) -> &'static str {
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

fn access_section(cx: &Context) -> Markup {
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

fn tag_section(
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

fn docs_section(cx: &Context) -> Markup {
	let docs = &cx.stats.docs;
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

fn growth_section(cx: &Context) -> Markup {
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
						ul class="flex flex-wrap gap-x-4 gap-y-1 whitespace-normal" {
							@for item in &year.classes {
								li {
									code {
										@if cx.api.classes.contains_key(&item.name) {
											a href=(class_href(&item.name)) class="text-code-type hover:underline" { (item.name) }
										} @else {
											span class="text-fg-muted" { (item.name) }
										}
									}
									" "
									span class="text-success" { "+" }
									(item.added)
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

fn tally_counts(tally: &ChangeTally) -> ChangeCounts {
	ChangeCounts {
		added: tally.added,
		removed: tally.removed,
		changed: tally.changed,
	}
}

fn tally_cell(tally: &ChangeTally, changes: bool) -> Markup {
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

fn trend_table(
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
						th scope="col" { "Members" }
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
							(tally_cell(&tallies.member, true))
							(tally_cell(&tallies.enumeration, false))
							(tally_cell(&tallies.enum_item, true))
							td data-number="" class=[(deprecated == 0).then_some("text-fg-faint")] { (deprecated) }
						}
					}
				}
			}
		}
	}
}

fn point_hint(trend: &UpdateTrend, value: u32, noun: &str) -> String {
	format!(
		"{} ({}): {value} {noun}",
		trend.version,
		format_date(&trend.published)
	)
}

fn size_chart(updates: &[UpdateTrend], classes: bool) -> Markup {
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

fn list_text(labels: &[String]) -> String {
	match labels {
		[] => String::new(),
		[only] => only.clone(),
		[first, second] => format!("{first} and {second}"),
		[rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
	}
}

fn extremes_text(months: &[PeriodTrend], value: impl Fn(&PeriodTrend) -> u32) -> String {
	let values: Vec<u32> = months.iter().map(&value).collect();
	let labels_of = |target: Option<u32>| {
		list_text(
			&months
				.iter()
				.filter(|month| Some(value(month)) == target)
				.map(|month| month_label(&month.period))
				.collect::<Vec<_>>(),
		)
	};
	format!(
		"Most in {}, fewest in {}",
		labels_of(values.iter().copied().max()),
		labels_of(values.iter().copied().min())
	)
}

fn month_hint(month: &PeriodTrend) -> String {
	let changes = month.tallies.change_total();
	format!(
		"{}: {changes} changes in {} updates, {} per update",
		month_label(&month.period),
		month.updates,
		js_number(js_round(f64::from(changes) / f64::from(month.updates)))
	)
}

fn first_letter(label: &str) -> String {
	label.chars().next().map(String::from).unwrap_or_default()
}

fn month_trends(months: &[PeriodTrend]) -> Markup {
	let columns = |value: &dyn Fn(&PeriodTrend) -> u32| -> Vec<Column> {
		months
			.iter()
			.map(|month| Column {
				label: first_letter(&month_label(&month.period)),
				value: f64::from(value(month)),
				hint: month_hint(month),
			})
			.collect()
	};
	let changes = |month: &PeriodTrend| month.tallies.change_total();
	let updates = |month: &PeriodTrend| month.updates;
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
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				(column_chart("Changes", &extremes_text(months, changes), &columns(&changes)))
				(column_chart("Updates", &extremes_text(months, updates), &columns(&updates)))
			}
			(trend_table("Changes by month", "Month", "Updates", &rows))
		},
	)
}

fn stats_trends(cx: &Context) -> Markup {
	let stats = &cx.stats;
	let updates = &stats.updates;
	let added: u32 = updates.iter().map(|trend| trend.tallies.member.added).sum();
	let removed: u32 = updates
		.iter()
		.map(|trend| trend.tallies.member.removed)
		.sum();
	let bars: Vec<BarColumn> = updates
		.iter()
		.map(|trend| BarColumn {
			up: f64::from(trend.tallies.member.added),
			down: f64::from(trend.tallies.member.removed),
			hint: format!(
				"{} ({}): {} added, {} removed",
				trend.version,
				format_date(&trend.published),
				trend.tallies.member.added,
				trend.tallies.member.removed
			),
		})
		.collect();
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
	html! {
		(stats_section("api-size", "API size", Some("The number of classes and enums after each update."), &html! {
			div class="grid grid-cols-1 gap-6 lg:grid-cols-2" {
				(size_chart(updates, true))
				(size_chart(updates, false))
			}
		}))
		(stats_section("members-per-update", "Members per update", Some("The members that each update added and removed. A number marks a bar that runs past the axis."), &diverging_bar_chart(
			"Members added and removed",
			&format!("{added} added, {removed} removed"),
			("Added", "Removed"),
			&bars,
			&year_ticks(updates.iter().map(|trend| trend.published.as_str())),
		)))
		(stats_section("changes-by-year", "Changes by year", Some(TRENDS_NOTE), &trend_table("Changes by year", "Year", "Updates", &year_rows)))
		(month_trends(&stats.months))
		(stats_section("changes-by-update", "Changes by update", Some("Every tracked update, newest first."), &trend_table("Changes by update", "Update", "Published", &update_rows)))
	}
}

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
					(record_table(&group.records))
					@if comebacks { (comeback_table(cx)) }
				}))
			}
			(access_section(cx))
			(tag_section(cx, "tags", "Tags", TAGS_LEAD, FacetKind::Tag, "Tag", &stats.tags))
			(tag_section(cx, "capabilities", "Capabilities", CAPABILITIES_LEAD, FacetKind::Capability, "Capability", &stats.capabilities))
			(docs_section(cx))
			(growth_section(cx))
			(stats_trends(cx))
		}
	};
	Shell::new(Some(Section::Stats))
		.rail(page_rail(&links))
		.render(cx, &body)
}
