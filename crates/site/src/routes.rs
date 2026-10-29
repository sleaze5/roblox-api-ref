use rapi_core::model::{Class, DataType, Enum, Facet, Update};
use rapi_routes::{FACET_KINDS, FacetKind, decode_uri_component, encode_uri_component};

use crate::context::Context;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexList {
	Types,
	Removed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SitePageName {
	About,
	Contact,
	Privacy,
}

#[derive(Clone, Copy)]
pub enum Route<'a> {
	Home,
	Classes,
	Enums,
	Index,
	IndexList(IndexList),
	FacetIndex(FacetKind),
	Class(&'a Class),
	Enum(&'a Enum),
	Type(&'a DataType),
	Facet(&'a Facet),
	Updates,
	Stats,
	Update(&'a Update),
	Site(SitePageName),
	NotFound,
}

const TOP_PAGES: [(&str, Route<'static>); 8] = [
	("classes", Route::Classes),
	("enums", Route::Enums),
	("index", Route::Index),
	("updates", Route::Updates),
	("stats", Route::Stats),
	("about", Route::Site(SitePageName::About)),
	("contact", Route::Site(SitePageName::Contact)),
	("privacy", Route::Site(SitePageName::Privacy)),
];

pub fn route_paths(cx: &Context) -> Vec<String> {
	let api = &cx.api;
	let mut paths = vec!["/".to_owned()];
	paths.extend(TOP_PAGES.iter().map(|(name, _)| format!("/{name}/")));
	paths.push("/404".to_owned());
	let item = |section: &str, name: &str| format!("/{section}/{}/", encode_uri_component(name));
	paths.extend(api.classes.keys().map(|name| item("classes", name)));
	paths.extend(api.enums.keys().map(|name| item("enums", name)));
	paths.extend(
		cx.updates
			.iter()
			.map(|update| item("updates", &update.version)),
	);
	paths.push("/index/types/".to_owned());
	paths.push("/index/removed/".to_owned());
	paths.extend(api.data_types.keys().map(|name| item("index/types", name)));
	for kind in FACET_KINDS {
		paths.push(format!("/index/{}/", kind.path()));
		paths.extend(api.facets.of(kind).keys().map(|name| {
			format!(
				"/index/{}/{}/",
				kind.path(),
				encode_uri_component(cx.slugs.slug(kind, name))
			)
		}));
	}
	paths
}

fn item_route<'a>(cx: &'a Context, section: &str, name: &str) -> Route<'a> {
	match section {
		"classes" => cx
			.api
			.classes
			.get(name)
			.map_or(Route::NotFound, Route::Class),
		"enums" => cx.api.enums.get(name).map_or(Route::NotFound, Route::Enum),
		"updates" => cx
			.updates
			.iter()
			.find(|update| update.version == name)
			.map_or(Route::NotFound, Route::Update),
		_ => Route::NotFound,
	}
}

fn index_route<'a>(cx: &'a Context, section: &str, name: Option<&str>) -> Route<'a> {
	match (section, name) {
		("removed", None) => Route::IndexList(IndexList::Removed),
		("types", None) => Route::IndexList(IndexList::Types),
		("types", Some(name)) => cx
			.api
			.data_types
			.get(name)
			.map_or(Route::NotFound, Route::Type),
		_ => match (FacetKind::from_path(section), name) {
			(None, _) => Route::NotFound,
			(Some(kind), None) => Route::FacetIndex(kind),
			(Some(kind), Some(slug)) => cx
				.api
				.facets
				.of(kind)
				.get(cx.slugs.name(kind, slug))
				.map_or(Route::NotFound, Route::Facet),
		},
	}
}

pub fn match_route<'a>(cx: &'a Context, path: &str) -> Route<'a> {
	let segments: Vec<String> = path
		.split('/')
		.filter(|segment| !segment.is_empty())
		.map(|segment| decode_uri_component(segment).unwrap_or_else(|| segment.to_owned()))
		.collect();
	let section = segments.first().map_or("", String::as_str);
	let name = segments.get(1).map(String::as_str);
	match segments.len() {
		0 => Route::Home,
		1 => TOP_PAGES
			.iter()
			.find(|(top, _)| *top == section)
			.map_or(Route::NotFound, |(_, route)| *route),
		2 | 3 if section == "index" => index_route(
			cx,
			name.unwrap_or_default(),
			segments.get(2).map(String::as_str),
		),
		2 => item_route(cx, section, name.unwrap_or_default()),
		_ => Route::NotFound,
	}
}
