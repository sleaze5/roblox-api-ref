use maud::{Markup, html};
use rapi_core::api::code::{CodeToken, CodeTone, linked_token, token, unlinked};
use rapi_core::api::signature::{SignatureParts, inline_tokens, signature_parts};
use rapi_core::icons::Icon;
use rapi_core::js::format_date;
use rapi_core::model::{ChangeCounts, ChangeStatus, EnumItem, Member, Removed};
use rapi_routes::{
	OwnerKind, change_anchor, enum_href, enum_item_href, member_href, removed_enum_item_anchor,
	removed_member_anchor, update_href,
};

use crate::components::members::signature;
use crate::context::Context;
use crate::html::{IconSize, icon};

pub const fn status_name(status: ChangeStatus) -> &'static str {
	match status {
		ChangeStatus::Added => "Added",
		ChangeStatus::Removed => "Removed",
		ChangeStatus::Changed => "Changed",
	}
}

pub fn change_mark(status: Option<ChangeStatus>, label: Option<&str>) -> Markup {
	html! {
		span class="change-mark" data-status=[status.map(status_name)] {
			@if let Some(label) = label { span class="sr-only" { (label) } }
		}
	}
}

const fn status_symbol(status: ChangeStatus) -> (&'static str, &'static str) {
	match status {
		ChangeStatus::Added => ("+", "text-success"),
		ChangeStatus::Removed => ("−", "text-danger"),
		ChangeStatus::Changed => ("~", "text-fg-faint"),
	}
}

fn status_symbol_mark(status: ChangeStatus) -> Markup {
	let (mark, class) = status_symbol(status);
	html! { span aria-hidden="true" class={ "font-mono " (class) } { (mark) } }
}

pub fn inline_mark(status: ChangeStatus) -> Markup {
	html! {
		(status_symbol_mark(status))
		span class="sr-only" { (status_name(status)) ": " }
	}
}

pub fn status_count(status: ChangeStatus, count: impl maud::Render) -> Markup {
	html! { span class="tabular-nums" { (status_symbol_mark(status)) (count) } }
}

pub fn status_count_or_zero(status: ChangeStatus, count: u32) -> Markup {
	html! {
		@if count == 0 {
			span class="text-fg-faint" { "0" }
		} @else {
			(status_count(status, count))
		}
	}
}

pub fn change_counts(counts: &ChangeCounts, changes: bool) -> Markup {
	let shown: Vec<ChangeStatus> = ChangeStatus::ALL
		.into_iter()
		.filter(|status| counts.get(*status) > 0 && (changes || *status != ChangeStatus::Changed))
		.collect();
	html! {
		@if !shown.is_empty() {
			span class="flex gap-2 tabular-nums" {
				@for status in shown {
					span {
						(status_symbol_mark(status))
						span class="sr-only" { (status_name(status)) " " }
						(counts.get(status))
					}
				}
			}
		}
	}
}

#[derive(Clone, Copy)]
pub enum ChangedValue<'a> {
	Member(&'a Member),
	EnumItem(&'a EnumItem),
}

impl ChangedValue<'_> {
	pub fn name(&self) -> &str {
		match self {
			Self::Member(member) => &member.name,
			Self::EnumItem(item) => &item.name,
		}
	}

	pub fn removed_anchor(&self) -> String {
		match self {
			Self::Member(member) => removed_member_anchor(member.kind(), &member.name),
			Self::EnumItem(item) => removed_enum_item_anchor(&item.name),
		}
	}
}

fn enum_item_parts(owner: &str, item: &EnumItem) -> SignatureParts {
	SignatureParts {
		owner: vec![
			token("Enum.", CodeTone::Punctuation),
			linked_token(owner, CodeTone::Type, enum_href(owner)),
			token(".", CodeTone::Punctuation),
		],
		head: vec![linked_token(
			&item.name,
			CodeTone::Constant,
			enum_item_href(owner, &item.name),
		)],
		parameters: None,
		tail: vec![
			token(" = ", CodeTone::Punctuation),
			token(item.value.to_string(), CodeTone::Constant),
		],
	}
}

fn live_tokens(cx: &Context, tokens: &[CodeToken]) -> Vec<CodeToken> {
	let links = cx.site_links();
	tokens
		.iter()
		.map(|part| match &part.href {
			Some(href) if !links.contains(href) => CodeToken {
				href: None,
				..part.clone()
			},
			_ => part.clone(),
		})
		.collect()
}

pub fn change_parts(
	cx: &Context,
	owner: &str,
	value: ChangedValue,
	linked: bool,
) -> SignatureParts {
	let parts = match value {
		ChangedValue::Member(member) => {
			let href = member_href(owner, member.kind(), &member.name);
			signature_parts(member, owner, Some(&href))
		}
		ChangedValue::EnumItem(item) => enum_item_parts(owner, item),
	};
	if linked {
		parts.map(|tokens| live_tokens(cx, tokens))
	} else {
		parts.map(unlinked)
	}
}

pub fn parts_text(parts: &SignatureParts) -> String {
	inline_tokens(parts)
		.iter()
		.map(|part| part.text.as_str())
		.collect()
}

pub struct RemovedList<'a> {
	pub id: &'a str,
	pub title: &'a str,
	pub kind: OwnerKind,
	pub owner: &'a str,
}

pub fn removed_items<'a, T>(
	cx: &Context,
	list: &RemovedList,
	removed: Option<&'a [Removed<T>]>,
	value: impl Fn(&'a T) -> ChangedValue<'a>,
) -> Markup {
	let removed = removed.unwrap_or_default();
	html! {
		@if !removed.is_empty() {
			section id=(list.id) aria-labelledby={ (list.id) "-title" } class="flex flex-col gap-3" {
				h2 id={ (list.id) "-title" } class="flex items-center gap-2 text-lg font-semibold" {
					(icon(cx, Icon::ClockCounterClockwise, IconSize::S20, "text-fg-muted"))
					(list.title)
					span class="text-sm font-normal text-fg-faint" { (removed.len()) }
				}
				ul class="flex flex-col gap-3" {
					@for item in removed {
						@let changed = value(&item.value);
						li id=(changed.removed_anchor()) data-change="" class="flex flex-col gap-1" {
							div class="change-line" {
								(change_mark(Some(ChangeStatus::Removed), Some("Removed:")))
								(signature(&change_parts(cx, list.owner, changed, false)))
							}
							p class="flex flex-wrap items-baseline gap-x-2 change-indent text-fg-faint" {
								"Removed in"
								a href={ (update_href(&item.removed.version)) "#" (change_anchor(list.kind, list.owner, changed.name())) } class="font-mono text-fg-muted hover:text-fg hover:underline" { (item.removed.version) }
								span class="text-xs" { (format_date(&item.removed.published)) }
							}
						}
					}
				}
			}
		}
	}
}
