use std::collections::HashMap;

use indexmap::IndexMap;

use super::extremes::{highest, info, measured, with_unit, Measured};
use super::refs::{
	current_entry_ref, current_owner_ref, event_of, oldest_first, removed_entry_ref,
	removed_owner_ref,
};
use crate::gen::history::{item_key, owner_key};
use crate::js::{days_between, js_round};
use crate::model::{
	Api, ChangeStatus, ComebackBatch, RecordGroup, StatRecord, StatRef, Update, UpdateEvent,
};

struct Comeback {
	item: StatRef,
	removed: UpdateEvent,
	returned: UpdateEvent,
}

#[derive(Default)]
struct Tracker {
	removed: HashMap<String, UpdateEvent>,
	comebacks: Vec<Comeback>,
}

impl Tracker {
	fn track(
		&mut self,
		key: String,
		status: Option<ChangeStatus>,
		update: &Update,
		item: impl FnOnce() -> StatRef,
	) {
		match status {
			Some(ChangeStatus::Added) => {
				if let Some(removed) = self.removed.get(&key)
					&& removed.version != update.version
				{
					let removed = removed.clone();
					self.comebacks.push(Comeback {
						item: item(),
						removed,
						returned: event_of(update),
					});
					self.removed.remove(&key);
				}
			}
			Some(ChangeStatus::Removed) => {
				self.removed.insert(key, event_of(update));
			}
			_ => {}
		}
	}
}

fn comebacks_of(api: &Api, updates: &[Update]) -> Vec<Comeback> {
	let mut tracker = Tracker::default();
	for update in oldest_first(updates) {
		for owner in &update.owners {
			tracker.track(owner_key(owner.kind, &owner.name), owner.status, update, || {
				current_owner_ref(api, owner)
					.unwrap_or_else(|| removed_owner_ref(owner, &update.version))
			});
			for entry in &owner.entries {
				tracker.track(
					item_key(owner.kind, &owner.name, &entry.name),
					Some(entry.status),
					update,
					|| {
						current_entry_ref(api, owner, entry)
							.unwrap_or_else(|| removed_entry_ref(owner, entry, &update.version))
					},
				);
			}
		}
	}
	tracker.comebacks
}

fn gone(comeback: &Comeback) -> i64 {
	#[expect(clippy::cast_possible_truncation, reason = "day counts are small")]
	let days = js_round(days_between(&comeback.removed.published, &comeback.returned.published)) as i64;
	days
}

fn batches(comebacks: &[Comeback]) -> Vec<ComebackBatch> {
	let mut groups: IndexMap<String, ComebackBatch> = IndexMap::new();
	for comeback in comebacks {
		let key = format!("{} {}", comeback.removed.version, comeback.returned.version);
		groups
			.entry(key)
			.or_insert_with(|| ComebackBatch {
				removed: comeback.removed.clone(),
				returned: comeback.returned.clone(),
				count: 0,
				days: gone(comeback),
			})
			.count += 1;
	}
	let mut batches: Vec<ComebackBatch> = groups.into_values().collect();
	batches.sort_by(|a, b| {
		b.count
			.cmp(&a.count)
			.then_with(|| b.returned.published.cmp(&a.returned.published))
	});
	batches
}

pub struct ComebackStats {
	pub records: RecordGroup,
	pub batches: Vec<ComebackBatch>,
}

pub fn comeback_stats(api: &Api, updates: &[Update]) -> ComebackStats {
	let comebacks = comebacks_of(api, updates);
	let owners: Vec<StatRef> = comebacks
		.iter()
		.filter(|comeback| matches!(comeback.item, StatRef::Class { .. } | StatRef::Enum { .. }))
		.map(|comeback| comeback.item.clone())
		.collect();
	#[expect(clippy::cast_precision_loss, reason = "counts are small")]
	let records = vec![
		StatRecord {
			id: "items-added-back".to_owned(),
			label: "Items added back".to_owned(),
			hint: "Classes, members, enums, and enum items that an update removed and a later update added back.".to_owned(),
			value: comebacks.len() as f64,
			unit: None,
			items: Vec::new(),
		},
		StatRecord {
			id: "owners-added-back".to_owned(),
			label: "Classes and enums added back".to_owned(),
			hint: "Classes and enums that an update removed and a later update added back.".to_owned(),
			value: owners.len() as f64,
			unit: None,
			items: owners,
		},
		with_unit(
			highest(
				info(
					"longest-gone",
					"Longest gone",
					"The items that stayed removed the longest before an update added them back.",
				),
				comebacks
					.iter()
					.map(|comeback| Measured {
						item: comeback.item.clone(),
						#[expect(clippy::cast_precision_loss, reason = "day counts are small")]
						value: gone(comeback) as f64,
					})
					.collect(),
			),
			"days",
		),
	];
	let _ = measured;
	ComebackStats {
		records: RecordGroup {
			id: "comeback-records".to_owned(),
			title: "Comebacks".to_owned(),
			records,
		},
		batches: batches(&comebacks),
	}
}
