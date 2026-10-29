use super::extremes::{Measured, Pair, highest, info, lowest, measured, nonzero, pair_by_kind};
use crate::api::stats_text::{kind_plural, kind_singular, kind_slug};
use crate::model::{Api, CountKind, HistoryKind, ItemHistory, RecordGroup, StatRef, Update};

fn api_changes(history: Option<&ItemHistory>) -> usize {
	history.map_or(0, |history| {
		history
			.entries
			.iter()
			.filter(|entry| {
				entry.members.is_some()
					|| entry
						.kinds
						.iter()
						.any(|kind| matches!(kind, HistoryKind::Changed | HistoryKind::Removed))
			})
			.count()
	})
}

fn changes(api: &Api) -> Vec<Measured> {
	let mut all = Vec::new();
	for item in api.classes.values() {
		all.push(measured(
			StatRef::Class {
				name: item.name.clone(),
			},
			api_changes(item.history.as_ref()),
		));
		for member in &item.members {
			all.push(measured(
				StatRef::Member {
					kind: member.kind(),
					owner: item.name.clone(),
					name: member.name.clone(),
				},
				api_changes(member.history.as_ref()),
			));
		}
	}
	for item in api.enums.values() {
		all.push(measured(
			StatRef::Enum {
				name: item.name.clone(),
			},
			api_changes(item.history.as_ref()),
		));
		for entry in &item.items {
			all.push(measured(
				StatRef::EnumItem {
					owner: item.name.clone(),
					name: entry.name.clone(),
				},
				api_changes(entry.history.as_ref()),
			));
		}
	}
	all
}

const fn counts_children(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => ", counting changes to their members",
		CountKind::Enum => ", counting changes to their items",
		_ => "",
	}
}

pub fn history_records(api: &Api, updates: &[Update]) -> RecordGroup {
	let sizes: Vec<Measured> = updates
		.iter()
		.map(|update| {
			measured(
				StatRef::Update {
					version: update.version.clone(),
					published: update.published.clone(),
				},
				update.counts.total() as usize,
			)
		})
		.collect();
	let mut records = vec![
		highest(
			info(
				"largest-update",
				"Largest update",
				"The updates that added, removed, or changed the most classes, members, enums, and enum items.",
			),
			sizes.clone(),
		),
		lowest(
			info(
				"smallest-update",
				"Smallest update",
				"The updates that added, removed, or changed the fewest classes, members, enums, and enum items, among those that changed any.",
			),
			nonzero(sizes),
		),
	];
	records.extend(pair_by_kind(
		&CountKind::ITEMS,
		&changes(api),
		&Pair {
			high: &|kind| {
				info(
					&format!("most-changed-{}", kind_slug(kind)),
					&format!("Most-changed {}", kind_singular(kind)),
					&format!(
						"The {} that the most updates changed{}.",
						kind_plural(kind),
						counts_children(kind)
					),
				)
			},
			low: Some(&|kind| {
				info(
					&format!("least-changed-{}", kind_slug(kind)),
					&format!("Least-changed {}", kind_singular(kind)),
					&format!(
						"The {} that the fewest updates changed{}.",
						kind_plural(kind),
						counts_children(kind)
					),
				)
			}),
			unit: None,
		},
	));
	RecordGroup {
		id: "history-records".to_owned(),
		title: "History".to_owned(),
		table: None,
		records,
	}
}
