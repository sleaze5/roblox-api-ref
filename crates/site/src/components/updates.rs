use maud::{Markup, html};
use rapi_core::api::signature::SignatureParts;
use rapi_core::api::tags::{DisplayTag, member_tags, plain_tags};
use rapi_core::icons::Icon;
use rapi_core::model::{
	ChangeDetail, ChangeEntry, ChangeStatus, DescriptionDetail, DescriptionPart, DetailTone,
	DetailValue, EntryValues, OwnerChanges, Update,
};
use rapi_routes::{OwnerKind, change_anchor, class_href, enum_href, owner_anchor};

use crate::components::changes::{ChangedValue, change_mark, change_parts, parts_text};
use crate::components::doc::doc_text;
use crate::components::members::signature;
use crate::components::tags::tag_list;
use crate::context::Context;
use crate::html::class_icon_style;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OwnerGroupKind {
	Additions,
	Removals,
	Changes,
}

impl OwnerGroupKind {
	pub const ALL: [Self; 3] = [Self::Additions, Self::Removals, Self::Changes];

	pub const fn key(self) -> &'static str {
		match self {
			Self::Additions => "additions",
			Self::Removals => "removals",
			Self::Changes => "changes",
		}
	}

	pub const fn title(self) -> &'static str {
		match self {
			Self::Additions => "Additions",
			Self::Removals => "Removals",
			Self::Changes => "Changes",
		}
	}

	const fn of(owner: &OwnerChanges) -> Self {
		match owner.status {
			Some(ChangeStatus::Added) => Self::Additions,
			Some(ChangeStatus::Removed) => Self::Removals,
			_ => Self::Changes,
		}
	}
}

pub struct OwnerSubsection<'a> {
	pub group: OwnerGroupKind,
	pub owners: Vec<&'a OwnerChanges>,
	pub id: String,
	pub title: &'static str,
	pub rail_label: String,
}

pub struct OwnerSection<'a> {
	pub id: &'static str,
	pub title: &'static str,
	pub icon: Icon,
	pub count: usize,
	pub groups: Vec<OwnerSubsection<'a>>,
}

pub fn group_owners(
	owners: &[OwnerChanges],
	kind: OwnerKind,
) -> Vec<(OwnerGroupKind, Vec<&OwnerChanges>)> {
	OwnerGroupKind::ALL
		.into_iter()
		.map(|group| {
			(
				group,
				owners
					.iter()
					.filter(|owner| owner.kind == kind && OwnerGroupKind::of(owner) == group)
					.collect::<Vec<_>>(),
			)
		})
		.filter(|(_, owners)| !owners.is_empty())
		.collect()
}

fn owner_section(update: &Update, kind: OwnerKind) -> OwnerSection<'_> {
	let (id, title, icon, noun) = match kind {
		OwnerKind::Class => ("classes", "Classes", Icon::Cube, "Class"),
		OwnerKind::Enum => ("enums", "Enums", Icon::ListBullets, "Enum"),
	};
	let groups: Vec<OwnerSubsection> = group_owners(&update.owners, kind)
		.into_iter()
		.map(|(group, owners)| OwnerSubsection {
			group,
			owners,
			id: format!("{id}-{}", group.key()),
			title: group.title(),
			rail_label: format!("{noun} {}", group.title().to_lowercase()),
		})
		.collect();
	OwnerSection {
		id,
		title,
		icon,
		count: groups.iter().map(|group| group.owners.len()).sum(),
		groups,
	}
}

pub fn owner_sections(update: &Update) -> Vec<OwnerSection<'_>> {
	[OwnerKind::Class, OwnerKind::Enum]
		.into_iter()
		.map(|kind| owner_section(update, kind))
		.filter(|section| section.count > 0)
		.collect()
}

fn detail_value_text(cx: &Context, value: &DetailValue) -> Markup {
	html! {
		@match value.tone {
			DetailTone::Text => span { (value.text) },
			DetailTone::Code => code { (value.text) },
			DetailTone::Type => {
				@if cx.api.classes.contains_key(&value.text) {
					code { a href=(class_href(&value.text)) class="text-code-type" { (value.text) } }
				} @else {
					code class="text-code-type" { (value.text) }
				}
			}
		}
	}
}

