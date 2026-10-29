use rapi_core::api::facets::{
	INDEX_DESCRIPTION, facet_index_description, facet_title, sorted_facets,
};
use rapi_core::api::tags::plain_tags;
use rapi_core::api::text::REMOVED_LEAD;
use rapi_core::js::{compare_names, format_date};
use rapi_core::model::RemovedOwner;
use rapi_routes::{
	ABOUT, CLASSES, CONTACT, ENUMS, FACET_KINDS, FacetKind, HOME, INDEX, LLMS_TXT, PRIVACY, STATS,
	TYPES, UPDATES, class_href, facet_index_href, markdown_href,
};

use super::items::release;
use super::text::{inline_code, join_blocks, link};
use super::urls::{
	class_markdown_url, enum_markdown_url, facet_index_markdown_url, facet_markdown_url,
	markdown_url, type_markdown_url,
};
use crate::context::Context;
use crate::meta::SITE_DESCRIPTION;
use crate::site_pages::{Block, Inline, PageContent, is_external};

fn sorted_names<'a>(names: impl Iterator<Item = &'a String>) -> Vec<&'a String> {
	let mut names: Vec<&String> = names.collect();
	names.sort_by(|a, b| compare_names(a, b));
	names
}

fn tag_suffix(tags: &[String]) -> String {
	let labels: Vec<String> = plain_tags(tags).into_iter().map(|tag| tag.label).collect();
	if labels.is_empty() {
		String::new()
	} else {
		format!(" ({})", labels.join(", "))
	}
}

fn intro(cx: &Context, kind: &str, href: &str) -> String {
	format!(
		"Every {kind} in the Roblox Engine API from {}. The HTML version of this list is at {}.",
		release(cx),
		cx.absolute_url(href)
	)
}

pub fn class_list_markdown(cx: &Context) -> String {
	let lines: Vec<String> = sorted_names(cx.api.classes.keys())
		.into_iter()
		.map(|name| {
			let item = &cx.api.classes[name];
			let parent = item
				.superclass
				.as_ref()
				.map_or_else(String::new, |superclass| {
					format!(": inherits from {superclass}")
				});
			format!(
				"- {}{parent}{}",
				link(name, &class_markdown_url(cx, name)),
				tag_suffix(&item.tags)
			)
		})
		.collect();
	join_blocks([
		Some("# Classes".to_owned()),
		Some(intro(cx, "class", CLASSES)),
		Some(lines.join("\n")),
	])
}

pub fn enum_list_markdown(cx: &Context) -> String {
	let lines: Vec<String> = sorted_names(cx.api.enums.keys())
		.into_iter()
		.map(|name| {
			format!(
				"- {}{}",
				link(&format!("Enum.{name}"), &enum_markdown_url(cx, name)),
				tag_suffix(&cx.api.enums[name].tags)
			)
		})
		.collect();
	join_blocks([
		Some("# Enums".to_owned()),
		Some(intro(cx, "enum", ENUMS)),
		Some(lines.join("\n")),
	])
}

pub fn type_list_markdown(cx: &Context) -> String {
	let lines: Vec<String> = sorted_names(cx.api.data_types.keys())
		.into_iter()
		.map(|name| {
			format!(
				"- {}: {}",
				link(name, &type_markdown_url(cx, name)),
				cx.api.data_types[name].category.name()
			)
		})
		.collect();
	join_blocks([
		Some("# Data types".to_owned()),
		Some(intro(cx, "data type", TYPES)),
		Some(lines.join("\n")),
	])
}

fn removed_lines(title: &str, owners: &[RemovedOwner]) -> String {
	if owners.is_empty() {
		return format!("## {title}\n\nNone.");
	}
	let lines: Vec<String> = owners
		.iter()
		.map(|owner| {
			format!(
				"- {}: removed in {} ({})",
				inline_code(&owner.name),
				owner.removed.version,
				format_date(&owner.removed.published)
			)
		})
		.collect();
	format!("## {title}\n\n{}", lines.join("\n"))
}

