use rapi_routes::{FacetKind, class_href, enum_href, enum_item_href, member_href};

use crate::js::compare_names;
use crate::model::{Api, Facet, FacetUse, Security, UseKind};

pub const fn facet_title(kind: FacetKind) -> &'static str {
	match kind {
		FacetKind::Tag => "Tags",
		FacetKind::PropertyCategory => "Property categories",
		FacetKind::Security => "Security levels",
		FacetKind::Capability => "Capabilities",
		FacetKind::ThreadSafety => "Thread safety",
		FacetKind::MemoryCategory => "Memory categories",
	}
}

pub const fn facet_label(kind: FacetKind) -> &'static str {
	match kind {
		FacetKind::Tag => "Tag",
		FacetKind::PropertyCategory => "Property category",
		FacetKind::Security => "Security level",
		FacetKind::Capability => "Capability",
		FacetKind::ThreadSafety => "Thread safety",
		FacetKind::MemoryCategory => "Memory category",
	}
}

pub const INDEX_DESCRIPTION: &str = "Every data type, tag, property category, security level, capability, thread safety level, and memory category found in the API dump. The dump does not describe them, so each page lists what uses it.";

pub const fn facet_index_description(kind: FacetKind) -> &'static str {
	match kind {
		FacetKind::Tag => "Every tag applied to classes, members, enums, and enum items.",
		FacetKind::PropertyCategory => {
			"Every property category, as grouped in Studio’s Properties window."
		}
		FacetKind::Security => "Every security level required to read, write, or call a member.",
		FacetKind::Capability => "Every capability required by a member.",
		FacetKind::ThreadSafety => "Every thread safety level assigned to a member.",
		FacetKind::MemoryCategory => "Every memory category assigned to a class.",
	}
}

fn compare_facets(a: &Facet, b: &Facet) -> std::cmp::Ordering {
	if a.kind == FacetKind::Security
		&& let (Some(left), Some(right)) = (Security::parse(&a.name), Security::parse(&b.name))
	{
		return left.rank().cmp(&right.rank());
	}
	compare_names(&a.name, &b.name)
}

pub fn sorted_facets(api: &Api, kind: FacetKind) -> Vec<&Facet> {
	let mut facets: Vec<&Facet> = api.facets.of(kind).values().collect();
	facets.sort_by(|a, b| compare_facets(a, b));
	facets
}

pub fn facet_description(facet: &Facet) -> String {
	let name = &facet.name;
	match facet.kind {
		FacetKind::PropertyCategory => format!("Every property in the {name} category."),
		FacetKind::Tag => format!(
			"Every class, member, enum, and enum item that the API dump tags {name}."
		),
		FacetKind::Security => {
			format!("Every member that needs {name} to read, write, or call it.")
		}
		FacetKind::Capability => format!("Every member that needs the {name} capability."),
		FacetKind::ThreadSafety => format!("Every member whose thread safety is {name}."),
		FacetKind::MemoryCategory => format!(
			"Every class whose instances count toward the {name} memory category."
		),
	}
}

pub const fn use_kind_title(kind: UseKind) -> &'static str {
	match kind {
		UseKind::Class => "Classes",
		UseKind::Enum => "Enums",
		UseKind::EnumItem => "Enum items",
		UseKind::Property => "Properties",
		UseKind::Method => "Methods",
		UseKind::Event => "Events",
		UseKind::Callback => "Callbacks",
	}
}

pub fn use_sections(uses: &[FacetUse]) -> Vec<(UseKind, Vec<&FacetUse>)> {
	UseKind::ALL
		.into_iter()
		.map(|kind| (kind, uses.iter().filter(|item| item.kind == kind).collect::<Vec<_>>()))
		.filter(|(_, list)| !list.is_empty())
		.collect()
}

pub fn use_section_id(kind: UseKind) -> String {
	use_kind_title(kind).to_lowercase().replace(' ', "-")
}

pub struct UseParts<'a> {
	pub prefix: &'static str,
	pub owner: Option<&'a str>,
	pub separator: &'static str,
	pub name: &'a str,
}

pub fn use_parts(item: &FacetUse) -> UseParts<'_> {
	match item.kind {
		UseKind::Class => UseParts {
			prefix: "",
			owner: None,
			separator: "",
			name: &item.name,
		},
		UseKind::Enum => UseParts {
			prefix: "Enum.",
			owner: None,
			separator: "",
			name: &item.name,
		},
		UseKind::EnumItem => UseParts {
			prefix: "Enum.",
			owner: Some(&item.owner),
			separator: ".",
			name: &item.name,
		},
		kind => UseParts {
			prefix: "",
			owner: Some(&item.owner),
			separator: if kind == UseKind::Method { ":" } else { "." },
			name: &item.name,
		},
	}
}

pub fn use_text(item: &FacetUse) -> String {
	let parts = use_parts(item);
	format!(
		"{}{}{}{}",
		parts.prefix,
		parts.owner.unwrap_or_default(),
		parts.separator,
		parts.name
	)
}

pub fn use_href(item: &FacetUse) -> String {
	match item.kind {
		UseKind::Class => class_href(&item.name),
		UseKind::Enum => enum_href(&item.name),
		UseKind::EnumItem => enum_item_href(&item.owner, &item.name),
		kind => member_href(
			&item.owner,
			kind.member_kind().unwrap_or(rapi_routes::MemberKind::Property),
			&item.name,
		),
	}
}
