use crate::encode_uri_component;

pub const FACET_KINDS: [FacetKind; 6] = [
	FacetKind::Tag,
	FacetKind::PropertyCategory,
	FacetKind::Security,
	FacetKind::Capability,
	FacetKind::ThreadSafety,
	FacetKind::MemoryCategory,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FacetKind {
	Tag,
	PropertyCategory,
	Security,
	Capability,
	ThreadSafety,
	MemoryCategory,
}

impl FacetKind {
	pub const fn index(self) -> usize {
		self as usize
	}

	pub const fn key(self) -> &'static str {
		match self {
			Self::Tag => "tag",
			Self::PropertyCategory => "propertyCategory",
			Self::Security => "security",
			Self::Capability => "capability",
			Self::ThreadSafety => "threadSafety",
			Self::MemoryCategory => "memoryCategory",
		}
	}

	pub const fn path(self) -> &'static str {
		match self {
			Self::Tag => "tags",
			Self::PropertyCategory => "property-categories",
			Self::Security => "security-levels",
			Self::Capability => "capabilities",
			Self::ThreadSafety => "thread-safety",
			Self::MemoryCategory => "memory-categories",
		}
	}

	pub fn from_path(path: &str) -> Option<Self> {
		FACET_KINDS.into_iter().find(|kind| kind.path() == path)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemberKind {
	Property,
	Method,
	Event,
	Callback,
}

impl MemberKind {
	pub const ALL: [Self; 4] = [Self::Property, Self::Method, Self::Event, Self::Callback];

	pub const fn name(self) -> &'static str {
		match self {
			Self::Property => "Property",
			Self::Method => "Method",
			Self::Event => "Event",
			Self::Callback => "Callback",
		}
	}

	pub const fn separator(self) -> char {
		match self {
			Self::Method => ':',
			_ => '.',
		}
	}
}

pub const HOME: &str = "/";
pub const CLASSES: &str = "/classes/";
pub const ENUMS: &str = "/enums/";
pub const INDEX: &str = "/index/";
pub const UPDATES: &str = "/updates/";
pub const STATS: &str = "/stats/";
pub const ABOUT: &str = "/about/";
pub const CONTACT: &str = "/contact/";
pub const PRIVACY: &str = "/privacy/";
pub const LLMS_TXT: &str = "/llms.txt";
pub const NOT_FOUND_MARKDOWN: &str = "/404.md";
pub const REMOVED: &str = "/index/removed/";
pub const TYPES: &str = "/index/types/";
pub const FEED: &str = "/updates.xml";

pub fn update_href(version: &str) -> String {
	format!("{UPDATES}{}/", encode_uri_component(version))
}

pub fn facet_index_href(kind: FacetKind) -> String {
	format!("{INDEX}{}/", kind.path())
}

pub fn facet_href_slug(kind: FacetKind, slug: &str) -> String {
	format!("{}{}/", facet_index_href(kind), encode_uri_component(slug))
}

pub fn class_href(name: &str) -> String {
	format!("{CLASSES}{}/", encode_uri_component(name))
}

pub fn enum_href(name: &str) -> String {
	format!("{ENUMS}{}/", encode_uri_component(name))
}

pub fn type_href(name: &str) -> String {
	format!("{TYPES}{}/", encode_uri_component(name))
}

pub fn member_anchor(kind: MemberKind, name: &str) -> String {
	format!("{}-{name}", kind.name())
}

pub fn enum_item_anchor(name: &str) -> String {
	format!("Item-{name}")
}

pub fn member_href(class_name: &str, kind: MemberKind, name: &str) -> String {
	format!("{}#{}", class_href(class_name), member_anchor(kind, name))
}

pub fn enum_item_href(enum_name: &str, item: &str) -> String {
	format!("{}#{}", enum_href(enum_name), enum_item_anchor(item))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnerKind {
	Class,
	Enum,
}

impl OwnerKind {
	pub const fn name(self) -> &'static str {
		match self {
			Self::Class => "Class",
			Self::Enum => "Enum",
		}
	}
}

pub fn owner_anchor(kind: OwnerKind, name: &str) -> String {
	format!("{}-{name}", kind.name())
}

pub fn change_anchor(kind: OwnerKind, owner: &str, name: &str) -> String {
	format!("{}.{name}", owner_anchor(kind, owner))
}

pub fn update_anchor(version: &str) -> String {
	format!("update-{version}")
}

pub fn upload_name(hash: &str) -> String {
	format!("version-{hash}")
}

pub fn markdown_href(href: &str) -> String {
	format!("{}.md", href.strip_suffix('/').unwrap_or(href))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builds_listing_and_detail_urls() {
		assert_eq!(class_href("Part"), "/classes/Part/");
		assert_eq!(enum_href("Material"), "/enums/Material/");
		assert_eq!(type_href("Array"), "/index/types/Array/");
		assert_eq!(
			facet_href_slug(FacetKind::PropertyCategory, "Body Parts"),
			"/index/property-categories/Body%20Parts/"
		);
		assert_eq!(update_href("0.744.0.7441077"), "/updates/0.744.0.7441077/");
		assert_eq!(
			member_href("Part", MemberKind::Property, "Anchored"),
			"/classes/Part/#Property-Anchored"
		);
		assert_eq!(enum_item_href("Material", "Grass"), "/enums/Material/#Item-Grass");
		assert_eq!(
			change_anchor(OwnerKind::Enum, "Material", "Old"),
			"Enum-Material.Old"
		);
		assert_eq!(markdown_href("/classes/Part/"), "/classes/Part.md");
	}

	#[test]
	fn maps_facet_paths_both_ways() {
		for kind in FACET_KINDS {
			assert_eq!(FacetKind::from_path(kind.path()), Some(kind));
		}
		assert_eq!(FacetKind::from_path("types"), None);
	}
}
