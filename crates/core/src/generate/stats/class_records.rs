use super::extremes::{Info, Measured, highest, info, lowest, measured};
use crate::api::stats_text::{kind_plural, kind_slug};
use crate::model::{Api, Class, CountKind, RecordGroup, StatRecord, StatRef};

const DEPRECATED: &str = "Deprecated";

fn class_ref(name: &str) -> StatRef {
	StatRef::Class {
		name: name.to_owned(),
	}
}

fn measure(api: &Api, value: impl Fn(&Class) -> usize) -> Vec<Measured> {
	api.classes
		.values()
		.map(|item| measured(class_ref(&item.name), value(item)))
		.collect()
}

fn both(most: Info, fewest: Info, measured: &[Measured]) -> [StatRecord; 2] {
	[
		highest(most, measured.to_vec()),
		lowest(fewest, measured.to_vec()),
	]
}

fn of_member_kind(item: &Class, kind: CountKind) -> usize {
	item.members
		.iter()
		.filter(|member| CountKind::member(member.kind()) == kind)
		.count()
}

fn member_kind_records(api: &Api, kind: CountKind) -> Vec<StatRecord> {
	let plural = kind_plural(kind);
	let slug = kind_slug(kind);
	let measured = measure(api, |item| of_member_kind(item, kind));
	let [most, fewest] = both(
		info(
			&format!("most-{slug}-members"),
			&format!("Most {plural}"),
			&format!("The classes that define the most {plural} themselves."),
		),
		info(
			&format!("fewest-{slug}-members"),
			&format!("Fewest {plural}"),
			&format!("The classes that define the fewest {plural} themselves."),
		),
		&measured,
	);
	vec![most, fewest]
}

pub fn class_records(api: &Api) -> RecordGroup {
	let mut records = Vec::new();
	records.extend(both(
		info(
			"most-superclasses",
			"Most superclasses",
			"The deepest classes in the class tree.",
		),
		info(
			"fewest-superclasses",
			"Fewest superclasses",
			"The classes at the root of the class tree.",
		),
		&measure(api, |item| api.ancestors_of(item).len()),
	));
	records.extend(both(
		info(
			"most-subclasses",
			"Most subclasses",
			"Classes that inherit directly from this class.",
		),
		info(
			"fewest-subclasses",
			"Fewest subclasses",
			"The classes that the fewest classes inherit from directly.",
		),
		&measure(api, |item| item.subclasses.len()),
	));
	for kind in CountKind::MEMBERS {
		records.extend(member_kind_records(api, kind));
	}
	records.extend(both(
		info(
			"most-inherited-members",
			"Most inherited members",
			"Members that the class inherits from its superclasses.",
		),
		info(
			"fewest-inherited-members",
			"Fewest inherited members",
			"The classes that inherit the fewest members from their superclasses.",
		),
		&measure(api, |item| {
			api.ancestors_of(item)
				.iter()
				.map(|ancestor| ancestor.members.len())
				.sum()
		}),
	));
	records.push(highest(
		info(
			"most-deprecated-members",
			"Most deprecated members",
			"The classes that define the most deprecated members themselves.",
		),
		measure(api, |item| {
			item.members
				.iter()
				.filter(|member| member.tags.iter().any(|tag| tag == DEPRECATED))
				.count()
		}),
	));
	RecordGroup {
		id: "class-records".to_owned(),
		title: "Classes".to_owned(),
		table: None,
		records,
	}
}
