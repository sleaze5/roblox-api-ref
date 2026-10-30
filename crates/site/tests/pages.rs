use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use rapi_core::generate::pipeline::{Paths, generate_site};
use rapi_routes::{
	FACET_KINDS, FacetKind, class_href, enum_href, facet_index_href, type_href, update_href,
};
use rapi_site::markdown::{markdown_path, page_markdown};
use rapi_site::meta::{SITE_DESCRIPTION, page_title};
use rapi_site::routes::{IndexList, Route, SitePageName, match_route, route_paths};
use rapi_site::{Context, RenderedPage, render_page};
use serde_json::Value;

const ORIGIN: &str = "https://api.example.com";

static CX: LazyLock<Context> = LazyLock::new(|| {
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let site = generate_site(&Paths::new(root)).expect("the site generates from data/ and diffs/");
	Context::new(
		site.api,
		site.updates,
		site.stats,
		site.slugs,
		"/icons.svg".to_owned(),
		ORIGIN.to_owned(),
	)
});

fn route(path: &str) -> Route<'static> {
	match_route(&CX, path)
}

fn markdown(path: &str) -> String {
	page_markdown(&CX, &route(path)).content
}

#[test]
fn resolves_every_generated_page_without_duplicate_paths() {
	let paths = route_paths(&CX);
	assert!(paths.len() > 2000);
	assert_eq!(paths.iter().collect::<HashSet<_>>().len(), paths.len());
	assert_eq!(
		paths
			.iter()
			.map(|path| path.to_lowercase())
			.collect::<HashSet<_>>()
			.len(),
		paths.len()
	);
	let missing: Vec<&String> = paths
		.iter()
		.filter(|path| matches!(route(path), Route::NotFound))
		.collect();
	assert_eq!(missing, ["/404"]);
}

#[test]
fn gives_classes_enums_types_facets_and_updates_their_pages() {
	assert!(matches!(route(&class_href("Part")), Route::Class(item) if item.name == "Part"));
	assert!(matches!(route(&enum_href("Material")), Route::Enum(item) if item.name == "Material"));
	assert!(matches!(route(&type_href("Array")), Route::Type(item) if item.name == "Array"));
	assert!(matches!(
		route("/index/types/"),
		Route::IndexList(IndexList::Types)
	));
	assert!(matches!(route("/about/"), Route::Site(SitePageName::About)));
	let paths: HashSet<String> = route_paths(&CX).into_iter().collect();
	for kind in FACET_KINDS {
		assert!(
			matches!(route(&facet_index_href(kind)), Route::FacetIndex(found) if found == kind)
		);
		for facet in CX.api.facets.of(kind).values() {
			let href = CX.facet_href(kind, &facet.name);
			assert!(paths.contains(&href), "{href}");
			assert!(
				matches!(route(&href), Route::Facet(found) if found.name == facet.name),
				"{href}"
			);
		}
	}
	assert_eq!(
		CX.facet_href(FacetKind::PropertyCategory, "Physics Response"),
		"/index/property-categories/Physics%20Response/"
	);
	for update in &CX.updates {
		assert!(
			matches!(route(&update_href(&update.version)), Route::Update(found) if found.version == update.version)
		);
	}
}

#[test]
fn rejects_unknown_values_extra_segments_and_retired_url_shapes() {
	for path in [
		"/index/unknown/",
		"/index/types/unknown/",
		"/index/tags/unknown/",
		"/index/types/Array/extra/",
		"/classes/Part/extra/",
		"/class/Part/",
		"/enum/Material/",
		"/type/Array/",
		"/tag/Deprecated/",
		"/about/extra/",
		"/updates/0.0.0.0/",
		"/classes/%E0%A4%A/",
	] {
		assert!(matches!(route(path), Route::NotFound), "{path}");
	}
}

#[test]
fn names_each_page_before_the_site_name() {
	let title = |path: &str| page_title(&route(path));
	assert_eq!(title("/"), "Roblox API Reference");
	assert_eq!(title("/classes/Part/"), "Part | Roblox API Reference");
	assert_eq!(
		title("/enums/Material/"),
		"Enum.Material | Roblox API Reference"
	);
	assert_eq!(
		title("/index/types/CFrame/"),
		"CFrame | Roblox API Reference"
	);
	assert_eq!(title("/index/"), "Index | Roblox API Reference");
	assert_eq!(title("/about/"), "About | Roblox API Reference");
	assert_eq!(title("/nope/"), "Page not found | Roblox API Reference");
}

fn head(path: &str) -> (RenderedPage, String) {
	let page = render_page(&CX, path);
	let tags = rapi_site::head::head_tags(&CX, &page, path);
	(page, tags)
}

