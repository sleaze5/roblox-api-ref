use super::extremes::{Measured, count, highest, info, lowest, with_unit};
use crate::js::{days_between, js_round};
use crate::model::{RecordGroup, StatRecord, StatRef, Update};

fn update_ref(update: &Update) -> StatRef {
	StatRef::Update {
		version: update.version.clone(),
		published: update.published.clone(),
	}
}

fn median(values: &[f64]) -> f64 {
	let mut sorted = values.to_vec();
	sorted.sort_by(f64::total_cmp);
	if sorted.is_empty() {
		return 0.0;
	}
	let middle = sorted.len() / 2;
	if sorted.len() % 2 == 1 {
		sorted[middle]
	} else {
		f64::midpoint(sorted[middle - 1], sorted[middle])
	}
}

fn one_decimal(value: f64) -> f64 {
	js_round(value * 10.0) / 10.0
}

fn gap_measures(gaps: &[(&Update, f64)]) -> Vec<Measured> {
	gaps.iter()
		.map(|(update, days)| Measured {
			item: update_ref(update),
			value: js_round(*days),
		})
		.collect()
}

pub fn rhythm_records(updates: &[Update]) -> RecordGroup {
	let mut by_date: Vec<&Update> = updates.iter().collect();
	by_date.sort_by(|a, b| a.published.cmp(&b.published));
	let gaps: Vec<(&Update, f64)> = by_date
		.windows(2)
		.map(|pair| {
			(
				pair[1],
				days_between(&pair[0].published, &pair[1].published),
			)
		})
		.collect();
	let quiet: Vec<StatRef> = updates
		.iter()
		.filter(|update| update.counts.total() == 0)
		.map(update_ref)
		.collect();
	let days: Vec<f64> = gaps.iter().map(|(_, days)| *days).collect();
	#[expect(clippy::cast_precision_loss, reason = "update counts are small")]
	let average = days.iter().sum::<f64>() / gaps.len().max(1) as f64;
	let quiet_count = quiet.len();
	RecordGroup {
		id: "rhythm-records".to_owned(),
		title: "Release rhythm".to_owned(),
		table: None,
		records: vec![
			StatRecord {
				id: "average-gap".to_owned(),
				label: "Average time between updates".to_owned(),
				hint: "The average time from one update's publish date to the next.".to_owned(),
				value: one_decimal(average),
				unit: Some("days".to_owned()),
				items: Vec::new(),
				ties: None,
			},
			StatRecord {
				id: "median-gap".to_owned(),
				label: "Median time between updates".to_owned(),
				hint: "Half of the updates came sooner than this after the one before, and half came later.".to_owned(),
				value: one_decimal(median(&days)),
				unit: Some("days".to_owned()),
				items: Vec::new(),
				ties: None,
			},
			with_unit(
				highest(
					info(
						"longest-gap",
						"Longest wait for an update",
						"The updates that came the longest after the one before them.",
					),
					gap_measures(&gaps),
				),
				"days",
			),
			with_unit(
				lowest(
					info(
						"shortest-gap",
						"Shortest wait for an update",
						"The updates that came the soonest after the one before them.",
					),
					gap_measures(&gaps),
				),
				"days",
			),
			StatRecord {
				items: quiet,
				..count(
					info(
						"quiet-updates",
						"Updates with no API changes",
						"Updates that added, removed, or changed nothing in the API, though they may have changed descriptions.",
					),
					quiet_count,
					None,
				)
			},
		],
	}
}
