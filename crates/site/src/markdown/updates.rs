use rapi_core::api::release::release_text;
use rapi_core::api::tags::{member_tags, plain_tags};
use rapi_core::api::text::{UPDATES_LEAD, update_summary};
use rapi_core::js::format_date;
use rapi_core::model::{
	ChangeDetail, ChangeEntry, ChangeStatus, DescriptionDetail, DescriptionPart, DetailTone,
	DetailValue, EntryValues, OwnerChanges, Update,
};
use rapi_routes::{OwnerKind, UPDATES, update_href};

use super::doc::doc_markdown;
use super::items::{enum_item_signature_text, member_signature_text};
use super::text::{escape_text, fact_list, inline_code, join_blocks, join_lines, link};
use super::urls::markdown_url;
use crate::components::updates::group_owners;
use crate::context::Context;

fn value_text(value: &DetailValue) -> String {
	if value.tone == DetailTone::Text {
		value.text.clone()
	} else {
		inline_code(&value.text)
	}
}

fn detail_text(detail: &ChangeDetail) -> String {
	match detail {
		ChangeDetail::Tags { label, tags } => format!(
			"{label}: {}",
			plain_tags(tags)
				.into_iter()
				.map(|tag| tag.label)
				.collect::<Vec<_>>()
				.join(", ")
		),
		ChangeDetail::Value {
			label,
			before,
			after,
		} => {
			let after = after.as_ref().map_or_else(|| "none".to_owned(), value_text);
			match before {
				None => format!("{label}: {after}"),
				Some(before) => format!("{label}: {} to {after}", value_text(before)),
			}
		}
	}
}

fn with_details(text: String, details: &[ChangeDetail]) -> String {
	if details.is_empty() {
		text
	} else {
		format!(
			"{text}. {}",
			details
				.iter()
				.map(detail_text)
				.collect::<Vec<_>>()
				.join(". ")
		)
	}
}

fn description_lines(cx: &Context, details: &[DescriptionDetail], indent: &str) -> Vec<String> {
	details
		.iter()
		.map(|detail| {
			let label = match detail.part {
				DescriptionPart::Description => "Description".to_owned(),
				DescriptionPart::Return => "Return value description".to_owned(),
				DescriptionPart::Parameter => format!(
					"Description of {}",
					inline_code(detail.parameter.as_deref().unwrap_or_default())
				),
			};
			let text = format!("{label} {}", detail.status.word());
			let after = join_lines(&doc_markdown(cx, detail.after.as_ref()));
			let after = rapi_core::js::js_trim(&after);
			if after.is_empty() {
				format!("{indent}- {text}.")
			} else {
				format!("{indent}- {text}: {after}")
			}
		})
		.collect()
}

fn entry_line(cx: &Context, entry: &ChangeEntry) -> String {
	let (before, after) = match &entry.values {
		EntryValues::Member { before, after } => (
			before
				.as_deref()
				.map(|member| member_signature_text(member, &entry.owner)),
			after
				.as_deref()
				.map(|member| member_signature_text(member, &entry.owner)),
		),
		EntryValues::EnumItem { before, after } => (
			before
				.as_deref()
				.map(|item| enum_item_signature_text(&entry.owner, item)),
			after
				.as_deref()
				.map(|item| enum_item_signature_text(&entry.owner, item)),
		),
	};
	let text = match (&before, &after) {
		(Some(before), Some(after)) if before != after => {
			format!("Changed {} to {}", inline_code(before), inline_code(after))
		}
		_ => format!(
			"{} {}",
			entry.status.name(),
			inline_code(
				after
					.as_deref()
					.or(before.as_deref())
					.unwrap_or(&entry.name)
			)
		),
	};
	let tags: Vec<String> = if entry.status == ChangeStatus::Added {
		match &entry.values {
			EntryValues::Member {
				after: Some(member),
				..
			} => member_tags(member)
				.into_iter()
				.map(|tag| tag.label)
				.collect(),
			EntryValues::EnumItem {
				after: Some(item), ..
			} => plain_tags(&item.tags)
				.into_iter()
				.map(|tag| tag.label)
				.collect(),
			_ => Vec::new(),
		}
	} else {
		Vec::new()
	};
	let tag_text = if tags.is_empty() {
		text
	} else {
		format!("{text}. Tags: {}", tags.join(", "))
	};
	let mut lines = vec![format!("- {}", with_details(tag_text, &entry.details))];
	lines.extend(description_lines(cx, &entry.descriptions, "  "));
	lines.join("\n")
}

fn owner_block(cx: &Context, owner: &OwnerChanges) -> String {
	let kind = owner.kind.name().to_lowercase();
	let mut lines = Vec::new();
	if let Some(status) = owner.status {
		lines.push(format!(
			"- {}",
			with_details(format!("{} {kind}", status.name()), &owner.details)
		));
	}
	lines.extend(description_lines(cx, &owner.descriptions, ""));
	lines.extend(owner.entries.iter().map(|entry| entry_line(cx, entry)));
	format!("#### {}\n\n{}", escape_text(&owner.name), lines.join("\n"))
}

fn count_line(update: &Update) -> String {
	format!(
		"{} added, {} removed, and {} changed",
		update.counts.added, update.counts.removed, update.counts.changed
	)
}

pub fn update_markdown(cx: &Context, update: &Update) -> String {
	let mut blocks = vec![
		Some(format!("# Update {}", update.version)),
		Some(format!(
			"{} Roblox published this version on {}. It has {} changes: {}. The HTML version of this page is at {}.",
			update_summary(update),
			format_date(&update.published),
			update.counts.total(),
			count_line(update),
			cx.absolute_url(&update_href(&update.version))
		)),
		Some(fact_list(&[
			("Version", Some(release_text(&update.version, &update.hash))),
			(
				"Compared with",
				Some(release_text(
					&update.previous.version,
					&update.previous.hash,
				)),
			),
			(
				"Description changes",
				Some(
					update
						.description_count
						.map_or_else(|| "Not recorded".to_owned(), |count| count.to_string()),
				),
			),
		])),
	];
	for (kind, title) in [(OwnerKind::Class, "Classes"), (OwnerKind::Enum, "Enums")] {
		let groups = group_owners(&update.owners, kind);
		if groups.is_empty() {
			continue;
		}
		blocks.push(Some(format!("## {title}")));
		for (group, owners) in groups {
			blocks.push(Some(format!("### {}", group.title())));
			blocks.extend(owners.into_iter().map(|owner| Some(owner_block(cx, owner))));
		}
	}
	join_blocks(blocks)
}

pub fn update_list_markdown(cx: &Context) -> String {
	let lines: Vec<String> = cx
		.updates
		.iter()
		.map(|update| {
			format!(
				"- {}: {}, compared with {}. {}.",
				link(
					&release_text(&update.version, &update.hash),
					&markdown_url(cx, &update_href(&update.version))
				),
				format_date(&update.published),
				release_text(&update.previous.version, &update.previous.hash),
				count_line(update)
			)
		})
		.collect();
	join_blocks([
		Some("# Updates".to_owned()),
		Some(format!(
			"{UPDATES_LEAD} The HTML version of this list is at {}.",
			cx.absolute_url(UPDATES)
		)),
		Some(if lines.is_empty() {
			"There are no updates yet.".to_owned()
		} else {
			lines.join("\n")
		}),
	])
}
