use super::extremes::{Info, Measured, highest, info, lowest, measured};
use crate::model::{Api, RecordGroup, StatRecord, StatRef};

fn enum_ref(name: &str) -> StatRef {
	StatRef::Enum {
		name: name.to_owned(),
	}
}

fn both(most: Info, fewest: Info, measured: &[Measured]) -> [StatRecord; 2] {
	[
		highest(most, measured.to_vec()),
		lowest(fewest, measured.to_vec()),
	]
}

fn item_values(api: &Api) -> Vec<Measured> {
	api.enums
		.values()
		.flat_map(|item| {
			item.items.iter().map(|entry| {
				#[expect(clippy::cast_precision_loss, reason = "enum values are small")]
				let value = entry.value as f64;
				Measured {
					item: StatRef::EnumItem {
						owner: item.name.clone(),
						name: entry.name.clone(),
					},
					value,
				}
			})
		})
		.collect()
}

pub fn type_records(api: &Api) -> RecordGroup {
	let mut records = Vec::new();
	records.extend(both(
		info(
			"most-enum-items",
			"Most enum items",
			"The enums with the most items.",
		),
		info(
			"fewest-enum-items",
			"Fewest enum items",
			"The enums with the fewest items.",
		),
		&api.enums
			.values()
			.map(|item| measured(enum_ref(&item.name), item.items.len()))
			.collect::<Vec<_>>(),
	));
	records.extend(both(
		info(
			"highest-enum-item-value",
			"Highest enum item value",
			"The enum items with the highest number value.",
		),
		info(
			"lowest-enum-item-value",
			"Lowest enum item value",
			"The enum items with the lowest number value.",
		),
		&item_values(api),
	));
	records.extend(both(
		info(
			"most-used-data-type",
			"Most-used data type",
			"The data types that the most properties, parameters, and return values use.",
		),
		info(
			"least-used-data-type",
			"Least-used data type",
			"The data types that the fewest properties, parameters, and return values use.",
		),
		&api.data_types
			.values()
			.map(|item| {
				measured(
					StatRef::DataType {
						name: item.name.clone(),
					},
					item.used_by.len(),
				)
			})
			.collect::<Vec<_>>(),
	));
	records.extend(both(
		info(
			"most-used-enum",
			"Most-used enum",
			"The enums that the most properties, parameters, and return values use.",
		),
		info(
			"least-used-enum",
			"Least-used enum",
			"The enums that the fewest properties, parameters, and return values use.",
		),
		&api.enums
			.values()
			.map(|item| measured(enum_ref(&item.name), item.used_by.len()))
			.collect::<Vec<_>>(),
	));
	records.extend(both(
		info(
			"most-used-class-type",
			"Most-used class as a type",
			"The classes that the most properties, parameters, and return values use as their type.",
		),
		info(
			"least-used-class-type",
			"Least-used class as a type",
			"The classes that the fewest properties, parameters, and return values use as their type.",
		),
		&api.classes
			.values()
			.map(|item| {
				measured(
					StatRef::Class {
						name: item.name.clone(),
					},
					item.used_by.len(),
				)
			})
			.collect::<Vec<_>>(),
	));
	RecordGroup {
		id: "type-records".to_owned(),
		title: "Enums and types".to_owned(),
		table: None,
		records,
	}
}
