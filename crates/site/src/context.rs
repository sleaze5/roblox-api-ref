//! Everything a page needs to render.

use std::collections::HashSet;
use std::sync::OnceLock;

use rapi_core::generate::facets::FacetSlugs;
use rapi_core::model::{Api, Release, Stats, Update};
use rapi_routes::{FacetKind, class_href, enum_href, enum_item_href, member_href, type_href};

pub struct Context {
	pub api: Api,
	pub updates: Vec<Update>,
	pub stats: Stats,
	pub slugs: FacetSlugs,
	/// The URL of the icon sprite, without a fragment.
	pub sprite: String,
	/// The origin that absolute URLs use, such as `https://example.com`.
	pub origin: String,
	links: OnceLock<HashSet<String>>,
}

impl Context {
	pub fn new(
		api: Api,
		updates: Vec<Update>,
		stats: Stats,
		slugs: FacetSlugs,
		sprite: String,
		origin: String,
	) -> Self {
		Self {
			api,
			updates,
			stats,
			slugs,
			sprite,
			origin,
			links: OnceLock::new(),
		}
	}

	/// Every page and member link that exists in the current API.
	pub fn site_links(&self) -> &HashSet<String> {
		self.links.get_or_init(|| {
			let mut links = HashSet::new();
			for item in self.api.classes.values() {
				links.insert(class_href(&item.name));
				for member in &item.members {
					links.insert(member_href(&item.name, member.kind(), &member.name));
				}
			}
			for item in self.api.enums.values() {
				links.insert(enum_href(&item.name));
				for entry in &item.items {
					links.insert(enum_item_href(&item.name, &entry.name));
				}
			}
			for name in self.api.data_types.keys() {
				links.insert(type_href(name));
			}
			links
		})
	}

	pub fn release(&self) -> Release {
		Release {
			version: self.api.version.clone(),
			hash: self.api.hash.clone(),
		}
	}

	pub fn facet_href(&self, kind: FacetKind, name: &str) -> String {
		rapi_routes::facet_href_slug(kind, self.slugs.slug(kind, name))
	}

	pub fn absolute_url(&self, path: &str) -> String {
		absolute_url(&self.origin, path)
	}
}

/// Resolves a root-relative path against the origin, like `new URL(path, origin)`.
pub fn absolute_url(origin: &str, path: &str) -> String {
	format!("{origin}{path}")
}

/// The origin from `SITE_ORIGIN` or `CF_PAGES_URL`, or the dev server's.
pub fn site_origin() -> String {
	let configured = ["SITE_ORIGIN", "CF_PAGES_URL"]
		.iter()
		.find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
		.unwrap_or_else(|| "http://localhost:5173".to_owned());
	origin_of(&configured)
}

fn origin_of(url: &str) -> String {
	let Some(scheme_end) = url.find("://") else {
		return url.trim_end_matches('/').to_owned();
	};
	let rest = &url[scheme_end + 3..];
	let host_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
	let scheme = url[..scheme_end].to_ascii_lowercase();
	let mut host = rest[..host_end].to_ascii_lowercase();
	let default_port = match scheme.as_str() {
		"http" => Some(":80"),
		"https" => Some(":443"),
		_ => None,
	};
	if let Some(port) = default_port
		&& host.ends_with(port)
	{
		host.truncate(host.len() - port.len());
	}
	format!("{scheme}://{host}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn keeps_only_the_origin_of_a_configured_url() {
		assert_eq!(
			origin_of("https://Example.com/path?x#y"),
			"https://example.com"
		);
		assert_eq!(origin_of("https://example.com:443"), "https://example.com");
		assert_eq!(origin_of("http://localhost:5173/"), "http://localhost:5173");
	}
}
