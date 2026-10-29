use indexmap::IndexMap;
use rapi_core::api::access::identities_allowing;
use rapi_core::api::access::{access_tier_label, tier_counts};
use rapi_core::api::docs_links::{class_docs_href, data_type_docs_href, enum_docs_href};
use rapi_core::api::facets::{
	facet_description, facet_label, use_kind_title, use_sections, use_text,
};
use rapi_core::api::history_text::{HistoryNoun, entry_text};
use rapi_core::api::luau::luau_name;
use rapi_core::api::members::{member_kind_title, members_in_page_order};
use rapi_core::api::release::release_text;
use rapi_core::api::signature::{inline_tokens, signature_parts};
use rapi_core::api::tags::{member_tags, plain_tags};
use rapi_core::js::{compare_names, format_date};
use rapi_core::model::{
	AccessTier, Class, DataType, Enum, EnumItem, Facet, FacetAccess, FacetUse, HistoryEvent,
	ItemHistory, Member, MemberDetail, Removed, Security, TypeRole, TypeUsage, UseKind,
};
use rapi_routes::{
	MemberKind, OwnerKind, change_anchor, class_href, enum_href, type_href, update_href,
};

use super::doc::doc_markdown;
use super::text::{escape_text, fact_list, inline_code, join_blocks, link, table_cell};
use super::urls::{class_markdown_url, enum_markdown_url};
use crate::context::Context;

pub fn release(cx: &Context) -> String {
	release_text(&cx.api.version, &cx.api.hash)
}

fn event_text(event: Option<&HistoryEvent>) -> Option<String> {
	event.map(|event| format!("{} ({})", event.version, format_date(&event.published)))
}

pub fn history_facts(
	history: Option<&ItemHistory>,
	full: bool,
	noun: HistoryNoun,
) -> Vec<(&'static str, Option<String>)> {
	let Some(history) = history else {
		return Vec::new();
	};
	let mut facts = vec![(
		"Added",
		Some(event_text(history.added.as_ref()).unwrap_or_else(|| {
			format!(
				"Not recorded ({} or earlier)",
				history
					.added_before
					.as_deref()
					.unwrap_or("oldest tracked version")
			)
		})),
	)];
	if full {
		facts.push(("Last changed", event_text(history.changed.as_ref())));
	}
	facts.push(("Deprecated", event_text(history.deprecated.as_ref())));
	facts.push((
		"History",
		Some(
			history
				.entries
				.iter()
				.map(|entry| format!("{}: {}", entry.version, entry_text(entry, noun)))
				.collect::<Vec<_>>()
				.join("; "),
		),
	));
	facts
}

pub fn member_signature_text(member: &Member, owner: &str) -> String {
	inline_tokens(&signature_parts(member, owner, None))
		.iter()
		.map(|part| part.text.as_str())
		.collect()
}

pub fn enum_item_signature_text(owner: &str, item: &EnumItem) -> String {
	format!("Enum.{owner}.{} = {}", item.name, item.value)
}

pub fn removed_markdown<T>(
	cx: &Context,
	title: &str,
	kind: OwnerKind,
	owner: &str,
	removed: Option<&[Removed<T>]>,
	text: impl Fn(&T) -> (String, String),
) -> Option<String> {
	let removed = removed.filter(|removed| !removed.is_empty())?;
	let lines: Vec<String> = removed
		.iter()
		.map(|item| {
			let (name, signature) = text(&item.value);
			let url = cx.absolute_url(&format!(
				"{}#{}",
				update_href(&item.removed.version),
				change_anchor(kind, owner, &name)
			));
			format!(
				"- {}: removed in {} ({})",
				inline_code(&signature),
				link(&item.removed.version, &url),
				format_date(&item.removed.published)
			)
		})
		.collect();
	Some(format!("## {title}\n\n{}", lines.join("\n")))
}

const fn role_name(role: TypeRole) -> &'static str {
	match role {
		TypeRole::Value => "value",
		TypeRole::Parameter => "parameter",
		TypeRole::Return => "return",
	}
}

