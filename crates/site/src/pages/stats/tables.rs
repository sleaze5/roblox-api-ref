use maud::{Markup, html};
use rapi_core::js::format_date;
use rapi_core::model::UpdateEvent;
use rapi_routes::update_href;

pub const HINT_CLASS: &str =
	"cursor-help underline decoration-line-strong decoration-dotted underline-offset-4";
pub const ROW_HINT_CLASS: &str =
	"cursor-help font-normal underline decoration-line-strong decoration-dotted underline-offset-4";

pub fn stats_section(id: &str, title: &str, lead: Option<&str>, children: &Markup) -> Markup {
	html! {
		section id=(id) aria-labelledby={ (id) "-title" } class="flex flex-col gap-3" {
			h2 id={ (id) "-title" } class="text-lg font-semibold" { (title) }
			@if let Some(lead) = lead { p class="max-w-prose text-fg-muted" { (lead) } }
			(children)
		}
	}
}

pub struct DataColumn<'a> {
	pub label: &'a str,
	pub hint: Option<&'a str>,
	pub number: bool,
}

pub const fn column(label: &str) -> DataColumn<'_> {
	DataColumn {
		label,
		hint: None,
		number: false,
	}
}

pub const fn number(label: &str) -> DataColumn<'_> {
	DataColumn {
		label,
		hint: None,
		number: true,
	}
}

pub const fn hinted<'a>(label: &'a str, hint: &'a str, number: bool) -> DataColumn<'a> {
	DataColumn {
		label,
		hint: Some(hint),
		number,
	}
}

pub fn data_table(caption: &str, columns: &[DataColumn], rows: &[(String, Vec<Markup>)]) -> Markup {
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

pub fn text(value: impl std::fmt::Display) -> Markup {
	html! { (value.to_string()) }
}

pub fn update_cell(update: Option<&UpdateEvent>) -> Markup {
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
