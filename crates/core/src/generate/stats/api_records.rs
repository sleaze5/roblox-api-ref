use super::extremes::{highest, info, measured};
use crate::model::{Api, ApiType, Member, RecordGroup, StatRef};

fn return_count(ty: &ApiType) -> usize {
	if ty.name == "void" || ty.name == "null" {
		return 0;
	}
	match (&ty.members, ty.name.as_str()) {
		(Some(members), "Tuple") => members.len(),
		_ => 1,
	}
}

fn member_ref(owner: &str, member: &Member) -> StatRef {
	StatRef::Member {
		kind: member.kind(),
		owner: owner.to_owned(),
		name: member.name.clone(),
	}
}

fn class_ref(name: &str) -> StatRef {
	StatRef::Class {
		name: name.to_owned(),
	}
}

fn class_records(api: &Api) -> RecordGroup {
	let measure = |value: &dyn Fn(&crate::model::Class) -> usize| {
		api.classes
			.values()
			.map(|item| measured(class_ref(&item.name), value(item)))
			.collect()
	};
	RecordGroup {
		id: "class-records".to_owned(),
		title: "Classes".to_owned(),
		records: vec![
			highest(
				info(
					"most-superclasses",
					"Most superclasses",
					"The deepest classes in the class tree.",
				),
				measure(&|item| api.ancestors_of(item).len()),
			),
			highest(
				info(
					"most-subclasses",
					"Most subclasses",
					"Classes that inherit directly from this class.",
				),
				measure(&|item| item.subclasses.len()),
			),
			highest(
				info(
					"most-members",
					"Most members",
					"Members that the class defines itself.",
				),
				measure(&|item| item.members.len()),
			),
			highest(
				info(
					"most-inherited-members",
					"Most inherited members",
					"Members that the class inherits from its superclasses.",
				),
				measure(&|item| {
					api.ancestors_of(item)
						.iter()
						.map(|ancestor| ancestor.members.len())
						.sum()
				}),
			),
		],
	}
}

fn member_records(api: &Api) -> RecordGroup {
	let members: Vec<(&str, &Member)> = api
		.classes
		.values()
		.flat_map(|item| {
			item.members
				.iter()
				.map(move |member| (item.name.as_str(), member))
		})
		.collect();
	RecordGroup {
		id: "member-records".to_owned(),
		title: "Members".to_owned(),
		records: vec![
			highest(
				info(
					"most-parameters",
					"Most parameters",
					"The methods, events, and callbacks with the most parameters.",
				),
				members
					.iter()
					.filter(|(_, member)| member.value_type().is_none())
					.map(|(owner, member)| {
						measured(member_ref(owner, member), member.parameters().len())
					})
					.collect(),
			),
			highest(
				info(
					"most-return-values",
					"Most return values",
					"The methods and callbacks that return the most values.",
				),
				members
					.iter()
					.filter_map(|(owner, member)| {
						member.returns().map(|returns| {
							measured(member_ref(owner, member), return_count(returns))
						})
					})
					.collect(),
			),
		],
	}
}

fn enum_and_type_records(api: &Api) -> RecordGroup {
	let enum_ref = |name: &str| StatRef::Enum {
		name: name.to_owned(),
	};
	RecordGroup {
		id: "type-records".to_owned(),
		title: "Enums and types".to_owned(),
		records: vec![
			highest(
				info(
					"most-enum-items",
					"Most enum items",
					"The enums with the most items.",
				),
				api.enums
					.values()
					.map(|item| measured(enum_ref(&item.name), item.items.len()))
					.collect(),
			),
			highest(
				info(
					"most-used-data-type",
					"Most-used data type",
					"The data types that the most properties, parameters, and return values use.",
				),
				api.data_types
					.values()
					.map(|item| {
						measured(
							StatRef::DataType {
								name: item.name.clone(),
							},
							item.used_by.len(),
						)
					})
					.collect(),
			),
			highest(
				info(
					"most-used-enum",
					"Most-used enum",
					"The enums that the most properties, parameters, and return values use.",
				),
				api.enums
					.values()
					.map(|item| measured(enum_ref(&item.name), item.used_by.len()))
					.collect(),
			),
			highest(
				info(
					"most-used-class-type",
					"Most-used class as a type",
					"The classes that the most properties, parameters, and return values use as their type.",
				),
				api.classes
					.values()
					.map(|item| measured(class_ref(&item.name), item.used_by.len()))
					.collect(),
			),
		],
	}
}

pub fn api_records(api: &Api) -> Vec<RecordGroup> {
	vec![
		class_records(api),
		member_records(api),
		enum_and_type_records(api),
	]
}
