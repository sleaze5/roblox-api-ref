use indexmap::IndexMap;
use rapi_routes::{FACET_KINDS, FacetKind};

use crate::js::{compare_code_units, compare_names};
use crate::model::{Class, Enum, Facet, FacetAccess, FacetUse, Facets, Member, UseKind};

#[derive(Default)]
struct FacetIndex([IndexMap<String, Vec<FacetUse>>; 6]);

impl FacetIndex {
	fn add(&mut self, kind: FacetKind, name: &str, item: FacetUse) {
		if name.is_empty() {
			return;
		}
		self.0[kind.index()].entry(name.to_owned()).or_default().push(item);
	}
}

fn split_access(read: &[String], write: &[String]) -> Vec<(String, Option<FacetAccess>)> {
	let mut names: Vec<&String> = Vec::new();
	for name in read.iter().chain(write) {
		if !names.contains(&name) {
			names.push(name);
		}
	}
	names
		.into_iter()
		.map(|name| {
			let in_read = read.contains(name);
			let in_write = write.contains(name);
			let access = if in_read && in_write {
				None
			} else if in_read {
				Some(FacetAccess::Read)
			} else {
				Some(FacetAccess::Write)
			};
			(name.clone(), access)
		})
		.collect()
}

fn index_member(index: &mut FacetIndex, owner: &str, member: &Member) {
	let target = FacetUse {
		kind: UseKind::member(member.kind()),
		owner: owner.to_owned(),
		name: member.name.clone(),
		access: None,
	};
	if let crate::model::MemberDetail::Property { category, .. } = &member.detail {
		index.add(FacetKind::PropertyCategory, category, target.clone());
	}
	for tag in &member.tags {
		index.add(FacetKind::Tag, tag, target.clone());
	}
	let read = [member.security.name().to_owned()];
	let write = [member.write_security().name().to_owned()];
	for (name, access) in split_access(&read, &write) {
		index.add(FacetKind::Security, &name, FacetUse { access, ..target.clone() });
	}
	for (name, access) in split_access(&member.capabilities.read, &member.capabilities.write) {
		index.add(FacetKind::Capability, &name, FacetUse { access, ..target.clone() });
	}
	if member.thread_safety != "Unknown" {
		index.add(FacetKind::ThreadSafety, &member.thread_safety, target);
	}
}

fn class_use(name: &str) -> FacetUse {
	FacetUse {
		kind: UseKind::Class,
		owner: String::new(),
		name: name.to_owned(),
		access: None,
	}
}

fn compare_uses(a: &FacetUse, b: &FacetUse) -> std::cmp::Ordering {
	a.kind
		.cmp(&b.kind)
		.then_with(|| compare_names(&a.owner, &b.owner))
		.then_with(|| compare_names(&a.name, &b.name))
}

