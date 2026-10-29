//! The members that use a class, enum, or data type.

use maud::{Markup, html};
use rapi_core::icons::Icon;
use rapi_core::js::compare_names;
use rapi_core::model::{TypeRole, TypeUsage};
use rapi_routes::{MemberKind, member_href};

use crate::components::members::kind_icon;
use crate::context::Context;
use crate::html::{IconSize, icon};

pub struct RoleSection {
	pub role: TypeRole,
	pub id: &'static str,
	pub title: &'static str,
	pub icon: Icon,
}

pub const ROLE_SECTIONS: [RoleSection; 3] = [
	RoleSection {
		role: TypeRole::Value,
		id: "used-by-properties",
		title: "Used by properties",
		icon: Icon::SlidersHorizontal,
	},
	RoleSection {
		role: TypeRole::Parameter,
		id: "used-by-parameters",
		title: "Used by parameters",
		icon: Icon::SignIn,
	},
	RoleSection {
		role: TypeRole::Return,
		id: "used-by-returns",
		title: "Used by return values",
		icon: Icon::SignOut,
	},
];

pub fn usage_count(usages: &[TypeUsage], role: TypeRole) -> usize {
	usages.iter().filter(|usage| usage.role == role).count()
}

fn usage_link(cx: &Context, usage: &TypeUsage) -> Markup {
	let separator = if usage.kind == MemberKind::Method {
		":"
	} else {
		"."
	};
	html! {
		li data-name={ (usage.class_name) (separator) (usage.member_name) } {
			a href=(member_href(&usage.class_name, usage.kind, &usage.member_name)) class="flex h-7 items-center gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover" {
				(icon(cx, kind_icon(usage.kind), IconSize::S14, "text-fg-muted"))
				span class="truncate" {
					span class="text-code-type" { (usage.class_name) }
					span { (separator) }
					span class="text-fg-bright" { (usage.member_name) }
				}
			}
		}
	}
}

pub fn usage_sections(cx: &Context, usages: &[TypeUsage], name: &str) -> Markup {
	let sections: Vec<(&RoleSection, Vec<&TypeUsage>)> = ROLE_SECTIONS
		.iter()
		.map(|section| {
			let mut list: Vec<&TypeUsage> = usages
				.iter()
				.filter(|usage| usage.role == section.role)
				.collect();
			list.sort_by(|a, b| {
				compare_names(&a.class_name, &b.class_name)
					.then_with(|| compare_names(&a.member_name, &b.member_name))
			});
			(section, list)
		})
		.filter(|(_, list)| !list.is_empty())
		.collect();
	if sections.is_empty() {
		return html! { p class="text-fg-muted" { "No class members use " (name) "." } };
	}
	html! {
		@for (section, list) in &sections {
			section id=(section.id) data-list-section="" aria-labelledby={ (section.id) "-title" } class="flex flex-col gap-3" {
				h2 id={ (section.id) "-title" } class="flex items-center gap-2 text-lg font-semibold" {
					(icon(cx, section.icon, IconSize::S20, "text-fg-muted"))
					(section.title)
					span class="text-sm font-normal text-fg-faint" { (list.len()) }
				}
				ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2" {
					@for usage in list { (usage_link(cx, usage)) }
				}
			}
		}
	}
}