fn role_label(usage: &TypeUsage) -> String {
	let kind = usage.kind.name().to_lowercase();
	match usage.role {
		TypeRole::Value => kind,
		role => format!("{kind} {}", role_name(role)),
	}
}

pub fn usage_markdown(cx: &Context, usages: &[TypeUsage]) -> Option<String> {
	if usages.is_empty() {
		return None;
	}
	let mut by_class: IndexMap<&str, Vec<&TypeUsage>> = IndexMap::new();
	for usage in usages {
		by_class
			.entry(usage.class_name.as_str())
			.or_default()
			.push(usage);
	}
	let mut classes: Vec<(&str, Vec<&TypeUsage>)> = by_class.into_iter().collect();
	classes.sort_by(|a, b| compare_names(a.0, b.0));
	let lines: Vec<String> = classes
		.into_iter()
		.map(|(class_name, list)| {
			let mut by_role: IndexMap<String, Vec<&TypeUsage>> = IndexMap::new();
			for usage in list {
				by_role.entry(role_label(usage)).or_default().push(usage);
			}
			let roles: Vec<String> = by_role
				.into_iter()
				.map(|(role, list)| {
					format!(
						"{role} {}",
						list.iter()
							.map(|usage| inline_code(&usage.member_name))
							.collect::<Vec<_>>()
							.join(", ")
					)
				})
				.collect();
			format!(
				"- {}: {}",
				link(class_name, &class_markdown_url(cx, class_name)),
				roles.join("; ")
			)
		})
		.collect();
	Some(format!(
		"## Used by\n\nEach entry lists the class, then the role and names of the members that use this type.\n\n{}",
		lines.join("\n")
	))
}

fn parameter_list(cx: &Context, member: &Member) -> Option<String> {
	let items: Vec<String> = member
		.parameters()
		.iter()
		.filter(|parameter| parameter.description.is_some())
		.map(|parameter| {
			format!(
				"- {}: {}",
				inline_code(&parameter.name),
				doc_markdown(cx, parameter.description.as_ref())
			)
		})
		.collect();
	(!items.is_empty()).then(|| format!("Parameters:\n\n{}", items.join("\n")))
}

fn preferred(cx: &Context, member: &Member) -> Option<String> {
	let (preferred, target) = (
		member.preferred.as_ref()?,
		member.preferred_target.as_ref()?,
	);
	Some(link(
		&inline_code(&format!("{}.{preferred}", target.owner)),
		&class_markdown_url(cx, &target.owner),
	))
}

fn returns(cx: &Context, member: &Member) -> Option<String> {
	match &member.detail {
		MemberDetail::Method {
			return_description, ..
		}
		| MemberDetail::Callback {
			return_description, ..
		} => {
			let text = doc_markdown(cx, return_description.as_ref());
			(!text.is_empty()).then(|| format!("Returns: {text}"))
		}
		_ => None,
	}
}

fn member_markdown(cx: &Context, member: &Member, owner: &str) -> String {
	let tags: Vec<String> = member_tags(member)
		.into_iter()
		.map(|tag| tag.label)
		.collect();
	let mut facts = vec![
		("Tags", Some(tags.join(", "))),
		("Use instead", preferred(cx, member)),
	];
	facts.extend(history_facts(
		member.history.as_ref(),
		false,
		HistoryNoun::Member,
	));
	join_blocks([
		Some(format!("### {}", escape_text(&member.name))),
		Some(format!(
			"```luau\n{}\n```",
			member_signature_text(member, owner)
		)),
		Some(fact_list(&facts)),
		Some(doc_markdown(cx, member.description.as_ref())),
		if member.kind() == MemberKind::Property {
			None
		} else {
			parameter_list(cx, member)
		},
		returns(cx, member),
	])
	.trim_end()
	.to_owned()
}

fn class_links(cx: &Context, names: &[&str]) -> String {
	names
		.iter()
		.map(|name| link(name, &class_markdown_url(cx, name)))
		.collect::<Vec<_>>()
		.join(", ")
}