pub fn removed_list_markdown(cx: &Context) -> String {
	join_blocks([
		Some("# Removed".to_owned()),
		Some(REMOVED_LEAD.to_owned()),
		Some(removed_lines("Classes", cx.api.removed_classes())),
		Some(removed_lines("Enums", cx.api.removed_enums())),
	])
}

fn count(value: usize, noun: &str) -> String {
	format!("{value} {noun}{}", if value == 1 { "" } else { "s" })
}

fn facet_lines(cx: &Context, kind: FacetKind) -> String {
	sorted_facets(&cx.api, kind)
		.into_iter()
		.map(|facet| {
			format!(
				"- {}: {}",
				link(&facet.name, &facet_markdown_url(cx, kind, &facet.name)),
				count(facet.uses.len(), "use")
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

fn index_intro(cx: &Context, description: &str, href: &str) -> String {
	format!(
		"{description} From {}. The HTML version of this list is at {}.",
		release(cx),
		cx.absolute_url(href)
	)
}

pub fn facet_index_markdown(cx: &Context, kind: FacetKind) -> String {
	join_blocks([
		Some(format!("# {}", facet_title(kind))),
		Some(index_intro(
			cx,
			facet_index_description(kind),
			&facet_index_href(kind),
		)),
		Some(facet_lines(cx, kind)),
	])
}

pub fn index_markdown(cx: &Context) -> String {
	let mut blocks = vec![
		Some("# Index".to_owned()),
		Some(index_intro(cx, INDEX_DESCRIPTION, INDEX)),
		Some(format!(
			"## Data types\n\n- {}: {}",
			link("Data types", &markdown_url(cx, TYPES)),
			count(cx.api.data_types.len(), "data type")
		)),
	];
	blocks.extend(FACET_KINDS.map(|kind| {
		Some(format!(
			"## {}\n\n{}\n\n{}",
			facet_title(kind),
			link(
				&format!("All {}", facet_title(kind).to_lowercase()),
				&facet_index_markdown_url(cx, kind)
			),
			facet_lines(cx, kind)
		))
	}));
	join_blocks(blocks)
}

fn inline_markdown(cx: &Context, content: &[Inline]) -> String {
	content
		.iter()
		.map(|part| match part {
			Inline::Text(text) => text.clone(),
			Inline::Code(code) => inline_code(code),
			Inline::Link { text, href } => link(
				text,
				&if is_external(href) {
					(*href).to_owned()
				} else {
					cx.absolute_url(href)
				},
			),
		})
		.collect()
}

pub fn site_page_markdown(cx: &Context, title: &str, content: &PageContent) -> String {
	let mut blocks = vec![Some(format!("# {title}")), Some(content.lead.to_owned())];
	for section in &content.sections {
		blocks.push(Some(format!("## {}", section.heading)));
		blocks.extend(section.blocks.iter().map(|block| {
			Some(match block {
				Block::Paragraph(content) => inline_markdown(cx, content),
				Block::List(items) => items
					.iter()
					.map(|item| format!("- {}", inline_markdown(cx, item)))
					.collect::<Vec<_>>()
					.join("\n"),
			})
		}));
	}
	join_blocks(blocks)
}

pub fn llms_txt(cx: &Context) -> String {
	let api = &cx.api;
	let home = cx.absolute_url(HOME);
	let counts = format!(
		"{} classes, {} enums, and {} data types",
		api.classes.len(),
		api.enums.len(),
		api.data_types.len()
	);
	let index_link = |label: &str, href: &str, text: &str| {
		format!(
			"- {}: {text}",
			link(label, &cx.absolute_url(&markdown_href(href)))
		)
	};
	join_blocks([
		Some("# Roblox API Reference".to_owned()),
		Some(format!("> {SITE_DESCRIPTION}")),
		Some(format!(
			"This build is generated from the API dump and the English API docs for {} on the {} channel, which Roblox published on {}. It lists {counts}.",
			release(cx),
			api.channel,
			api.published.get(..10).unwrap_or(&api.published)
		)),
		Some(format!(
			"Every page has a Markdown version. To get the Markdown version of a page, remove the trailing slash from the page URL and add {}. For example, the Markdown version of {} is {}. You can also request any page URL with {} to get its Markdown version from the same URL.",
			inline_code(".md"),
			cx.absolute_url(&class_href("Part")),
			class_markdown_url(cx, "Part"),
			inline_code("Accept: text/markdown")
		)),
		Some("## When to use this reference".to_owned()),
		Some(
			[
				"Use this reference to:",
				"",
				"- Look up the properties, methods, events, and callbacks of a Roblox class.",
				"- Find the members that a class inherits from its superclasses.",
				"- Check the Luau signature of a member, including parameter types, default values, and return types.",
				"- Check the items and numeric values of a Roblox enum.",
				"- Check the security level, thread safety, capabilities, and tags of a member, such as Deprecated or Yields.",
				"- Find the class members that use a data type or an enum.",
				"- Check what changed in the API from one Roblox Studio version to the next.",
			]
			.join("\n"),
		),
		Some("Do not use this reference for tutorials, guides, or the Luau language.".to_owned()),
		Some("## URL patterns".to_owned()),
		Some(
			[
				format!("- Class: {home}classes/{{ClassName}}.md"),
				format!("- Enum: {home}enums/{{EnumName}}.md"),
				format!("- Data type: {home}index/types/{{TypeName}}.md"),
				format!("- Update: {home}updates/{{Version}}.md"),
			]
			.join("\n"),
		),
		Some(format!(
			"Replace the placeholder with the exact name, such as {}, {}, or {}, or with a version, such as {}. Names are case-sensitive. Enum URLs do not include the {} prefix.",
			inline_code("Part"),
			inline_code("KeyCode"),
			inline_code("CFrame"),
			inline_code(&api.version),
			inline_code("Enum.")
		)),
		Some("## Indexes".to_owned()),
		Some(
			[
				index_link("Classes", CLASSES, "Every class, with its superclass and tags, such as Service and Deprecated"),
				index_link("Enums", ENUMS, "Every enum, with its tags"),
				index_link("Data types", TYPES, "Every data type, with its category"),
				index_link("Index", INDEX, "Every tag, property category, security level, capability, thread safety level, and memory category, with links to what uses each one"),
				index_link("Updates", UPDATES, "Every update, with the number of added, removed, and changed API items"),
				index_link("Statistics", STATS, "How many classes, members, enums, and data types exist and were removed, the largest and longest of each, deprecations, removed items that came back, access, tags, capabilities, description coverage, and how much each update, year, and month changed"),
			]
			.join("\n"),
		),
		Some("## Optional".to_owned()),
		Some(
			[
				index_link("About", ABOUT, "What this site is, where its data comes from, and how it is updated"),
				index_link("Contact", CONTACT, "How to report a problem or suggest a feature"),
				index_link("Privacy policy", PRIVACY, "What the site collects"),
				format!("- {}: Every HTML page", link("Sitemap", &format!("{home}sitemap.xml"))),
			]
			.join("\n"),
		),
	])
}

pub fn not_found_markdown(cx: &Context) -> String {
	join_blocks([
		Some("# Page not found".to_owned()),
		Some(format!(
			"This page does not exist in {}. Check the spelling and case of the class, enum, or data type name in the URL, or start from one of these pages.",
			release(cx)
		)),
		Some(
			[
				format!(
					"- {}: What this reference covers and its URL patterns",
					link("llms.txt", &cx.absolute_url(LLMS_TXT))
				),
				format!(
					"- {}: Every class",
					link("Classes", &markdown_url(cx, CLASSES))
				),
				format!("- {}: Every enum", link("Enums", &markdown_url(cx, ENUMS))),
				format!(
					"- {}: Every data type",
					link("Data types", &markdown_url(cx, TYPES))
				),
				format!(
					"- {}: Every tag, security level, and other facet",
					link("Index", &markdown_url(cx, INDEX))
				),
				format!(
					"- {}: What changed in each Roblox Studio build",
					link("Updates", &markdown_url(cx, UPDATES))
				),
				format!(
					"- {}: Every HTML page",
					link("Sitemap", &cx.absolute_url("/sitemap.xml"))
				),
			]
			.join("\n"),
		),
	])
}