fn meta(html: &str, key: &str) -> Option<String> {
	["name", "property"].iter().find_map(|attribute| {
		let needle = format!("<meta {attribute}=\"{key}\" content=\"");
		let start = html.find(&needle)? + needle.len();
		Some(html[start..start + html[start..].find('"')?].to_owned())
	})
}

#[test]
fn gives_social_cards_the_title_the_description_and_the_large_image() {
	let (_, html) = head("/classes/Part/");
	for key in ["og:title", "twitter:title"] {
		assert_eq!(
			meta(&html, key).as_deref(),
			Some("Part | Roblox API Reference")
		);
	}
	for key in ["og:description", "twitter:description"] {
		assert_eq!(meta(&html, key).as_deref(), Some(SITE_DESCRIPTION));
	}
	assert_eq!(
		meta(&html, "twitter:card").as_deref(),
		Some("summary_large_image")
	);
	for key in ["og:image", "og:image:secure_url", "twitter:image"] {
		assert_eq!(meta(&html, key), Some(format!("{ORIGIN}/og-image.png")));
	}
	assert_eq!(meta(&html, "og:image:width").as_deref(), Some("1200"));
	assert_eq!(meta(&html, "og:image:height").as_deref(), Some("630"));
	assert_eq!(meta(&html, "og:type").as_deref(), Some("article"));
	assert_eq!(
		meta(&html, "robots").as_deref(),
		Some("index, follow, max-image-preview:large")
	);
}

#[test]
fn links_the_canonical_url_the_markdown_version_and_search() {
	let (_, html) = head("/classes/Part/");
	assert!(html.contains(&format!(
		"<link rel=\"canonical\" href=\"{ORIGIN}/classes/Part/\">"
	)));
	assert_eq!(
		meta(&html, "og:url"),
		Some(format!("{ORIGIN}/classes/Part/"))
	);
	assert!(
		html.contains("<link rel=\"alternate\" type=\"text/markdown\" href=\"/classes/Part.md\">")
	);
	assert!(html.contains("rel=\"search\""));
}

#[test]
fn keeps_the_not_found_page_out_of_search_results() {
	let (page, html) = head("/404");
	assert_eq!(page.status, 404);
	assert_eq!(meta(&html, "robots").as_deref(), Some("noindex"));
	assert!(!html.contains("rel=\"canonical\""));
	assert!(!html.contains("rel=\"alternate\""));
	assert_eq!(meta(&html, "og:type").as_deref(), Some("website"));
	assert!(!html.contains("application/ld+json"));
}

fn structured_data(path: &str) -> Value {
	let (_, html) = head(path);
	let start = html
		.find("<script type=\"application/ld+json\">")
		.expect("structured data")
		+ 35;
	let end = start + html[start..].find("</script>").expect("a closed script");
	serde_json::from_str(&html[start..end]).expect("valid JSON")
}

fn graph_node<'a>(data: &'a Value, kind: &str) -> &'a Value {
	data["@graph"]
		.as_array()
		.into_iter()
		.flatten()
		.find(|node| node["@type"] == kind)
		.unwrap_or_else(|| panic!("no {kind} node"))
}

#[test]
fn publishes_the_organization_on_the_home_about_and_contact_pages() {
	let organization = format!("{ORIGIN}/#organization");
	let home = structured_data("/");
	assert_eq!(home["@context"], "https://schema.org");
	assert_eq!(
		graph_node(&home, "WebSite")["publisher"]["@id"],
		organization.as_str()
	);
	let node = graph_node(&home, "Organization");
	assert_eq!(
		node["sameAs"],
		serde_json::json!(["https://github.com/sleaze5/roblox-api-ref"])
	);
	assert_eq!(
		node["contactPoint"]["url"],
		format!("{ORIGIN}/contact/").as_str()
	);
	for (path, kind) in [("/about/", "AboutPage"), ("/contact/", "ContactPage")] {
		assert_eq!(
			graph_node(&structured_data(path), kind)["mainEntity"]["@id"],
			organization.as_str()
		);
	}
	let article = structured_data("/classes/Part/");
	assert_eq!(article["@type"], "TechArticle");
	assert!(article.get("@graph").is_none());
	assert_eq!(
		article["breadcrumb"]["itemListElement"][2]["item"],
		format!("{ORIGIN}/classes/Part/").as_str()
	);
}

