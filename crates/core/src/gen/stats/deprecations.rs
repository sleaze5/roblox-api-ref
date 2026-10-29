use std::collections::HashMap;

use indexmap::IndexMap;

use super::extremes::{count, highest, info, with_unit, Measured};
use super::refs::{oldest_first, removed_entry_ref};
use crate::api::access::access_tier;
use crate::gen::history::{entry_deprecation, item_key};
use crate::js::{days_between, js_round};
use crate::model::{
	AccessTier, Api, ChangeStatus, Deprecation, EntryValues, HistoryEvent, ItemHistory,
	RecordGroup, StatRecord, StatRef, Update,
};

const DEPRECATED: &str = "Deprecated";

struct Removal {
	item: StatRef,
	deprecated: bool,
	scriptable: bool,
	days: Option<f64>,
}

fn removals(updates: &[Update]) -> Vec<Removal> {
	let mut deprecated_at: HashMap<String, HistoryEvent> = HashMap::new();
	let mut removed: IndexMap<String, Removal> = IndexMap::new();
	for update in oldest_first(updates) {
		for owner in &update.owners {
			for entry in &owner.entries {
				let key = item_key(owner.kind, &owner.name, &entry.name);
				if entry.status == ChangeStatus::Added {
					removed.shift_remove(&key);
				}
				if entry.status == ChangeStatus::Removed {
					let since = deprecated_at.get(&key);
					let scriptable = match &entry.values {
						EntryValues::EnumItem { .. } => true,
						EntryValues::Member { before, .. } => before
							.as_ref()
							.is_some_and(|member| access_tier(member) == AccessTier::Game),
					};
					removed.insert(
						key.clone(),
						Removal {
							item: removed_entry_ref(owner, entry, &update.version),
							deprecated: entry
								.before_tags()
								.is_some_and(|tags| tags.iter().any(|tag| tag == DEPRECATED)),
							scriptable,
							days: since.map(|since| {
								js_round(days_between(&since.published, &update.published))
							}),
						},
					);
				}
				match entry_deprecation(entry) {
					Some(Deprecation::Set) => {
						deprecated_at.insert(
							key,
							HistoryEvent {
								version: update.version.clone(),
								published: update.published.clone(),
							},
						);
					}
					Some(Deprecation::Clear) => {
						deprecated_at.remove(&key);
					}
					None => {}
				}
			}
		}
	}
	removed.into_values().collect()
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
	tags.iter()
		.any(|tag| tag == DEPRECATED)
		.then_some(Deprecated { item, history })
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

fn median(values: &[f64]) -> f64 {
	let mut sorted = values.to_vec();
	sorted.sort_by(f64::total_cmp);
	if sorted.is_empty() {
		return 0.0;
	}
	sorted[(sorted.len() - 1) / 2]
}

pub fn deprecation_records(api: &Api, updates: &[Update]) -> RecordGroup {
	let all = removals(updates);
	let deprecated: Vec<&Removal> = all.iter().filter(|removal| removal.deprecated).collect();
	let timed: Vec<Measured> = deprecated
		.iter()
		.filter_map(|removal| {
			removal.days.map(|value| Measured {
				item: removal.item.clone(),
				value,
			})
		})
		.collect();
	let current = current_items(api);
	let dated: Vec<Measured> = current
		.iter()
		.filter_map(|item| {
			let deprecated = item.history?.deprecated.as_ref()?;
			Some(Measured {
				item: item.item.clone(),
				value: js_round(days_between(&deprecated.published, &api.published)),
			})
		})
		.collect();
	let timed_values: Vec<f64> = timed.iter().map(|entry| entry.value).collect();
	let undeprecated_scriptable = all
		.iter()
		.filter(|removal| !removal.deprecated && removal.scriptable)
		.count();
	let dated_count = dated.len();
	RecordGroup {
		id: "deprecation-records".to_owned(),
		title: "Deprecations".to_owned(),
		records: vec![
			count(
				info(
					"removed-deprecated",
					"Removed after deprecation",
					"Members and enum items that were deprecated when an update removed them.",
				),
				deprecated.len(),
				None,
			),
			count(
				info(
					"removed-undeprecated",
					"Removed without deprecation",
					"Members and enum items that an update removed without deprecating them first.",
				),
				all.len() - deprecated.len(),
				None,
			),
			count(
				info(
					"removed-undeprecated-scriptable",
					"Removed without deprecation, usable by scripts",
					"Of the items removed without deprecation, the enum items and the members that game scripts could use.",
				),
				undeprecated_scriptable,
				None,
			),
			StatRecord {
				id: "median-deprecated-time".to_owned(),
				label: "Median time deprecated before removal".to_owned(),
				hint: "For removed members and enum items whose deprecation is recorded, the middle time between deprecation and removal.".to_owned(),
				value: median(&timed_values),
				unit: Some("days".to_owned()),
				items: Vec::new(),
			},
			with_unit(
				highest(
					info(
						"longest-deprecated-removed",
						"Longest deprecated before removal",
						"The removed members and enum items that stayed deprecated the longest.",
					),
					timed,
				),
				"days",
			),
			with_unit(
				highest(
					info(
						"longest-deprecated-current",
						"Longest deprecated, still present",
						"The deprecated items that still exist and were deprecated the longest ago, counting only deprecations the history records.",
					),
					dated,
				),
				"days",
			),
			count(
				info(
					"deprecated-untracked",
					"Deprecated before tracking, still present",
					"Deprecated classes, members, enums, and enum items that were already deprecated in the oldest tracked version.",
				),
				current.len() - dated_count,
				None,
			),
		],
	}
}
