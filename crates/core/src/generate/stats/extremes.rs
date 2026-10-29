use crate::model::{CountKind, KindRow, StatRecord, StatRef, Ties};

const LISTED_TIES: usize = 40;

#[derive(Clone)]
pub struct Measured {
	pub item: StatRef,
	pub value: f64,
}

pub type Info = (String, String, String);

pub fn info(id: &str, label: &str, hint: &str) -> Info {
	(id.to_owned(), label.to_owned(), hint.to_owned())
}

fn ties_of(items: &mut Vec<StatRef>) -> Option<Ties> {
	if items.len() <= LISTED_TIES {
		return None;
	}
	let ties = Ties {
		count: u32::try_from(items.len()).unwrap_or(u32::MAX),
		kind: items[0].kind_name().to_owned(),
	};
	items.clear();
	Some(ties)
}

fn extreme(
	(id, label, hint): Info,
	measured: Vec<Measured>,
	beats: impl Fn(f64, f64) -> bool,
) -> StatRecord {
	let mut value: Option<f64> = None;
	let mut items: Vec<StatRef> = Vec::new();
	for entry in measured {
		match value {
			Some(best) if !beats(entry.value, best) =>
			{
				#[expect(clippy::float_cmp, reason = "ties are exact integer counts")]
				if entry.value == best {
					items.push(entry.item);
				}
			}
			_ => {
				value = Some(entry.value);
				items = vec![entry.item];
			}
		}
	}
	let ties = ties_of(&mut items);
	StatRecord {
		id,
		label,
		hint,
		value: value.unwrap_or(0.0),
		unit: None,
		items,
		ties,
	}
}

pub fn highest(info: Info, measured: Vec<Measured>) -> StatRecord {
	extreme(info, measured, |value, best| value > best)
}

pub fn lowest(info: Info, measured: Vec<Measured>) -> StatRecord {
	extreme(info, measured, |value, best| value < best)
}

pub fn count(info: Info, value: usize, unit: Option<&str>) -> StatRecord {
	let (id, label, hint) = info;
	#[expect(clippy::cast_precision_loss, reason = "counts are small")]
	let value = value as f64;
	StatRecord {
		id,
		label,
		hint,
		value,
		unit: unit.map(str::to_owned),
		items: Vec::new(),
		ties: None,
	}
}

pub fn with_unit(mut record: StatRecord, unit: &str) -> StatRecord {
	record.unit = Some(unit.to_owned());
	record
}

pub fn measured(item: StatRef, value: usize) -> Measured {
	#[expect(clippy::cast_precision_loss, reason = "counts are small")]
	let value = value as f64;
	Measured { item, value }
}

pub fn of_kind(measured: &[Measured], kind: CountKind) -> Vec<Measured> {
	measured
		.iter()
		.filter(|entry| entry.item.count_kind() == Some(kind))
		.cloned()
		.collect()
}

pub fn nonzero(measured: Vec<Measured>) -> Vec<Measured> {
	measured
		.into_iter()
		.filter(|entry| entry.value > 0.0)
		.collect()
}

pub struct Pair<'a> {
	pub high: &'a dyn Fn(CountKind) -> Info,
	pub low: Option<&'a dyn Fn(CountKind) -> Info>,
	pub unit: Option<&'a str>,
}

pub fn pair_by_kind(kinds: &[CountKind], measured: &[Measured], pair: &Pair) -> Vec<StatRecord> {
	let unit = |record: StatRecord| match pair.unit {
		Some(unit) => with_unit(record, unit),
		None => record,
	};
	kinds
		.iter()
		.filter_map(|kind| {
			let found = of_kind(measured, *kind);
			(!found.is_empty()).then(|| {
				let high = highest((pair.high)(*kind), found.clone());
				let low = pair
					.low
					.map(|low| lowest(low(*kind), found))
					.filter(|low| low.value.total_cmp(&high.value).is_ne());
				[Some(high), low].into_iter().flatten().map(unit)
			})
		})
		.flatten()
		.collect()
}

#[expect(clippy::cast_precision_loss, reason = "counts are small")]
pub fn kind_row(
	info: Info,
	unit: Option<&str>,
	values: impl IntoIterator<Item = Option<usize>>,
) -> KindRow {
	let (id, label, hint) = info;
	KindRow {
		id,
		label,
		hint,
		unit: unit.map(str::to_owned),
		values: values
			.into_iter()
			.map(|value| value.map(|value| value as f64))
			.collect(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn class_measures(values: &[(&str, usize)]) -> Vec<Measured> {
		values
			.iter()
			.map(|(name, value)| {
				measured(
					StatRef::Class {
						name: (*name).to_owned(),
					},
					*value,
				)
			})
			.collect()
	}

	#[test]
	fn keeps_every_item_that_ties_for_the_highest_value() {
		let record = highest(
			info("test", "Test", "A test record."),
			class_measures(&[("A", 2), ("B", 5), ("C", 5), ("D", 1)]),
		);
		assert!((record.value - 5.0).abs() < f64::EPSILON);
		assert_eq!(
			record.items,
			[
				StatRef::Class {
					name: "B".to_owned()
				},
				StatRef::Class {
					name: "C".to_owned()
				}
			]
		);
	}

	#[test]
	fn keeps_every_item_that_ties_for_the_lowest_value() {
		let record = lowest(
			info("test", "Test", "A test record."),
			class_measures(&[("A", 2), ("B", 1), ("C", 3), ("D", 1)]),
		);
		assert!((record.value - 1.0).abs() < f64::EPSILON);
		assert_eq!(record.items.len(), 2);
	}

	#[test]
	fn is_empty_when_nothing_is_measured() {
		let record = highest(info("test", "Test", "A test record."), Vec::new());
		assert!(record.value.abs() < f64::EPSILON);
		assert!(record.items.is_empty());
	}

	#[test]
	fn counts_ties_instead_of_listing_too_many_items() {
		let values: Vec<(String, usize)> = (0..50).map(|index| (format!("C{index}"), 0)).collect();
		let measures: Vec<(&str, usize)> = values
			.iter()
			.map(|(name, value)| (name.as_str(), *value))
			.collect();
		let record = lowest(
			info("test", "Test", "A test record."),
			class_measures(&measures),
		);
		assert!(record.items.is_empty());
		assert_eq!(
			record.ties,
			Some(Ties {
				count: 50,
				kind: "Class".to_owned()
			})
		);
	}

	#[test]
	fn leaves_out_the_opposite_record_when_it_repeats_the_first() {
		let measures = class_measures(&[("A", 3)]);
		let records = pair_by_kind(
			&[CountKind::Class],
			&measures,
			&Pair {
				high: &|_| info("high", "High", "Highest."),
				low: Some(&|_| info("low", "Low", "Lowest.")),
				unit: None,
			},
		);
		assert_eq!(records.len(), 1);
	}
}