pub fn class_markdown(cx: &Context, item: &Class) -> String {
	let ancestors = cx.api.ancestors_of(item);
	let ancestor_names: Vec<&str> = ancestors
		.iter()
		.map(|ancestor| ancestor.name.as_str())
		.collect();
	let subclasses: Vec<&str> = item.subclasses.iter().map(String::as_str).collect();
	let access = (!item.members.is_empty()).then(|| {
		let counts = tier_counts(&item.members);
		AccessTier::ALL
			.into_iter()
			.map(|tier| format!("{}: {}", access_tier_label(tier), counts[tier as usize]))
			.collect::<Vec<_>>()
			.join(", ")
	});
	let mut facts = vec![
		(
			"Inherits from, nearest first",
			Some(class_links(cx, &ancestor_names)),
		),
		("Subclasses", Some(class_links(cx, &subclasses))),
		(
			"Tags",
			Some(
				plain_tags(&item.tags)
					.into_iter()
					.map(|tag| tag.label)
					.collect::<Vec<_>>()
					.join(", "),
			),
		),
		("Memory category", Some(item.memory_category.clone())),
		("Access, by own members", access),
	];
	facts.extend(history_facts(
		item.history.as_ref(),
		true,
		HistoryNoun::Member,
	));
	facts.push(("HTML page", Some(cx.absolute_url(&class_href(&item.name)))));
	facts.push((
		"Roblox documentation",
		Some(class_docs_href(&item.name, None)),
	));
	let mut blocks = vec![
		Some(format!("# {}", escape_text(&item.name))),
		Some(format!("Roblox Engine API class from {}.", release(cx))),
		Some(fact_list(&facts)),
		Some(doc_markdown(cx, item.description.as_ref())),
	];
	for kind in MemberKind::ALL {
		let members = members_in_page_order(&item.members, kind);
		blocks.push((!members.is_empty()).then(|| {
			let entries: Vec<String> = members
				.iter()
				.map(|member| member_markdown(cx, member, &item.name))
				.collect();
			format!("## {}\n\n{}", member_kind_title(kind), entries.join("\n\n"))
		}));
	}
	let inherited: Vec<String> = ancestors
		.iter()
		.filter(|ancestor| !ancestor.members.is_empty())
		.map(|ancestor| {
			let names: Vec<String> = MemberKind::ALL
				.into_iter()
				.flat_map(|kind| {
					members_in_page_order(&ancestor.members, kind)
						.into_iter()
						.map(|member| inline_code(&member.name))
				})
				.collect();
			format!(
				"- From {}: {}",
				link(&ancestor.name, &class_markdown_url(cx, &ancestor.name)),
				names.join(", ")
			)
		})
		.collect();
	blocks.push(
		(!inherited.is_empty())
			.then(|| format!("## Inherited members\n\n{}", inherited.join("\n"))),
	);
	blocks.push(usage_markdown(cx, &item.used_by));
	blocks.push(removed_markdown(
		cx,
		"Removed members",
		OwnerKind::Class,
		&item.name,
		item.removed_members.as_deref(),
		|member: &Member| {
			(
				member.name.clone(),
				member_signature_text(member, &item.name),
			)
		},
	));
	join_blocks(blocks)
}

fn item_row(cx: &Context, item: &EnumItem) -> String {
	let mut notes: Vec<String> = plain_tags(&item.tags)
		.into_iter()
		.map(|tag| tag.label)
		.collect();
	notes.extend(
		item.legacy_names
			.iter()
			.map(|name| format!("Legacy name: {}", inline_code(name))),
	);
	let description: Vec<String> = [
		notes.join(", "),
		doc_markdown(cx, item.description.as_ref()),
	]
	.into_iter()
	.filter(|part| !part.is_empty())
	.collect();
	format!(
		"| {} | {} | {} |",
		inline_code(&item.name),
		item.value,
		table_cell(&description.join(". "))
	)
}