pub fn change_details(cx: &Context, details: &[ChangeDetail]) -> Markup {
	html! {
		@if !details.is_empty() {
			dl class="flex flex-col gap-1 change-indent" {
				@for detail in details {
					div class="flex flex-wrap items-center gap-x-3 gap-y-1" {
						dt class="text-fg-faint" { (detail.label()) }
						@match detail {
							ChangeDetail::Tags { tags, .. } => dd { (tag_list(&plain_tags(tags), "")) },
							ChangeDetail::Value { before, after, .. } => dd class="flex min-w-0 flex-wrap items-center gap-x-2" {
								@if let Some(before) = before {
									del class="text-fg-faint" { (detail_value_text(cx, before)) }
								}
								@if before.is_some() && after.is_some() {
									span class="text-fg-faint" {
										span aria-hidden="true" { "→" }
										span class="sr-only" { "changed to" }
									}
								}
								@if let Some(after) = after {
									ins class="text-fg no-underline" { (detail_value_text(cx, after)) }
								}
							},
						}
					}
				}
			}
		}
	}
}

fn description_label(detail: &DescriptionDetail) -> Markup {
	let status = detail.status.word();
	html! {
		@match detail.part {
			DescriptionPart::Description => { "Description " (status) }
			DescriptionPart::Return => { "Return value description " (status) }
			DescriptionPart::Parameter => {
				"Description of "
				code class="text-fg" { (detail.parameter.as_deref().unwrap_or_default()) }
				" " (status)
			}
		}
	}
}

pub fn description_changes(details: &[DescriptionDetail]) -> Markup {
	html! {
		@if !details.is_empty() {
			div class="flex flex-col gap-3 change-indent" {
				@for detail in details {
					div class="flex flex-col gap-1.5" {
						p class="text-fg-faint" { (description_label(detail)) }
						(doc_text(detail.after.as_ref(), "member-description"))
						@if let Some(before) = &detail.before {
							details class="change-previous" {
								summary { "Previous description" }
								(doc_text(Some(before), "member-description"))
							}
						}
					}
				}
			}
		}
	}
}

struct Line {
	status: Option<ChangeStatus>,
	label: Option<&'static str>,
	parts: SignatureParts,
}

fn change_lines(cx: &Context, entry: &ChangeEntry) -> Vec<Line> {
	let (before, after) = match &entry.values {
		EntryValues::Member { before, after } => (
			before.as_deref().map(ChangedValue::Member),
			after.as_deref().map(ChangedValue::Member),
		),
		EntryValues::EnumItem { before, after } => (
			before.as_deref().map(ChangedValue::EnumItem),
			after.as_deref().map(ChangedValue::EnumItem),
		),
	};
	let before = before.map(|value| change_parts(cx, &entry.owner, value, false));
	let after = after.map(|value| change_parts(cx, &entry.owner, value, true));
	let line = |status, label, parts| Line {
		status: Some(status),
		label: Some(label),
		parts,
	};
	match (before, after) {
		(Some(before), Some(after)) => {
			if parts_text(&before) == parts_text(&after) {
				vec![line(ChangeStatus::Changed, "Changed:", after)]
			} else {
				vec![
					line(ChangeStatus::Removed, "Before:", before),
					line(ChangeStatus::Added, "After:", after),
				]
			}
		}
		(None, Some(after)) => vec![line(ChangeStatus::Added, "Added:", after)],
		(Some(before), None) => vec![line(ChangeStatus::Removed, "Removed:", before)],
		(None, None) => Vec::new(),
	}
}

fn added_tags(entry: &ChangeEntry) -> Vec<DisplayTag> {
	if entry.status != ChangeStatus::Added {
		return Vec::new();
	}
	match &entry.values {
		EntryValues::Member {
			after: Some(member),
			..
		} => member_tags(member),
		EntryValues::EnumItem {
			after: Some(item), ..
		} => plain_tags(&item.tags),
		_ => Vec::new(),
	}
}

fn change_entry_row(
	cx: &Context,
	entry: &ChangeEntry,
	owner_added: bool,
	id_prefix: &str,
) -> Markup {
	let tags = added_tags(entry);
	let mut lines = change_lines(cx, entry);
	if owner_added {
		for line in &mut lines {
			line.status = None;
			line.label = None;
		}
	}
	html! {
		li id={ (id_prefix) (change_anchor(entry.owner_kind(), &entry.owner, &entry.name)) } data-change="" data-name={ (entry.owner) "." (entry.name) } class="flex flex-col gap-1" {
			@for line in &lines {
				div class="change-line" {
					(change_mark(line.status, line.label))
					(signature(&line.parts))
				}
			}
			@if !tags.is_empty() { (tag_list(&tags, "change-indent")) }
			(change_details(cx, &entry.details))
			(description_changes(&entry.descriptions))
		}
	}
}

