use maud::{Markup, html};
use rapi_core::api::tags::{DisplayTag, TagTone};
use rapi_core::js::format_date;
use rapi_core::model::ItemHistory;

use crate::html::classes;

const fn tone_class(tone: TagTone) -> &'static str {
	match tone {
		TagTone::Neutral => "",
		TagTone::Warning => "tag-warning",
		TagTone::Danger => "tag-danger",
	}
}

pub fn tag_list(tags: &[DisplayTag], class: &str) -> Markup {
	html! {
		@if !tags.is_empty() {
			ul class=(classes(&["tag-list", class])) {
				@for tag in tags {
					li data-tooltip=[tag.hint.as_deref()] class=(classes(&["tag-chip gap-1.5", tone_class(tag.tone)])) {
						(tag.label)
						@if let Some(note) = &tag.note {
							span class="text-fg-faint" { (note) }
						}
					}
				}
			}
		}
	}
}

/// Adds when the item was deprecated to its Deprecated tag.
pub fn dated_tags(tags: Vec<DisplayTag>, history: Option<&ItemHistory>) -> Vec<DisplayTag> {
	let deprecated = history.and_then(|history| history.deprecated.as_ref());
	tags.into_iter()
		.map(|tag| {
			if tag.label != "Deprecated" {
				return tag;
			}
			match deprecated {
				None => DisplayTag {
					hint: Some(
						"Not recorded: deprecated before the oldest version that this site tracks."
							.to_owned(),
					),
					..tag
				},
				Some(event) => {
					let date = format_date(&event.published);
					DisplayTag {
						hint: Some(format!("Deprecated in {} on {date}.", event.version)),
						note: Some(date),
						..tag
					}
				}
			}
		})
		.collect()
}
