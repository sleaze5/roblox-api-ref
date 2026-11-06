use rapi_core::model::{Class, DataType, Enum, Facet, Update};
use rapi_routes::{
	CLASSES, ENUMS, FACET_KINDS, FacetKind, HOME, INDEX, NOT_FOUND, REMOVED, STATS, TYPES, UPDATES,
	class_href, decode_uri_component, encode_uri_component, enum_href, facet_index_href, type_href,
	update_href,
};

use crate::context::Context;
use crate::site_pages::site_page;

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

impl Route<'_> {
	pub fn href(&self, cx: &Context) -> String {
		match self {
			Self::Home => HOME.to_owned(),
			Self::Classes => CLASSES.to_owned(),
			Self::Enums => ENUMS.to_owned(),
			Self::Index => INDEX.to_owned(),
			Self::IndexList(IndexList::Types) => TYPES.to_owned(),
			Self::IndexList(IndexList::Removed) => REMOVED.to_owned(),
			Self::FacetIndex(kind) => facet_index_href(*kind),
			Self::Class(item) => class_href(&item.name),
			Self::Enum(item) => enum_href(&item.name),
			Self::Type(item) => type_href(&item.name),
			Self::Facet(facet) => cx.facet_href(facet.kind, &facet.name),
			Self::Updates => UPDATES.to_owned(),
			Self::Stats => STATS.to_owned(),
			Self::Update(update) => update_href(&update.version),
			Self::Site(name) => site_page(*name).href.to_owned(),
			Self::NotFound => NOT_FOUND.to_owned(),
		}
	}
}

pub fn all_routes(cx: &Context) -> Vec<Route<'_>> {
	let api = &cx.api;
	let mut routes = vec![
		Route::Home,
		Route::Classes,
		Route::Enums,
		Route::Index,
		Route::Updates,
		Route::Stats,
		Route::Site(SitePageName::About),
		Route::Site(SitePageName::Contact),
		Route::Site(SitePageName::Privacy),
		Route::NotFound,
	];
	routes.extend(api.classes.values().map(Route::Class));
	routes.extend(api.enums.values().map(Route::Enum));
	routes.extend(cx.updates.iter().map(Route::Update));
	routes.push(Route::IndexList(IndexList::Types));
	routes.push(Route::IndexList(IndexList::Removed));
	routes.extend(api.data_types.values().map(Route::Type));
	for kind in FACET_KINDS {
		routes.push(Route::FacetIndex(kind));
		routes.extend(api.facets.of(kind).values().map(Route::Facet));
	}
	routes
}

fn canonical_path(path: &str) -> String {
	let segments: Vec<String> = path
		.split('/')
		.filter(|segment| !segment.is_empty())
		.map(|segment| {
			decode_uri_component(segment)
				.map_or_else(|| segment.to_owned(), |name| encode_uri_component(&name))
		})
		.collect();
	if segments.is_empty() {
		HOME.to_owned()
	} else {
		format!("/{}/", segments.join("/"))
	}
}

pub fn match_route<'a>(cx: &'a Context, path: &str) -> Route<'a> {
	let index = cx.route_index.get_or_init(|| {
		all_routes(cx)
			.iter()
			.enumerate()
			.filter(|(_, route)| !matches!(route, Route::NotFound))
			.map(|(index, route)| (route.href(cx), index))
			.collect()
	});
	index
		.get(&canonical_path(path))
		.map_or(Route::NotFound, |&found| all_routes(cx)[found])
}