const fn kind_noun(kind: OwnerKind) -> &'static str {
	match kind {
		OwnerKind::Class => "class",
		OwnerKind::Enum => "enum",
	}
}

fn owner_name(cx: &Context, owner: &OwnerChanges) -> Markup {
	let href = match owner.kind {
		OwnerKind::Class => cx
			.api
			.classes
			.contains_key(&owner.name)
			.then(|| class_href(&owner.name)),
		OwnerKind::Enum => cx
			.api
			.enums
			.contains_key(&owner.name)
			.then(|| enum_href(&owner.name)),
	};
	html! {
		code class="min-w-0 truncate font-semibold text-code-type" {
			@match href {
				None => (owner.name),
				Some(href) => a href=(href) { (owner.name) },
			}
		}
	}
}

pub fn owner_icon(cx: &Context, owner: &OwnerChanges) -> Option<u32> {
	match owner.kind {
		OwnerKind::Class => cx.api.classes.get(&owner.name).and_then(|item| item.icon),
		OwnerKind::Enum => None,
	}
}

#[derive(Clone, Copy)]
pub enum Heading {
	H3,
	H4,
}

pub fn owner_group(
	cx: &Context,
	owner: &OwnerChanges,
	id_prefix: &str,
	heading: Heading,
) -> Markup {
	let id = format!("{id_prefix}{}", owner_anchor(owner.kind, &owner.name));
	let noun = kind_noun(owner.kind);
	let label = match owner.status {
		Some(ChangeStatus::Added) => Some(format!("Added {noun}")),
		Some(ChangeStatus::Removed) => Some(format!("Removed {noun}")),
		_ => None,
	};
	let status = owner
		.status
		.or((!owner.descriptions.is_empty()).then_some(ChangeStatus::Changed));
	let heading_content = html! {
		(change_mark(status, None))
		span class="flex min-w-0 items-center gap-2" {
			span aria-hidden="true" data-kind=(owner.kind.name()) style=[class_icon_style(owner_icon(cx, owner))] class="kind-icon text-fg-muted" {}
			span class="sr-only" { (label.as_deref().unwrap_or(noun)) }
			(owner_name(cx, owner))
			@if let Some(label) = &label {
				span aria-hidden="true" class="shrink-0 text-xs text-fg-faint" { (label) }
			}
		}
	};
	let title_id = format!("{id}-title");
	let change = status.map(|_| "");
	html! {
		section id=(id) data-change-group="" data-name=(owner.name) aria-labelledby=(title_id) class="flex flex-col gap-2" {
			@match heading {
				Heading::H3 => h3 id=(title_id) data-change=[change] class="change-line" { (heading_content) },
				Heading::H4 => h4 id=(title_id) data-change=[change] class="change-line" { (heading_content) },
			}
			(change_details(cx, &owner.details))
			(description_changes(&owner.descriptions))
			@if !owner.entries.is_empty() {
				ul class="flex flex-col gap-2" {
					@for entry in &owner.entries {
						(change_entry_row(cx, entry, owner.status == Some(ChangeStatus::Added), id_prefix))
					}
				}
			}
		}
	}
}

pub fn owner_subsection_block(
	cx: &Context,
	group: &OwnerSubsection,
	title: Option<&str>,
	id_prefix: &str,
) -> Markup {
	let id = format!("{id_prefix}{}", group.id);
	html! {
		section id=(id) data-list-section="" aria-labelledby={ (id) "-title" } class="flex flex-col gap-4" {
			h3 id={ (id) "-title" } class="flex items-center gap-2 border-b border-line-subtle pb-2 text-base font-semibold" {
				(title.unwrap_or(group.title))
				span class="text-sm font-normal text-fg-faint" { (group.owners.len()) }
			}
			div class="flex flex-col gap-6" {
				@for owner in &group.owners { (owner_group(cx, owner, id_prefix, Heading::H4)) }
			}
		}
	}
}
