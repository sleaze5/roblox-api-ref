use maud::{Markup, html};
use rapi_core::api::history_text::{HistoryNoun, entry_text};
use rapi_core::js::format_date;
use rapi_core::model::{HistoryEvent, ItemHistory};
use rapi_routes::{OwnerKind, owner_anchor, update_href};

use crate::components::facts::fact;
use crate::html::classes;

pub fn history_href(event: &HistoryEvent, kind: OwnerKind, owner: &str) -> String {
	format!(
		"{}#{}",
		update_href(&event.version),
		owner_anchor(kind, owner)
	)
}

const fn noun(kind: OwnerKind) -> &'static str {
	match kind {
		OwnerKind::Class => "class",
		OwnerKind::Enum => "enum",
	}
}

fn event_value(event: &HistoryEvent, kind: OwnerKind, name: &str) -> Markup {
	html! {
		span class="flex flex-wrap items-baseline gap-x-2" {
			a href=(history_href(event, kind, name)) class="font-mono text-fg underline decoration-line-strong hover:decoration-fg" { (event.version) }
			span class="text-fg-faint" { (format_date(&event.published)) }
		}
	}
}

pub fn history_facts(
	history: Option<&ItemHistory>,
	kind: OwnerKind,
	name: &str,
	start: Option<&str>,
) -> Markup {
	let noun = noun(kind);
	let added = history.and_then(|history| history.added.as_ref());
	let changed = history.and_then(|history| history.changed.as_ref());
	let added_hint = match added {
		None => format!(
			"Not recorded: this {noun} already existed in {}, the oldest version that this site tracks.",
			start.unwrap_or("the oldest version")
		),
		Some(_) => format!("The update that added this {noun}."),
	};
	html! {
		(fact("Added", Some(&added_hint), &match added {
			None => html! { "Not recorded" },
			Some(event) => event_value(event, kind, name),
		}))
		@if let Some(event) = changed {
			(fact(
				"Last changed",
				Some(&format!("The last update that changed this {noun} or its members, not counting descriptions.")),
				&event_value(event, kind, name),
			))
		}
	}
}

pub fn history_list(
	history: Option<&ItemHistory>,
	fragment: &str,
	noun: HistoryNoun,
	class: &str,
) -> Markup {
	html! {
		@if let Some(history) = history {
			details class=(classes(&["history-disclosure", class])) {
				summary {
					"History"
					span class="text-fg-faint" { (history.entries.len()) }
				}
				ul class="flex flex-col gap-1" {
					@for entry in &history.entries {
						li class="flex flex-wrap items-baseline gap-x-3" {
							a href={ (update_href(&entry.version)) "#" (fragment) } class="font-mono text-fg hover:text-fg-bright hover:underline" { (entry.version) }
							span class="text-xs text-fg-faint" { (format_date(&entry.published)) }
							span class="text-fg-muted" { (entry_text(entry, noun)) }
						}
					}
					@if let Some(before) = &history.added_before {
						li class="text-fg-faint" {
							"Added: not recorded ("
							span class="font-mono" { (before) }
							" or earlier)"
						}
					}
				}
			}
		}
	}
}
