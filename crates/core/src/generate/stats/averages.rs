use super::extremes::info;
use super::member_records::return_count;
use crate::api::stats_text::{kind_plural, kind_singular, kind_slug};
use crate::js::js_round;
use crate::model::{Api, CountKind, Member, RecordGroup, StatRecord};

fn average(values: &[usize]) -> f64 {
	if values.is_empty() {
		return 0.0;
	}
	#[expect(clippy::cast_precision_loss, reason = "counts are small")]
	let mean = values.iter().sum::<usize>() as f64 / values.len() as f64;
	js_round(mean * 100.0) / 100.0
}

fn record(
	(id, label, hint): (String, String, String),
	values: &[usize],
	unit: Option<&str>,
) -> StatRecord {
	StatRecord {
		id,
		label,
		hint,
		value: average(values),
		unit: unit.map(str::to_owned),
		items: Vec::new(),
		ties: None,
	}
}

fn members_of(api: &Api, kind: CountKind) -> Vec<&Member> {
	api.classes
		.values()
		.flat_map(|item| &item.members)
		.filter(|member| CountKind::member(member.kind()) == kind)
		.collect()
}

fn length(name: &str) -> usize {
	name.encode_utf16().count()
}

pub fn average_records(api: &Api) -> RecordGroup {
	let mut records = Vec::new();
	for kind in CountKind::MEMBERS {
		let plural = kind_plural(kind);
		records.push(record(
			info(
				&format!("average-{}-members", kind_slug(kind)),
				&format!("Average {plural} per class"),
				&format!("The {plural} that a class defines itself, on average."),
			),
			&api.classes
				.values()
				.map(|item| {
					item.members
						.iter()
						.filter(|member| CountKind::member(member.kind()) == kind)
						.count()
				})
				.collect::<Vec<_>>(),
			None,
		));
	}
	for kind in [CountKind::Method, CountKind::Event, CountKind::Callback] {
		let singular = kind_singular(kind);
		records.push(record(
			info(
				&format!("average-{}-parameters", kind_slug(kind)),
				&format!("Average parameters per {singular}"),
				&format!("The parameters of a {singular}, on average."),
			),
			&members_of(api, kind)
				.iter()
				.map(|member| member.parameters().len())
				.collect::<Vec<_>>(),
			None,
		));
	}
	for kind in [CountKind::Method, CountKind::Callback] {
		let singular = kind_singular(kind);
		records.push(record(
			info(
				&format!("average-{}-return-values", kind_slug(kind)),
				&format!("Average return values per {singular}"),
				&format!("The values that a {singular} returns, on average."),
			),
			&members_of(api, kind)
				.iter()
				.filter_map(|member| member.returns().map(return_count))
				.collect::<Vec<_>>(),
			None,
		));
	}
	records.push(record(
		info(
			"average-enum-items",
			"Average items per enum",
			"The items of an enum, on average.",
		),
		&api.enums
			.values()
			.map(|item| item.items.len())
			.collect::<Vec<_>>(),
		None,
	));
	let names: Vec<(CountKind, Vec<usize>)> = vec![
		(
			CountKind::Class,
			api.classes.keys().map(|name| length(name)).collect(),
		),
		(
			CountKind::Property,
			members_of(api, CountKind::Property)
				.iter()
				.map(|member| length(&member.name))
				.collect(),
		),
		(
			CountKind::Method,
			members_of(api, CountKind::Method)
				.iter()
				.map(|member| length(&member.name))
				.collect(),
		),
		(
			CountKind::Event,
			members_of(api, CountKind::Event)
				.iter()
				.map(|member| length(&member.name))
				.collect(),
		),
		(
			CountKind::Callback,
			members_of(api, CountKind::Callback)
				.iter()
				.map(|member| length(&member.name))
				.collect(),
		),
		(
			CountKind::Enum,
			api.enums.keys().map(|name| length(name)).collect(),
		),
		(
			CountKind::EnumItem,
			api.enums
				.values()
				.flat_map(|item| item.items.iter().map(|entry| length(&entry.name)))
				.collect(),
		),
	];
	for (kind, lengths) in names {
		let singular = kind_singular(kind);
		records.push(record(
			info(
				&format!("average-{}-name-length", kind_slug(kind)),
				&format!("Average {singular} name length"),
				&format!("The length of a {singular} name, on average, in characters."),
			),
			&lengths,
			Some("characters"),
		));
	}
	RecordGroup {
		id: "average-records".to_owned(),
		title: "Averages".to_owned(),
		table: None,
		records,
	}
}
