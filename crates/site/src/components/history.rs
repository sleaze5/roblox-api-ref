use maud::{Markup, html};
use rapi_core::api::history_text::{
	HistoryNoun, HistoryPart, capitalize, deprecation_text, entry_parts, entry_status, plural,
};
use rapi_core::js::format_date;
use rapi_core::model::{
	ChangeCounts, ChangeStatus, Deprecation, HistoryEntry, HistoryEvent, ItemHistory, MemberCounts,
};
use rapi_routes::{OwnerKind, owner_anchor, update_href};

use crate::components::changes::{change_counts, change_mark, status_name};
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

fn event_value(event: &HistoryEvent, kind: OwnerKind, name: &str, status: ChangeStatus) -> Markup {
	html! {
		span class="flex flex-wrap items-baseline gap-x-2" {
			(change_mark(Some(status), None))
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
	let unrecorded = history.is_some_and(|history| history.unrecorded);
	let added_hint = match added {
		None if unrecorded => format!(
			"Unrecorded: the public API dumps that older versions are compared with do not list this {noun}, so the update that added it is unknown."
		),
		None => format!(
			"Not recorded: this {noun} already existed in {}, the oldest version that this site tracks.",
			start.unwrap_or("the oldest version")
		),
		Some(_) => format!("The update that added this {noun}."),
	};
	html! {
		(fact("Added", Some(&added_hint), &match added {
			None if unrecorded => html! { "Unrecorded" },
			None => html! { "Not recorded" },
			Some(event) => event_value(event, kind, name, ChangeStatus::Added),
		}))
		@if let Some(event) = changed {
			(fact(
				"Last changed",
				Some(&format!("The last update that changed this {noun} or its members, not counting descriptions.")),
				&event_value(event, kind, name, ChangeStatus::Changed),
			))
		}
	}
}

fn members_part(counts: MemberCounts, deprecated: u32, noun: HistoryNoun) -> Markup {
	let total = counts.added + counts.removed + counts.changed;
	let counts = ChangeCounts {
		added: counts.added,
		removed: counts.removed,
		changed: counts.changed,
	};
	html! {
		@if total > 0 {
			span class="inline-flex flex-wrap items-baseline gap-x-1.5" {
				(change_counts(&counts, true))
				span { (if total == 1 { noun.word().to_owned() } else { format!("{}s", noun.word()) }) }
			}
		}
		@if total > 0 && deprecated > 0 { ", " }
		@if deprecated > 0 {
			span class="text-warning" {
				@if total > 0 { (deprecated) " deprecated" } @else { (plural(deprecated, noun.word())) " deprecated" }
			}
		}
	}
}

fn part_text(text: &str, first: bool) -> String {
	if first {
		capitalize(text)
	} else {
		text.to_owned()
	}
}

fn entry_summary(entry: &HistoryEntry, noun: HistoryNoun) -> Markup {
	let parts = entry_parts(entry);
	html! {
		span class="text-fg-muted" {
			@for (index, part) in parts.iter().enumerate() {
				@if index > 0 { ", " }
				@match part {
					HistoryPart::Added => span class="text-success" { (part_text("added", index == 0)) },
					HistoryPart::Removed => span class="text-danger" { (part_text("removed", index == 0)) },
					HistoryPart::Changed => span { (part_text("changed", index == 0)) },
					HistoryPart::Members { counts, deprecated } => (members_part(*counts, *deprecated, noun)),
					HistoryPart::Deprecation(deprecation) => span class=[(*deprecation == Deprecation::Set).then_some("text-warning")] {
						(part_text(deprecation_text(*deprecation), index == 0))
					},
					HistoryPart::Text(text) => span { (part_text(text, index == 0)) },
				}
			}
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
						@let status = entry_status(entry);
						li {
							(change_mark(Some(status), Some(&format!("{}:", status_name(status)))))
							span class="flex min-w-0 flex-wrap items-baseline gap-x-3" {
								a href={ (update_href(&entry.version)) "#" (fragment) } class="font-mono text-fg hover:text-fg-bright hover:underline" { (entry.version) }
								span class="text-xs text-fg-faint" { (format_date(&entry.published)) }
								(entry_summary(entry, noun))
							}
						}
					}
					@if history.unrecorded {
						li class="text-fg-faint" { span {} span { "Added: unrecorded" } }
					}
					@if let Some(before) = &history.added_before {
						li class="text-fg-faint" {
							span {}
							span {
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
}