#[test]
fn renders_every_page_with_its_markdown_at_the_page_url_plus_md() {
	for path in route_paths(&CX) {
		let page = render_page(&CX, &path);
		assert!(page.body.contains("id=\"main\""), "{path}");
		assert!(
			page.markdown.content.ends_with('\n') && !page.markdown.content.ends_with("\n\n"),
			"{path}"
		);
		match path.as_str() {
			"/" => assert_eq!(page.markdown.href, "/llms.txt"),
			"/404" => assert_eq!(page.markdown.href, "/404.md"),
			_ => assert_eq!(
				page.markdown.href,
				format!("{}.md", path.trim_end_matches('/')),
				"{path}"
			),
		}
		assert!(page.markdown.content.starts_with("# "), "{path}");
	}
	assert_eq!(markdown_path(&CX, &route("/no/such/page/")), "/404.md");
}

#[test]
fn explains_a_missing_page_and_links_to_llms_txt_and_the_sitemap() {
	let content = markdown("/404");
	assert!(content.starts_with("# Page not found\n\nThis page does not exist"));
	assert!(content.contains(&format!("({ORIGIN}/llms.txt)")));
	assert!(content.contains(&format!("({ORIGIN}/sitemap.xml)")));
}

#[test]
fn links_to_other_pages_with_absolute_urls_on_the_site_origin() {
	for path in [
		"/classes/",
		"/enums/",
		"/index/",
		"/index/types/",
		"/index/security-levels/",
		"/index/tags/Deprecated/",
		"/updates/",
		"/stats/",
		"/404",
	] {
		let content = markdown(path);
		let targets: Vec<&str> = content
			.split("](")
			.skip(1)
			.filter_map(|rest| rest.split(')').next())
			.collect();
		assert!(!targets.is_empty(), "{path}");
		for target in targets {
			assert!(
				target.starts_with(&format!("{ORIGIN}/")),
				"{path}: {target}"
			);
		}
	}
}

#[test]
fn tells_agents_about_markdown_and_lists_the_indexes_in_llms_txt() {
	let text = markdown("/");
	assert!(text.contains("`Accept: text/markdown`"));
	for href in [
		"/index.md",
		"/stats.md",
		"/about.md",
		"/contact.md",
		"/privacy.md",
	] {
		assert!(text.contains(&format!("({ORIGIN}{href})")), "{href}");
	}
	assert!(!text.contains("roblox.com"));
}

#[test]
fn renders_the_statistics_page_with_its_counts_and_records() {
	let content = markdown("/stats/");
	assert!(content.starts_with("# Statistics\n"));
	assert!(content.contains(&format!("| Classes | {} |", CX.api.classes.len())));
	assert!(content.contains("## Changes by update"));
}

#[test]
fn gives_each_site_page_at_least_500_characters_of_text() {
	for path in ["/about/", "/contact/", "/privacy/"] {
		let content = markdown(path);
		let text: String = content
			.chars()
			.filter(|character| !"#[]`*-".contains(*character))
			.collect();
		assert!(text.len() > 500, "{path}");
		assert!(!content.contains("roblox.com"), "{path}");
	}
	assert!(markdown("/privacy/").starts_with("# Privacy policy\n"));
}

#[test]
fn groups_update_changes_into_additions_removals_and_changes() {
	for update in &CX.updates {
		let content = markdown(&update_href(&update.version));
		let headings: Vec<&str> = content
			.lines()
			.filter(|line| line.starts_with("## ") || line.starts_with("### "))
			.collect();
		let mut order = Vec::new();
		for heading in headings {
			match heading {
				"## Classes" | "## Enums" => order.push(heading),
				"### Additions" | "### Removals" | "### Changes" => {
					assert!(order.last().is_some(), "{}: {heading}", update.version);
				}
				_ => panic!("{}: unexpected heading {heading}", update.version),
			}
		}
		assert!(
			order == ["## Classes", "## Enums"] || order.len() <= 1,
			"{}: {order:?}",
			update.version
		);
	}
}

#[test]
fn lists_description_changes_under_their_item() {
	let update = CX
		.updates
		.iter()
		.find(|update| update.version == "0.619.0.6190508")
		.expect("a tracked update");
	let count = update.description_count.expect("recorded descriptions");
	assert!(count > 0);
	let content = markdown(&update_href(&update.version));
	assert!(content.contains(&format!("- Description changes: {count}")));
	assert!(
		content
			.lines()
			.any(|line| line.starts_with("  - Description changed: ")
				|| line.starts_with("  - Description added: "))
	);
}

#[test]
fn calls_an_update_without_changes_a_rebuild() {
	let update = CX
		.updates
		.iter()
		.find(|update| update.version == "0.47.0.391")
		.expect("a backfilled update");
	assert!(update.owners.is_empty());
	assert_eq!(update.description_count, None);
	let content = markdown(&update_href(&update.version));
	assert!(content.contains("A rebuild of the Roblox Engine API"));
	assert!(content.contains("- Description changes: Not recorded"));
}

