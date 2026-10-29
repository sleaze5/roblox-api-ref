use indexmap::IndexMap;
use rapi_routes::MemberKind;

use super::extremes::{Measured, highest, info, lowest, measured};
use crate::api::stats_text::{kind_singular, kind_slug};
use crate::model::{Api, CountKind, RecordGroup, StatRecord, StatRef};

const SERVICE: &str = "Service";

struct NameKind {
	id: String,
	plural: String,
	items: Vec<StatRef>,
}

fn name_of(item: &StatRef) -> &str {
	match item {
		StatRef::Update { version, .. } => version,
		StatRef::Removed { label, .. } => label,
		StatRef::Class { name }
		| StatRef::Enum { name }
		| StatRef::DataType { name }
		| StatRef::Member { name, .. }
		| StatRef::EnumItem { name, .. }
		| StatRef::Parameter { name, .. }
		| StatRef::Name { name, .. } => name,
	}
}

fn members_of(api: &Api, kind: MemberKind) -> Vec<StatRef> {
	api.classes
		.values()
		.flat_map(|item| {
			item.members
				.iter()
				.filter(move |member| member.kind() == kind)
				.map(move |member| StatRef::Member {
					kind,
					owner: item.name.clone(),
					name: member.name.clone(),
				})
		})
		.collect()
}

fn parameters_of(api: &Api, kind: MemberKind) -> Vec<StatRef> {
	api.classes
		.values()
		.flat_map(|item| {
			item.members
				.iter()
				.filter(move |member| member.kind() == kind)
				.flat_map(move |member| {
					member
						.parameters()
						.iter()
						.map(move |parameter| StatRef::Parameter {
							owner: item.name.clone(),
							member: member.name.clone(),
							member_kind: kind,
							name: parameter.name.clone(),
						})
				})
		})
		.collect()
}

fn enum_items(api: &Api) -> Vec<StatRef> {
	api.enums
		.values()
		.flat_map(|item| {
			item.items.iter().map(move |entry| StatRef::EnumItem {
				owner: item.name.clone(),
				name: entry.name.clone(),
			})
		})
		.collect()
}

fn kind(kind: CountKind, items: Vec<StatRef>) -> NameKind {
	NameKind {
		id: kind_slug(kind).to_owned(),
		plural: format!("{} names", kind_singular(kind)),
		items,
	}
}

fn parameter_kind(member: MemberKind, items: Vec<StatRef>) -> NameKind {
	let singular = kind_singular(CountKind::member(member));
	NameKind {
		id: format!("{}-parameter", kind_slug(CountKind::member(member))),
		plural: format!("{singular} parameter names"),
		items,
	}
}

fn kinds(api: &Api) -> Vec<NameKind> {
	vec![
		kind(
			CountKind::Class,
			api.classes
				.keys()
				.map(|name| StatRef::Class { name: name.clone() })
				.collect(),
		),
		kind(
			CountKind::Service,
			api.classes
				.values()
				.filter(|item| item.tags.iter().any(|tag| tag == SERVICE))
				.map(|item| StatRef::Class {
					name: item.name.clone(),
				})
				.collect(),
		),
		kind(CountKind::Property, members_of(api, MemberKind::Property)),
		kind(CountKind::Method, members_of(api, MemberKind::Method)),
		kind(CountKind::Event, members_of(api, MemberKind::Event)),
		kind(CountKind::Callback, members_of(api, MemberKind::Callback)),
		parameter_kind(MemberKind::Method, parameters_of(api, MemberKind::Method)),
		parameter_kind(MemberKind::Event, parameters_of(api, MemberKind::Event)),
		parameter_kind(
			MemberKind::Callback,
			parameters_of(api, MemberKind::Callback),
		),
		kind(
			CountKind::Enum,
			api.enums
				.keys()
				.map(|name| StatRef::Enum { name: name.clone() })
				.collect(),
		),
		kind(CountKind::EnumItem, enum_items(api)),
		kind(
			CountKind::DataType,
			api.data_types
				.keys()
				.map(|name| StatRef::DataType { name: name.clone() })
				.collect(),
		),
	]
}

fn length_records(kind: &NameKind) -> [StatRecord; 2] {
	let NameKind { id, plural, items } = kind;
	let singular = plural.strip_suffix('s').unwrap_or(plural);
	let measures = || {
		items
			.iter()
			.map(|item| measured(item.clone(), name_of(item).encode_utf16().count()))
			.collect()
	};
	[
		highest(
			info(
				&format!("longest-{id}-name"),
				&format!("Longest {singular}"),
				&format!("The longest {plural}, in characters."),
			),
			measures(),
		),
		lowest(
			info(
				&format!("shortest-{id}-name"),
				&format!("Shortest {singular}"),
				&format!("The shortest {plural}, in characters."),
			),
			measures(),
		),
	]
}

fn shared_names(of: CountKind, items: &[StatRef]) -> Vec<Measured> {
	let mut uses: IndexMap<&str, usize> = IndexMap::new();
	for item in items {
		*uses.entry(name_of(item)).or_default() += 1;
	}
	uses.into_iter()
		.map(|(name, count)| {
			measured(
				StatRef::Name {
					of,
					name: name.to_owned(),
				},
				count,
			)
		})
		.collect()
}

fn common_records(of: CountKind, owners: &str, items: &[StatRef]) -> [StatRecord; 2] {
	let singular = kind_singular(of);
	let slug = kind_slug(of);
	let names = shared_names(of, items);
	[
		highest(
			info(
				&format!("most-common-{slug}-name"),
				&format!("Most common {singular} name"),
				&format!("The {singular} names that the most {owners} use."),
			),
			names.clone(),
		),
		lowest(
			info(
				&format!("least-common-{slug}-name"),
				&format!("Least common {singular} name"),
				&format!("The {singular} names that the fewest {owners} use."),
			),
			names,
		),
	]
}

pub fn name_records(api: &Api) -> RecordGroup {
	let kinds = kinds(api);
	let mut records: Vec<StatRecord> = kinds.iter().flat_map(length_records).collect();
	for (of, member) in [
		(CountKind::Property, MemberKind::Property),
		(CountKind::Method, MemberKind::Method),
		(CountKind::Event, MemberKind::Event),
		(CountKind::Callback, MemberKind::Callback),
	] {
		records.extend(common_records(of, "classes", &members_of(api, member)));
	}
	records.extend(common_records(
		CountKind::EnumItem,
		"enums",
		&enum_items(api),
	));
	RecordGroup {
		id: "name-records".to_owned(),
		title: "Names".to_owned(),
		table: None,
		records,
	}
}
