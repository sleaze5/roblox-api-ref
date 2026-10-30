use std::collections::HashMap;

use indexmap::IndexMap;

use super::extremes::{Measured, Pair, info, kind_row, pair_by_kind};
use super::refs::{
	entry_kind, oldest_first, owner_count_kind, removed_entry_ref, removed_owner_ref,
};
use crate::api::access::access_tier;
use crate::api::stats_text::{kind_plural, kind_singular, kind_slug};
use crate::generate::history::{entry_deprecation, item_key, owner_deprecation, owner_key};
use crate::js::{days_between, js_round};
use crate::model::{
	AccessTier, Api, ChangeStatus, CountKind, Deprecation, EntryValues, HistoryEvent, ItemHistory,
	KindRow, KindTable, RecordGroup, StatRef, Update,
};

const DEPRECATED: &str = "Deprecated";

struct Removal {
	item: StatRef,
	kind: CountKind,
	deprecated: bool,
	scriptable: Option<bool>,
	days: Option<f64>,
}

#[derive(Default)]
struct Tracker {
	deprecated_at: HashMap<String, HistoryEvent>,
	removed: IndexMap<String, Removal>,
}

impl Tracker {
	fn removal(
		&mut self,
		key: &str,
		update: &Update,
		removal: impl FnOnce(Option<f64>) -> Removal,
	) {
		let days = self
			.deprecated_at
			.get(key)
			.map(|since| js_round(days_between(&since.published, &update.published)));
		self.removed.insert(key.to_owned(), removal(days));
	}

	fn deprecation(&mut self, key: String, change: Option<Deprecation>, update: &Update) {
		match change {
			Some(Deprecation::Set) => {
				self.deprecated_at.insert(
					key,
					HistoryEvent {
						version: update.version.clone(),
						published: update.published.clone(),
					},
				);
			}
			Some(Deprecation::Clear) => {
				self.deprecated_at.remove(&key);
			}
			None => {}
		}
	}
}

fn has_deprecated(tags: &[String]) -> bool {
	tags.iter().any(|tag| tag == DEPRECATED)
}

fn removals(api: &Api, updates: &[Update]) -> Vec<Removal> {
	let mut tracker = Tracker::default();
	for update in oldest_first(updates) {
		for owner in &update.owners {
			let key = owner_key(owner.kind, &owner.name);
			match owner.status {
				Some(ChangeStatus::Added) => {
					tracker.removed.shift_remove(&key);
				}
				Some(ChangeStatus::Removed) => {
					tracker.removal(&key, update, |days| Removal {
						item: removed_owner_ref(api, owner, &update.version),
						kind: owner_count_kind(owner.kind),
						deprecated: has_deprecated(&owner.tags),
						scriptable: None,
						days,
					});
				}
				_ => {}
			}
			tracker.deprecation(key, owner_deprecation(owner), update);
			for entry in &owner.entries {
				let key = item_key(owner.kind, &owner.name, &entry.name);
				if entry.status == ChangeStatus::Added {
					tracker.removed.shift_remove(&key);
				}
				if entry.status == ChangeStatus::Removed {
					tracker.removal(&key, update, |days| Removal {
						item: removed_entry_ref(api, owner, entry, &update.version),
						kind: entry_kind(entry),
						deprecated: entry.before_tags().is_some_and(has_deprecated),
						scriptable: Some(match &entry.values {
							EntryValues::EnumItem { .. } => true,
							EntryValues::Member { before, .. } => before
								.as_ref()
								.is_some_and(|member| access_tier(member) == AccessTier::Game),
						}),
						days,
					});
				}
				tracker.deprecation(key, entry_deprecation(entry), update);
			}
		}
	}
	tracker.removed.into_values().collect()
}

struct Deprecated<'a> {
	item: StatRef,
	history: Option<&'a ItemHistory>,
}

fn deprecated_item<'a>(
	item: StatRef,
	tags: &[String],
	history: Option<&'a ItemHistory>,
) -> Option<Deprecated<'a>> {
	has_deprecated(tags).then_some(Deprecated { item, history })
}

fn current_items(api: &Api) -> Vec<Deprecated<'_>> {
	let mut items = Vec::new();
	for item in api.classes.values() {
		items.extend(deprecated_item(
			StatRef::Class {
				name: item.name.clone(),
			},
			&item.tags,
			item.history.as_ref(),
		));
		for member in &item.members {
			items.extend(deprecated_item(
				StatRef::Member {
					kind: member.kind(),
					owner: item.name.clone(),
					name: member.name.clone(),
				},
				&member.tags,
				member.history.as_ref(),
			));
		}
	}
	for item in api.enums.values() {
		items.extend(deprecated_item(
			StatRef::Enum {
				name: item.name.clone(),
			},
			&item.tags,
			item.history.as_ref(),
		));
		for entry in &item.items {
			items.extend(deprecated_item(
				StatRef::EnumItem {
					owner: item.name.clone(),
					name: entry.name.clone(),
				},
				&entry.tags,
				entry.history.as_ref(),
			));
		}
	}
	items
}

fn median(values: &[f64]) -> Option<f64> {
	let mut sorted = values.to_vec();
	sorted.sort_by(f64::total_cmp);
	sorted.get(sorted.len().checked_sub(1)? / 2).copied()
}

fn of(item: &Deprecated, kind: CountKind) -> bool {
	item.item.count_kind() == Some(kind)
}