#[test]
fn lists_the_tags_of_added_members_in_update_markdown() {
	let update = &CX.updates[0];
	let content = markdown(&update_href(&update.version));
	assert!(
		content
			.lines()
			.any(|line| line.starts_with("- Added `") && line.contains(". Tags: "))
	);
}

#[test]
fn lists_removed_items_with_a_link_to_their_removal() {
	let (item, removed) = CX
		.api
		.enums
		.values()
		.find_map(|item| Some((item, item.removed_items.as_ref()?.first()?)))
		.expect("an enum with a removed item");
	let content = markdown(&enum_href(&item.name));
	let line = format!(
		"- `Enum.{}.{} = {}`: removed in [{}]({ORIGIN}/updates/{}/#Enum-{}.{})",
		item.name,
		removed.value.name,
		removed.value.value,
		removed.removed.version,
		removed.removed.version,
		item.name,
		removed.value.name
	);
	assert!(content.contains(&line), "{line}");
}

#[test]
fn lists_the_20_newest_updates_in_the_atom_feed() {
	let feed = rapi_site::feed::updates_feed(&CX);
	assert!(feed.starts_with(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?><feed xmlns=\"http://www.w3.org/2005/Atom\">"
	));
	assert_eq!(feed.matches("<entry>").count(), 20.min(CX.updates.len()));
	let newest = &CX.updates[0];
	assert!(feed.contains(&format!(
		"<link href=\"{ORIGIN}/updates/{}/\"/>",
		newest.version
	)));
	assert!(feed.contains(&format!("<updated>{}</updated>", newest.published)));
	assert!(feed.contains(&format!(
		"{} added, {} removed, and {} changed.",
		newest.counts.added, newest.counts.removed, newest.counts.changed
	)));
}

#[test]
fn fills_the_filter_counts_before_scripts_run() {
	let page = render_page(&CX, "/classes/Part/");
	let members = page.body.matches("data-member=\"\"").count();
	assert!(page.body.contains(&format!(">{members} total</output>")));
	let enums = render_page(&CX, "/enums/");
	assert!(
		enums
			.body
			.contains(&format!(">{} total</output>", CX.api.enums.len()))
	);
}

fn bool_property(name: &str) -> rapi_core::model::Member {
	use rapi_core::model::{ApiType, Capabilities, Member, MemberDetail, Security, TypeCategory};
	Member {
		name: name.to_owned(),
		history: None,
		tags: Vec::new(),
		security: Security::None,
		thread_safety: "Safe".to_owned(),
		capabilities: Capabilities::default(),
		preferred: None,
		preferred_target: None,
		description: None,
		detail: MemberDetail::Property {
			ty: ApiType {
				category: TypeCategory::Primitive,
				name: "bool".to_owned(),
				optional: false,
				members: None,
			},
			category: "Data".to_owned(),
			default: None,
			write_security: Security::None,
			can_load: true,
			can_save: true,
			simulation_access: false,
		},
	}
}

fn linked_texts(parts: &rapi_core::api::signature::SignatureParts) -> Vec<String> {
	rapi_core::api::signature::inline_tokens(parts)
		.into_iter()
		.filter(|part| part.href.is_some())
		.map(|part| part.text)
		.collect()
}

#[test]
fn keeps_the_text_of_a_historical_signature_but_drops_links_to_removed_pages() {
	use rapi_site::components::changes::{ChangedValue, change_parts, parts_text};
	let gone = bool_property("Anchored");
	let parts = change_parts(&CX, "OldPart", ChangedValue::Member(&gone), true);
	assert_eq!(parts_text(&parts), "OldPart.Anchored: boolean");
	assert_eq!(linked_texts(&parts), ["boolean"]);
	let live = CX.api.classes["BasePart"]
		.members
		.iter()
		.find(|member| member.name == "Anchored")
		.expect("BasePart.Anchored");
	assert!(
		linked_texts(&change_parts(
			&CX,
			"BasePart",
			ChangedValue::Member(live),
			true
		))
		.contains(&"Anchored".to_owned())
	);
	assert!(
		linked_texts(&change_parts(
			&CX,
			"BasePart",
			ChangedValue::Member(live),
			false
		))
		.is_empty()
	);
}

#[test]
fn keeps_live_enum_item_links() {
	use rapi_site::components::changes::{ChangedValue, change_parts};
	let grass = CX.api.enums["Material"]
		.items
		.iter()
		.find(|item| item.name == "Grass")
		.expect("Material.Grass");
	assert_eq!(
		linked_texts(&change_parts(
			&CX,
			"Material",
			ChangedValue::EnumItem(grass),
			true
		)),
		["Material", "Grass"]
	);
	assert!(
		linked_texts(&change_parts(
			&CX,
			"Material",
			ChangedValue::EnumItem(grass),
			false
		))
		.is_empty()
	);
}
