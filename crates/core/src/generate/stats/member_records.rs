use super::extremes::{Measured, Pair, highest, info, measured, pair_by_kind};
use crate::api::stats_text::{kind_singular, kind_slug};
use crate::model::{Api, ApiType, CountKind, Member, RecordGroup, StatRef};

const WITH_PARAMETERS: [CountKind; 3] = [CountKind::Method, CountKind::Event, CountKind::Callback];

const WITH_RETURNS: [CountKind; 2] = [CountKind::Method, CountKind::Callback];

pub fn return_count(ty: &ApiType) -> usize {
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

fn measure(api: &Api, value: impl Fn(&Member) -> Option<usize>) -> Vec<Measured> {
	api.classes
		.values()
		.flat_map(|item| {
			item.members.iter().filter_map(|member| {
				value(member).map(|value| measured(member_ref(&item.name, member), value))
			})
		})
		.collect()
}

pub fn member_records(api: &Api) -> RecordGroup {
	let parameters = measure(api, |member| {
		member
			.value_type()
			.is_none()
			.then(|| member.parameters().len())
	});
	let returns = measure(api, |member| member.returns().map(return_count));
	let mut records = pair_by_kind(
		&WITH_PARAMETERS,
		&parameters,
		&Pair {
			high: &|kind| {
				info(
					&format!("most-{}-parameters", kind_slug(kind)),
					&format!("Most {} parameters", kind_singular(kind)),
					&format!("The {}s with the most parameters.", kind_singular(kind)),
				)
			},
			low: Some(&|kind| {
				info(
					&format!("fewest-{}-parameters", kind_slug(kind)),
					&format!("Fewest {} parameters", kind_singular(kind)),
					&format!("The {}s with the fewest parameters.", kind_singular(kind)),
				)
			}),
			unit: None,
		},
	);
	records.extend(pair_by_kind(
		&WITH_RETURNS,
		&returns,
		&Pair {
			high: &|kind| {
				info(
					&format!("most-{}-return-values", kind_slug(kind)),
					&format!("Most {} return values", kind_singular(kind)),
					&format!("The {}s that return the most values.", kind_singular(kind)),
				)
			},
			low: Some(&|kind| {
				info(
					&format!("fewest-{}-return-values", kind_slug(kind)),
					&format!("Fewest {} return values", kind_singular(kind)),
					&format!(
						"The {}s that return the fewest values.",
						kind_singular(kind)
					),
				)
			}),
			unit: None,
		},
	));
	records.push(highest(
		info(
			"most-optional-parameters",
			"Most optional parameters",
			"The methods with the most parameters that have a default value.",
		),
		measure(api, |member| {
			(CountKind::member(member.kind()) == CountKind::Method).then(|| {
				member
					.parameters()
					.iter()
					.filter(|parameter| parameter.default.is_some())
					.count()
			})
		}),
	));
	RecordGroup {
		id: "member-records".to_owned(),
		title: "Members".to_owned(),
		table: None,
		records,
	}
}
