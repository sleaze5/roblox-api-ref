use maud::{Markup, html};
use rapi_core::api::facets::{
	facet_title, sorted_facets, use_href, use_kind_title, use_parts, use_section_id, use_text,
};
use rapi_core::icons::Icon;
use rapi_core::js::compare_names;
use rapi_core::model::{DataType, DataTypeCategory, FacetAccess, FacetUse, RemovedOwner, UseKind};
use rapi_routes::{
	FACET_KINDS, FacetKind, OwnerKind, REMOVED, TYPES, facet_index_href, owner_anchor, type_href,
	update_href,
};

use crate::components::members::kind_icon;
use crate::context::Context;
use crate::html::{IconSize, class_icon_style, icon};
use crate::layout::section_sidebar;

pub const fn data_type_category_title(category: DataTypeCategory) -> &'static str {
	match category {
		DataTypeCategory::Primitive => "Primitive types",
		DataTypeCategory::Group => "Group types",
		DataTypeCategory::DataType => "Roblox types",
	}
}

pub fn sorted_data_types(cx: &Context) -> Vec<&DataType> {
	let mut types: Vec<&DataType> = cx.api.data_types.values().collect();
	types.sort_by(|a, b| compare_names(&a.name, &b.name));
	types
}

pub const fn facet_icon(kind: FacetKind) -> Icon {
	match kind {
		FacetKind::Tag => Icon::Tag,
		FacetKind::PropertyCategory => Icon::Folders,
		FacetKind::Security => Icon::Shield,
		FacetKind::Capability => Icon::PuzzlePiece,
		FacetKind::ThreadSafety => Icon::GitFork,
		FacetKind::MemoryCategory => Icon::Memory,
	}
}

pub const fn use_kind_icon(kind: UseKind) -> Icon {
	match kind {
		UseKind::Class => Icon::Cube,
		UseKind::Enum => Icon::ListBullets,
		UseKind::EnumItem => Icon::Hash,
		UseKind::Property => kind_icon(rapi_routes::MemberKind::Property),
		UseKind::Method => kind_icon(rapi_routes::MemberKind::Method),
		UseKind::Event => kind_icon(rapi_routes::MemberKind::Event),
		UseKind::Callback => kind_icon(rapi_routes::MemberKind::Callback),
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndexGroup {
	Type,
	Facet(FacetKind),
	Removed,
}

impl IndexGroup {
	const fn key(self) -> &'static str {
		match self {
			Self::Type => "type",
			Self::Facet(kind) => kind.key(),
			Self::Removed => "removed",
		}
	}
}

pub struct IndexCurrent<'a> {
	pub group: IndexGroup,
	pub name: Option<&'a str>,
}

struct Leaf {
	name: String,
	href: String,
}

struct Branch {
	title: String,
	href: Option<String>,
	leaves: Vec<Leaf>,
	branches: Vec<Branch>,
}

fn data_type_group(cx: &Context) -> Branch {
	let types = sorted_data_types(cx);
	Branch {
		title: "Data types".to_owned(),
		href: Some(TYPES.to_owned()),
		leaves: Vec::new(),
		branches: DataTypeCategory::ALL
			.into_iter()
			.map(|category| Branch {
				title: data_type_category_title(category).to_owned(),
				href: None,
				leaves: types
					.iter()
					.filter(|item| item.category == category)
					.map(|item| Leaf {
						name: item.name.clone(),
						href: type_href(&item.name),
					})
					.collect(),
				branches: Vec::new(),
			})
			.collect(),
	}
}

fn removed_leaves(kind: OwnerKind, owners: &[RemovedOwner]) -> Vec<Leaf> {
	owners
		.iter()
		.map(|owner| Leaf {
			name: owner.name.clone(),
			href: format!(
				"{}#{}",
				update_href(&owner.removed.version),
				owner_anchor(kind, &owner.name)
			),
		})
		.collect()
}

fn removed_group(cx: &Context) -> Branch {
	let branch = |title: &str, leaves| Branch {
		title: title.to_owned(),
		href: None,
		leaves,
		branches: Vec::new(),
	};
	Branch {
		title: "Removed".to_owned(),
		href: Some(REMOVED.to_owned()),
		leaves: Vec::new(),
		branches: vec![
			branch(
				"Classes",
				removed_leaves(OwnerKind::Class, cx.api.removed_classes()),
			),
			branch(
				"Enums",
				removed_leaves(OwnerKind::Enum, cx.api.removed_enums()),
			),
		],
	}
}

