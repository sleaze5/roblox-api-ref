//! Titles, breadcrumbs, and schema.org types for each route.

use rapi_core::api::facets::{facet_label, facet_title};
use rapi_routes::{
	CLASSES, ENUMS, HOME, INDEX, REMOVED, STATS, TYPES, UPDATES, class_href, enum_href,
	facet_index_href, type_href, update_href,
};

use crate::context::Context;
use crate::routes::{IndexList, Route};
use crate::site_pages::site_page;

pub const SITE_NAME: &str = "Roblox API Reference";
pub const SITE_TAGLINE: &str = "Unofficial Roblox Engine API reference.";
pub const SITE_SUMMARY: &str = "Search and browse every class, member, enum, and data type, and see what changed in each Roblox Studio build.";
pub const SITE_DESCRIPTION: &str = "Unofficial Roblox Engine API reference. Search and browse every class, member, enum, and data type, and see what changed in each Roblox Studio build.";
pub const REPOSITORY_URL: &str = "https://github.com/sleaze5/roblox-api-ref";
pub const ISSUES_URL: &str = "https://github.com/sleaze5/roblox-api-ref/issues";
pub const COMMITS_URL: &str = "https://github.com/sleaze5/roblox-api-ref/commits/main/";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaType {
	WebSite,
	CollectionPage,
	TechArticle,
	AboutPage,
	ContactPage,
	WebPage,
}

impl SchemaType {
	pub const fn name(self) -> &'static str {
		match self {
			Self::WebSite => "WebSite",
			Self::CollectionPage => "CollectionPage",
			Self::TechArticle => "TechArticle",
			Self::AboutPage => "AboutPage",
			Self::ContactPage => "ContactPage",
			Self::WebPage => "WebPage",
		}
	}
}

pub fn brand_title(page: Option<&str>) -> String {
	page.map_or_else(
		|| SITE_NAME.to_owned(),
		|page| format!("{page} | {SITE_NAME}"),
	)
}

const fn index_list_title(list: IndexList) -> &'static str {
	match list {
		IndexList::Types => "Data types",
		IndexList::Removed => "Removed",
	}
}

fn page_name(route: &Route) -> Option<String> {
	Some(match route {
		Route::Home => return None,
		Route::Class(item) => item.name.clone(),
		Route::Type(item) => item.name.clone(),
		Route::Enum(item) => format!("Enum.{}", item.name),
		Route::Facet(facet) => format!("{}: {}", facet_label(facet.kind), facet.name),
		Route::Update(update) => format!("Update {}", update.version),
		Route::Classes => "Classes".to_owned(),
		Route::Enums => "Enums".to_owned(),
		Route::Index => "Index".to_owned(),
		Route::IndexList(list) => index_list_title(*list).to_owned(),
		Route::FacetIndex(kind) => facet_title(*kind).to_owned(),
		Route::Updates => "Updates".to_owned(),
		Route::Stats => "Statistics".to_owned(),
		Route::Site(name) => site_page(*name).title.to_owned(),
		Route::NotFound => "Page not found".to_owned(),
	})
}

pub fn page_title(route: &Route) -> String {
	brand_title(page_name(route).as_deref())
}

pub fn schema_type(route: &Route) -> Option<SchemaType> {
	match route {
		Route::Home => Some(SchemaType::WebSite),
		Route::Class(_) | Route::Enum(_) | Route::Type(_) | Route::Update(_) => {
			Some(SchemaType::TechArticle)
		}
		Route::Stats => Some(SchemaType::WebPage),
		Route::Site(name) => Some(site_page(*name).schema_type),
		Route::NotFound => None,
		_ => Some(SchemaType::CollectionPage),
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Breadcrumb {
	pub name: String,
	pub href: String,
}

fn crumb(name: impl Into<String>, href: impl Into<String>) -> Breadcrumb {
	Breadcrumb {
		name: name.into(),
		href: href.into(),
	}
}

pub fn breadcrumbs(cx: &Context, route: &Route) -> Vec<Breadcrumb> {
	let home = || crumb(SITE_NAME, HOME);
	let index = || crumb("Index", INDEX);
	match route {
		Route::Home | Route::NotFound => Vec::new(),
		Route::Update(update) => vec![
			home(),
			crumb("Updates", UPDATES),
			crumb(&update.version, update_href(&update.version)),
		],
		Route::Class(item) => vec![
			home(),
			crumb("Classes", CLASSES),
			crumb(&item.name, class_href(&item.name)),
		],
		Route::Enum(item) => vec![
			home(),
			crumb("Enums", ENUMS),
			crumb(format!("Enum.{}", item.name), enum_href(&item.name)),
		],
		Route::Type(item) => vec![
			home(),
			index(),
			crumb("Data types", TYPES),
			crumb(&item.name, type_href(&item.name)),
		],
		Route::Facet(facet) => vec![
			home(),
			index(),
			crumb(facet_title(facet.kind), facet_index_href(facet.kind)),
			crumb(&facet.name, cx.facet_href(facet.kind, &facet.name)),
		],
		Route::Classes => vec![home(), crumb("Classes", CLASSES)],
		Route::Enums => vec![home(), crumb("Enums", ENUMS)],
		Route::Index => vec![home(), index()],
		Route::IndexList(IndexList::Types) => vec![home(), index(), crumb("Data types", TYPES)],
		Route::IndexList(IndexList::Removed) => vec![home(), index(), crumb("Removed", REMOVED)],
		Route::Updates => vec![home(), crumb("Updates", UPDATES)],
		Route::Stats => vec![home(), crumb("Statistics", STATS)],
		Route::Site(name) => {
			let page = site_page(*name);
			vec![home(), crumb(page.title, page.href)]
		}
		Route::FacetIndex(kind) => vec![
			home(),
			index(),
			crumb(facet_title(*kind), facet_index_href(*kind)),
		],
	}
}
