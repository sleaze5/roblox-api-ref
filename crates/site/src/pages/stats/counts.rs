use maud::{Markup, html};
use rapi_core::api::stats_text::{SERVICES_HINT, count_label};
use rapi_core::icons::Icon;
use rapi_core::model::{ChangeStatus, CountKind, CountRow};

use super::tables::{HINT_CLASS, ROW_HINT_CLASS};
use crate::components::changes::status_count_or_zero;
use crate::context::Context;
use crate::html::{IconSize, icon};

pub const fn count_icon(kind: CountKind) -> Icon {
	match kind {
		CountKind::Class | CountKind::Service => Icon::Cube,
		CountKind::Property => Icon::SlidersHorizontal,
		CountKind::Method => Icon::Function,
		CountKind::Event => Icon::Lightning,
		CountKind::Callback => Icon::ArrowUUpLeft,
		CountKind::Enum => Icon::ListBullets,
		CountKind::EnumItem => Icon::Hash,
		CountKind::DataType => Icon::BracketsCurly,
	}
}

pub fn count_table(cx: &Context, rows: &[CountRow], history_start: &str) -> Markup {
	let totals = rows.iter().filter(|row| !row.kind.is_subset());
	let current: u32 = totals.clone().map(|row| row.current).sum();
	let removed: u32 = totals.map(|row| row.removed).sum();
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
								@if row.kind.is_subset() {
									span class="flex items-center gap-2 pl-6" {
										span data-tooltip=(SERVICES_HINT) class=(ROW_HINT_CLASS) { (count_label(row.kind)) }
									}
								} @else {
									span class="flex items-center gap-2" {
										(icon(cx, count_icon(row.kind), IconSize::S14, "text-fg-muted"))
										(count_label(row.kind))
									}
								}
							}
							td data-number="" { (row.current) }
							td data-number="" { (status_count_or_zero(ChangeStatus::Removed, row.removed)) }
							td data-number="" { (row.current + row.removed) }
						}
					}
				}
				tfoot {
					tr {
						th scope="row" { "All" }
						td data-number="" { (current) }
						td data-number="" { (status_count_or_zero(ChangeStatus::Removed, removed)) }
						td data-number="" { (current + removed) }
					}
				}
			}
		}
	}
}