pub fn collect_facets<'a>(
	classes: impl Iterator<Item = &'a Class>,
	enums: impl Iterator<Item = &'a Enum>,
) -> Facets {
	let mut index = FacetIndex::default();
	for item in classes {
		for tag in &item.tags {
			index.add(FacetKind::Tag, tag, class_use(&item.name));
		}
		index.add(FacetKind::MemoryCategory, &item.memory_category, class_use(&item.name));
		for member in &item.members {
			index_member(&mut index, &item.name, member);
		}
	}
	for item in enums {
		for tag in &item.tags {
			index.add(
				FacetKind::Tag,
				tag,
				FacetUse {
					kind: UseKind::Enum,
					owner: String::new(),
					name: item.name.clone(),
					access: None,
				},
			);
		}
		for entry in &item.items {
			for tag in &entry.tags {
				index.add(
					FacetKind::Tag,
					tag,
					FacetUse {
						kind: UseKind::EnumItem,
						owner: item.name.clone(),
						name: entry.name.clone(),
						access: None,
					},
				);
			}
		}
	}
	let mut facets = Facets::default();
	for kind in FACET_KINDS {
		let values = std::mem::take(&mut index.0[kind.index()]);
		let mut names: Vec<String> = values.keys().cloned().collect();
		names.sort_by(|a, b| compare_names(a, b));
		let mut values = values;
		let target = facets.of_mut(kind);
		for name in names {
			let mut uses = values.swap_remove(&name).unwrap_or_default();
			uses.sort_by(compare_uses);
			target.insert(
				name.clone(),
				Facet {
					kind,
					name,
					uses,
				},
			);
		}
	}
	facets
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FacetSlugs([(IndexMap<String, String>, IndexMap<String, String>); 6]);

impl FacetSlugs {
	pub fn slug<'a>(&'a self, kind: FacetKind, name: &'a str) -> &'a str {
		self.0[kind.index()].0.get(name).map_or(name, String::as_str)
	}

	pub fn name<'a>(&'a self, kind: FacetKind, slug: &'a str) -> &'a str {
		self.0[kind.index()].1.get(slug).map_or(slug, String::as_str)
	}

	pub fn to_json(&self) -> serde_json::Value {
		let mut map = serde_json::Map::new();
		for kind in FACET_KINDS {
			let (to_slug, to_name) = &self.0[kind.index()];
			map.insert(
				kind.key().to_owned(),
				serde_json::json!({ "toSlug": to_slug, "toName": to_name }),
			);
		}
		serde_json::Value::Object(map)
	}
}

pub fn build_facet_slugs(facets: &Facets) -> FacetSlugs {
	let mut slugs = FacetSlugs::default();
	for kind in FACET_KINDS {
		let mut names: Vec<&String> = facets.of(kind).keys().collect();
		names.sort_by(|a, b| compare_code_units(a, b));
		let mut reserved: Vec<String> = names.iter().map(|name| name.to_lowercase()).collect();
		let mut seen: Vec<String> = Vec::new();
		let (to_slug, to_name) = &mut slugs.0[kind.index()];
		for name in names {
			let folded = name.to_lowercase();
			if seen.contains(&folded) {
				let mut suffix = 2;
				while reserved.contains(&format!("{folded}-{suffix}")) {
					suffix += 1;
				}
				let slug = format!("{name}-{suffix}");
				reserved.push(slug.to_lowercase());
				to_slug.insert(name.clone(), slug.clone());
				to_name.insert(slug, name.clone());
			}
			seen.push(folded);
		}
	}
	slugs
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::model::{ApiType, Capabilities, MemberDetail, Security, TypeCategory};

	fn property(name: &str, category: &str) -> Member {
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
				category: category.to_owned(),
				default: None,
				write_security: Security::None,
				can_load: true,
				can_save: true,
				simulation_access: false,
			},
		}
	}

	fn class(name: &str, superclass: Option<&str>, members: Vec<Member>) -> Class {
		Class {
			name: name.to_owned(),
			history: None,
			icon: None,
			superclass: superclass.map(str::to_owned),
			memory_category: "Instances".to_owned(),
			tags: Vec::new(),
			members,
			removed_members: None,
			subclasses: Vec::new(),
			used_by: Vec::new(),
			description: None,
		}
	}

	#[test]
	fn indexes_defining_properties_without_duplicating_inherited_uses() {
		let parent = class(
			"Parent",
			None,
			vec![property("Enabled", "Behavior"), property("Visible", "Appearance")],
		);
		let child = class(
			"Child",
			Some("Parent"),
			vec![property("Active", "Behavior"), property("Unnamed", "")],
		);
		let facets = collect_facets([parent, child].iter(), std::iter::empty());
		let categories = facets.of(FacetKind::PropertyCategory);
		assert_eq!(categories.keys().collect::<Vec<_>>(), ["Appearance", "Behavior"]);
		let behavior: Vec<_> = categories["Behavior"]
			.uses
			.iter()
			.map(|item| (item.owner.as_str(), item.name.as_str()))
			.collect();
		assert_eq!(behavior, [("Child", "Active"), ("Parent", "Enabled")]);
	}

	#[test]
	fn gives_case_insensitive_duplicates_their_own_slug() {
		let parent = class(
			"Parent",
			None,
			vec![property("A", "Animation"), property("B", "animation")],
		);
		let facets = collect_facets([parent].iter(), std::iter::empty());
		let slugs = build_facet_slugs(&facets);
		assert_eq!(slugs.slug(FacetKind::PropertyCategory, "animation"), "animation-2");
		assert_eq!(slugs.name(FacetKind::PropertyCategory, "animation-2"), "animation");
		assert_eq!(slugs.slug(FacetKind::PropertyCategory, "Animation"), "Animation");
	}

	#[test]
	fn splits_read_and_write_access() {
		assert_eq!(
			split_access(&["A".into(), "B".into()], &["B".into(), "C".into()]),
			[
				("A".to_owned(), Some(FacetAccess::Read)),
				("B".to_owned(), None),
				("C".to_owned(), Some(FacetAccess::Write)),
			]
		);
	}
}
