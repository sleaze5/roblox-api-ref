use rapi_routes::MemberKind;

use super::extremes::{highest, info, lowest, measured};
use crate::model::{Api, RecordGroup, StatRecord, StatRef};

fn name_of(item: &StatRef) -> &str {
	match item {
		StatRef::Update { version, .. } => version,
		StatRef::Removed { label, .. } => label,
		StatRef::Class { name }
		| StatRef::Enum { name }
		| StatRef::DataType { name }
		| StatRef::Member { name, .. }
		| StatRef::EnumItem { name, .. }
		| StatRef::Parameter { name, .. } => name,
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

fn kinds(api: &Api) -> Vec<(&'static str, &'static str, Vec<StatRef>)> {
	vec![
		(
			"class",
			"class names",
			api.classes
				.keys()
				.map(|name| StatRef::Class { name: name.clone() })
				.collect(),
		),
		("property", "property names", members_of(api, MemberKind::Property)),
		("method", "method names", members_of(api, MemberKind::Method)),
		("event", "event names", members_of(api, MemberKind::Event)),
		("callback", "callback names", members_of(api, MemberKind::Callback)),
		(
			"parameter",
			"parameter names",
			api.classes
				.values()
				.flat_map(|item| {
					item.members.iter().flat_map(move |member| {
						member.parameters().iter().map(move |parameter| StatRef::Parameter {
							owner: item.name.clone(),
							member: member.name.clone(),
							member_kind: member.kind(),
							name: parameter.name.clone(),
						})
					})
				})
				.collect(),
		),
		(
			"enum",
			"enum names",
			api.enums
				.keys()
				.map(|name| StatRef::Enum { name: name.clone() })
				.collect(),
		),
		(
			"enum-item",
			"enum item names",
			api.enums
				.values()
				.flat_map(|item| {
					item.items.iter().map(move |entry| StatRef::EnumItem {
						owner: item.name.clone(),
						name: entry.name.clone(),
					})
				})
				.collect(),
		),
		(
			"data-type",
			"data type names",
			api.data_types
				.keys()
				.map(|name| StatRef::DataType { name: name.clone() })
				.collect(),
		),
	]
}

fn kind_records(id: &str, plural: &str, items: &[StatRef]) -> [StatRecord; 2] {
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

pub fn name_records(api: &Api) -> RecordGroup {
	RecordGroup {
		id: "name-records".to_owned(),
		title: "Names".to_owned(),
		records: kinds(api)
			.iter()
			.flat_map(|(id, plural, items)| kind_records(id, plural, items))
			.collect(),
	}
}