fn groups_of(cx: &Context) -> Vec<(IndexGroup, Branch)> {
	let mut groups = vec![(IndexGroup::Type, data_type_group(cx))];
	groups.extend(FACET_KINDS.into_iter().map(|kind| {
		(
			IndexGroup::Facet(kind),
			Branch {
				title: facet_title(kind).to_owned(),
				href: Some(facet_index_href(kind)),
				leaves: sorted_facets(&cx.api, kind)
					.into_iter()
					.map(|facet| Leaf {
						name: facet.name.clone(),
						href: cx.facet_href(kind, &facet.name),
					})
					.collect(),
				branches: Vec::new(),
			},
		)
	}));
	groups.push((IndexGroup::Removed, removed_group(cx)));
	groups
}

fn holds(branch: &Branch, name: &str) -> bool {
	branch.leaves.iter().any(|leaf| leaf.name == name)
		|| branch.branches.iter().any(|child| holds(child, name))
}

fn branch_node(branch: &Branch, depth: usize, current: Option<&str>, active: bool) -> Markup {
	let open = active && current.is_none_or(|name| holds(branch, name));
	html! {
		details open[open] {
			summary style={ "--depth: " (depth) } {
				@match &branch.href {
					None => span { (branch.title) },
					Some(href) => a href=(href) aria-current=[(active && current.is_none()).then_some("page")] {
						span { (branch.title) }
					},
				}
			}
			ul {
				@for child in &branch.branches {
					li { (branch_node(child, depth + 1, current, active)) }
				}
				@for leaf in &branch.leaves {
					li {
						a href=(leaf.href) aria-current=[(current == Some(leaf.name.as_str())).then_some("page")] {
							span { (leaf.name) }
						}
					}
				}
			}
		}
	}
}

pub fn index_sidebar(cx: &Context, current: Option<&IndexCurrent>) -> Markup {
	let tree = html! {
		ul id="sidebar-tree" data-tree="" data-sticky-tree="" class="tree-sticky class-tree tree-index-icons" {
			@for (group, branch) in groups_of(cx) {
				@let active = current.is_some_and(|current| current.group == group);
				li data-group=(group.key()) {
					(branch_node(&branch, 0, if active { current.and_then(|current| current.name) } else { None }, active))
				}
			}
		}
	};
	section_sidebar(cx, "Filter the index", "sidebar-tree", &tree)
}

const fn name_tone(kind: UseKind) -> &'static str {
	match kind {
		UseKind::Class | UseKind::Enum => "text-code-type",
		UseKind::EnumItem => "text-code-constant",
		_ => "text-fg-bright",
	}
}

const fn access_name(access: FacetAccess) -> &'static str {
	match access {
		FacetAccess::Read => "Read",
		FacetAccess::Write => "Write",
	}
}

fn facet_use_row(cx: &Context, item: &FacetUse) -> Markup {
	let parts = use_parts(item);
	let icon_index = if item.kind == UseKind::Class {
		cx.api.classes.get(&item.name).and_then(|class| class.icon)
	} else {
		None
	};
	html! {
		li data-name=(use_text(item)) {
			a href=(use_href(item)) class="flex h-7 items-center gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover" {
				span aria-hidden="true" data-kind=(item.kind.name()) style=[class_icon_style(icon_index)] class="kind-icon text-fg-muted" {}
				span class="truncate" {
					(parts.prefix)
					@if let Some(owner) = parts.owner { span class="text-code-type" { (owner) } }
					(parts.separator)
					span class=(name_tone(item.kind)) { (parts.name) }
				}
				@if let Some(access) = item.access {
					span class="ml-auto shrink-0 font-sans text-xs text-fg-muted" { (access_name(access)) " access" }
				}
			}
		}
	}
}

pub fn facet_use_sections(
	cx: &Context,
	id: &str,
	sections: &[(UseKind, Vec<&FacetUse>)],
) -> Markup {
	html! {
		div id=(id) class="flex flex-col gap-8" {
			@for (kind, uses) in sections {
				@let section_id = use_section_id(*kind);
				section id=(section_id) data-list-section="" aria-labelledby={ (section_id) "-title" } class="flex flex-col gap-3" {
					h2 id={ (section_id) "-title" } class="flex items-center gap-2 text-lg font-semibold" {
						(icon(cx, use_kind_icon(*kind), IconSize::S20, "text-fg-muted"))
						(use_kind_title(*kind))
						span class="text-sm font-normal text-fg-faint" { (uses.len()) }
					}
					ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2" {
						@for item in uses { (facet_use_row(cx, item)) }
					}
				}
			}
		}
	}
}
