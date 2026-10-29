use std::collections::HashSet;

use rapi_routes::MemberKind;

use crate::model::{Api, CountKind, CountRow, EntryValues, Member, TypeCategory, Update};

fn removed_data_types(api: &Api, updates: &[Update]) -> usize {
	let mut names: HashSet<&str> = HashSet::new();
	for update in updates {
		for owner in &update.owners {
			for entry in &owner.entries {
				let EntryValues::Member { before, after } = &entry.values else {
					continue;
				};
				for member in [before, after].into_iter().flatten() {
					for ty in member.types() {
						if matches!(
							ty.category,
							TypeCategory::Primitive | TypeCategory::DataType | TypeCategory::Group
						) && !api.data_types.contains_key(&ty.name)
						{
							names.insert(&ty.name);
						}
					}
				}
			}
		}
	}
	names.len()
}

fn to_u32(value: usize) -> u32 {
	u32::try_from(value).unwrap_or(u32::MAX)
}

pub fn count_stats(api: &Api, updates: &[Update]) -> Vec<CountRow> {
	let members: Vec<&Member> = api
		.classes
		.values()
		.flat_map(|item| &item.members)
		.collect();
	let removed_members: Vec<&Member> = api
		.classes
		.values()
		.flat_map(|item| item.removed_members.iter().flatten())
		.map(|removed| &removed.value)
		.collect();
	let of_kind = |list: &[&Member], kind: MemberKind| {
		to_u32(list.iter().filter(|member| member.kind() == kind).count())
	};
	CountKind::ALL
		.into_iter()
		.map(|kind| {
			let (current, removed) = match kind {
				CountKind::Class => (
					to_u32(api.classes.len()),
					to_u32(api.removed_classes().len()),
				),
				CountKind::Enum => (to_u32(api.enums.len()), to_u32(api.removed_enums().len())),
				CountKind::EnumItem => (
					to_u32(api.enums.values().map(|item| item.items.len()).sum()),
					to_u32(
						api.enums
							.values()
							.map(|item| item.removed_items.as_ref().map_or(0, Vec::len))
							.sum(),
					),
				),
				CountKind::DataType => (
					to_u32(api.data_types.len()),
					to_u32(removed_data_types(api, updates)),
				),
				CountKind::Property => (
					of_kind(&members, MemberKind::Property),
					of_kind(&removed_members, MemberKind::Property),
				),
				CountKind::Method => (
					of_kind(&members, MemberKind::Method),
					of_kind(&removed_members, MemberKind::Method),
				),
				CountKind::Event => (
					of_kind(&members, MemberKind::Event),
					of_kind(&removed_members, MemberKind::Event),
				),
				CountKind::Callback => (
					of_kind(&members, MemberKind::Callback),
					of_kind(&removed_members, MemberKind::Callback),
				),
			};
			CountRow {
				kind,
				current,
				removed,
			}
		})
		.collect()
}