fn dated(api: &Api, item: &Deprecated) -> Option<Measured> {
	let deprecated = item.history?.deprecated.as_ref()?;
	Some(Measured {
		item: item.item.clone(),
		value: js_round(days_between(&deprecated.published, &api.published)),
	})
}

fn table(api: &Api, all: &[Removal], current: &[Deprecated]) -> KindTable {
	let kinds = CountKind::ITEMS.to_vec();
	let removed = |keep: &dyn Fn(&Removal) -> Option<bool>| {
		kinds
			.iter()
			.map(|kind| {
				let of_kind: Vec<&Removal> =
					all.iter().filter(|removal| removal.kind == *kind).collect();
				let kept: Vec<Option<bool>> = of_kind.iter().map(|removal| keep(removal)).collect();
				(!kept.contains(&None))
					.then(|| kept.iter().filter(|kept| **kept == Some(true)).count())
			})
			.collect::<Vec<_>>()
	};
	let current_count = |keep: &dyn Fn(&Deprecated) -> bool| {
		kinds
			.iter()
			.map(|kind| {
				Some(
					current
						.iter()
						.filter(|item| of(item, *kind) && keep(item))
						.count(),
				)
			})
			.collect::<Vec<_>>()
	};
	let medians = KindRow {
		values: kinds
			.iter()
			.map(|kind| {
				median(
					&all.iter()
						.filter(|removal| removal.kind == *kind && removal.deprecated)
						.filter_map(|removal| removal.days)
						.collect::<Vec<_>>(),
				)
			})
			.collect(),
		..kind_row(
			info(
				"median-deprecated-time",
				"Median time deprecated before removal",
				"For removed items whose deprecation is recorded, the middle time between deprecation and removal.",
			),
			Some("days"),
			[],
		)
	};
	KindTable {
		rows: vec![
			kind_row(
				info(
					"deprecated-now",
					"Deprecated now",
					"Items in the current version that are deprecated.",
				),
				None,
				current_count(&|_| true),
			),
			kind_row(
				info(
					"deprecated-untracked",
					"Deprecated before tracking, still present",
					"Deprecated items that were already deprecated in the oldest tracked version.",
				),
				None,
				current_count(&|item| dated(api, item).is_none()),
			),
			kind_row(
				info(
					"removed-deprecated",
					"Removed after deprecation",
					"Items that were deprecated when an update removed them.",
				),
				None,
				removed(&|removal| Some(removal.deprecated)),
			),
			kind_row(
				info(
					"removed-undeprecated",
					"Removed without deprecation",
					"Items that an update removed without deprecating them first.",
				),
				None,
				removed(&|removal| Some(!removal.deprecated)),
			),
			kind_row(
				info(
					"removed-undeprecated-scriptable",
					"Removed without deprecation, usable by scripts",
					"Of the items removed without deprecation, the enum items and the members that game scripts could use.",
				),
				None,
				removed(&|removal| {
					removal
						.scriptable
						.map(|scriptable| scriptable && !removal.deprecated)
				}),
			),
			medians,
		],
		kinds,
	}
}

pub fn deprecation_records(api: &Api, updates: &[Update]) -> RecordGroup {
	let all = removals(api, updates);
	let current = current_items(api);
	let timed: Vec<Measured> = all
		.iter()
		.filter(|removal| removal.deprecated)
		.filter_map(|removal| {
			removal.days.map(|value| Measured {
				item: removal.item.clone(),
				value,
			})
		})
		.collect();
	let dated: Vec<Measured> = current.iter().filter_map(|item| dated(api, item)).collect();
	let mut records = pair_by_kind(
		&CountKind::ITEMS,
		&timed,
		&Pair {
			high: &|kind| {
				info(
					&format!("longest-deprecated-removed-{}", kind_slug(kind)),
					&format!("Longest-deprecated removed {}", kind_singular(kind)),
					&format!(
						"The removed {} that stayed deprecated the longest.",
						kind_plural(kind)
					),
				)
			},
			low: Some(&|kind| {
				info(
					&format!("shortest-deprecated-removed-{}", kind_slug(kind)),
					&format!("Shortest-deprecated removed {}", kind_singular(kind)),
					&format!(
						"The removed {} that stayed deprecated the shortest time.",
						kind_plural(kind)
					),
				)
			}),
			unit: Some("days"),
		},
	);
	records.extend(pair_by_kind(
		&CountKind::ITEMS,
		&dated,
		&Pair {
			high: &|kind| {
				info(
					&format!("longest-deprecated-current-{}", kind_slug(kind)),
					&format!("Longest-deprecated {}, still present", kind_singular(kind)),
					&format!(
						"The deprecated {} that still exist and were deprecated the longest ago, counting only deprecations the history records.",
						kind_plural(kind)
					),
				)
			},
			low: Some(&|kind| {
				info(
					&format!("newest-deprecated-{}", kind_slug(kind)),
					&format!("Most recently deprecated {}", kind_singular(kind)),
					&format!(
						"The deprecated {} that still exist and were deprecated most recently.",
						kind_plural(kind)
					),
				)
			}),
			unit: Some("days"),
		},
	));
	RecordGroup {
		id: "deprecation-records".to_owned(),
		title: "Deprecations".to_owned(),
		table: Some(table(api, &all, &current)),
		records,
	}
}