pub fn enum_markdown(cx: &Context, item: &Enum) -> String {
	let mut facts = vec![(
		"Tags",
		Some(
			plain_tags(&item.tags)
				.into_iter()
				.map(|tag| tag.label)
				.collect::<Vec<_>>()
				.join(", "),
		),
	)];
	facts.extend(history_facts(
		item.history.as_ref(),
		true,
		HistoryNoun::Item,
	));
	facts.push(("HTML page", Some(cx.absolute_url(&enum_href(&item.name)))));
	facts.push(("Roblox documentation", Some(enum_docs_href(&item.name))));
	let items = if item.items.is_empty() {
		"This enum has no items.".to_owned()
	} else {
		let mut sorted: Vec<&EnumItem> = item.items.iter().collect();
		sorted.sort_by_key(|entry| entry.value);
		let rows: Vec<String> = sorted
			.into_iter()
			.map(|entry| item_row(cx, entry))
			.collect();
		format!(
			"## Items\n\n| Name | Value | Description |\n| --- | --- | --- |\n{}",
			rows.join("\n")
		)
	};
	join_blocks([
		Some(format!("# Enum.{}", escape_text(&item.name))),
		Some(format!("Roblox Engine API enum from {}.", release(cx))),
		Some(fact_list(&facts)),
		Some(doc_markdown(cx, item.description.as_ref())),
		Some(items),
		usage_markdown(cx, &item.used_by),
		removed_markdown(
			cx,
			"Removed items",
			OwnerKind::Enum,
			&item.name,
			item.removed_items.as_deref(),
			|entry: &EnumItem| {
				(
					entry.name.clone(),
					enum_item_signature_text(&item.name, entry),
				)
			},
		),
	])
}

pub fn type_markdown(cx: &Context, item: &DataType) -> String {
	let luau = luau_name(&item.name);
	join_blocks([
		Some(format!("# {}", escape_text(&item.name))),
		Some(format!("Roblox Engine API data type from {}.", release(cx))),
		Some(fact_list(&[
			("Category", Some(item.category.name().to_owned())),
			("Luau type", (luau != item.name).then(|| inline_code(luau))),
			("HTML page", Some(cx.absolute_url(&type_href(&item.name)))),
			("Roblox documentation", data_type_docs_href(&item.name)),
		])),
		Some(
			usage_markdown(cx, &item.used_by)
				.unwrap_or_else(|| "No class member uses this type.".to_owned()),
		),
	])
}

fn use_markdown_url(cx: &Context, item: &FacetUse) -> String {
	match item.kind {
		UseKind::Class => class_markdown_url(cx, &item.name),
		UseKind::Enum => enum_markdown_url(cx, &item.name),
		UseKind::EnumItem => enum_markdown_url(cx, &item.owner),
		_ => class_markdown_url(cx, &item.owner),
	}
}

pub fn facet_markdown(cx: &Context, facet: &Facet) -> String {
	let identities = match (facet.kind, Security::parse(&facet.name)) {
		(rapi_routes::FacetKind::Security, Some(security)) => Some(
			identities_allowing(security)
				.iter()
				.map(ToString::to_string)
				.collect::<Vec<_>>()
				.join(", "),
		),
		_ => None,
	};
	let uses = use_sections(&facet.uses)
		.into_iter()
		.map(|(kind, uses)| {
			let lines: Vec<String> = uses
				.iter()
				.map(|item| {
					let access = match item.access {
						Some(FacetAccess::Read) => " (Read access)",
						Some(FacetAccess::Write) => " (Write access)",
						None => "",
					};
					format!(
						"- {}{access}",
						link(&use_text(item), &use_markdown_url(cx, item))
					)
				})
				.collect();
			format!("## {}\n\n{}", use_kind_title(kind), lines.join("\n"))
		})
		.collect::<Vec<_>>()
		.join("\n\n");
	join_blocks([
		Some(format!("# {}", escape_text(&facet.name))),
		Some(format!(
			"{} From {}.",
			facet_description(facet),
			release(cx)
		)),
		Some(fact_list(&[
			("Kind", Some(facet_label(facet.kind).to_owned())),
			("Uses", Some(facet.uses.len().to_string())),
			("Usable by identities", identities),
			(
				"HTML page",
				Some(cx.absolute_url(&cx.facet_href(facet.kind, &facet.name))),
			),
		])),
		Some(uses),
	])
}
