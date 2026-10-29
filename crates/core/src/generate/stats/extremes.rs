use crate::model::{StatRecord, StatRef};

pub struct Measured {
	pub item: StatRef,
	pub value: f64,
}

pub fn info(id: &str, label: &str, hint: &str) -> (String, String, String) {
	(id.to_owned(), label.to_owned(), hint.to_owned())
}

fn extreme(
	(id, label, hint): (String, String, String),
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
	StatRecord {
		id,
		label,
		hint,
		value: value.unwrap_or(0.0),
		unit: None,
		items,
	}
}

pub fn highest(info: (String, String, String), measured: Vec<Measured>) -> StatRecord {
	extreme(info, measured, |value, best| value > best)
}

pub fn lowest(info: (String, String, String), measured: Vec<Measured>) -> StatRecord {
	extreme(info, measured, |value, best| value < best)
}

pub fn count(info: (String, String, String), value: usize, unit: Option<&str>) -> StatRecord {
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
}
