use indexmap::IndexMap;

use super::refs::oldest_first;
use crate::api::access::{access_tier, tier_rows};
use crate::model::{
	AccessNow, AccessPeriod, AccessStats, AccessTier, Api, ChangeStatus, EntryValues, Member,
	TierRow, Update,
};

fn scriptable(member: &Member) -> bool {
	access_tier(member) == AccessTier::Game
}

fn thread_safe(member: &Member) -> bool {
	member.thread_safety != "Unsafe"
}

fn empty_period(period: &str) -> AccessPeriod {
	AccessPeriod {
		period: period.to_owned(),
		added_open: 0,
		added_restricted: 0,
		opened: 0,
		restricted: 0,
		made_safe: 0,
		made_unsafe: 0,
	}
}

fn count(members: &[&Member], keep: impl Fn(&Member) -> bool) -> u32 {
	u32::try_from(members.iter().filter(|member| keep(member)).count()).unwrap_or(u32::MAX)
}

pub fn access_stats(api: &Api, updates: &[Update]) -> AccessStats {
	let mut years: IndexMap<String, AccessPeriod> = IndexMap::new();
	for update in oldest_first(updates) {
		let year = update.published[..4].to_owned();
		let period = years.entry(year.clone()).or_insert_with(|| empty_period(&year));
		for owner in &update.owners {
			for entry in &owner.entries {
				let EntryValues::Member { before, after } = &entry.values else {
					continue;
				};
				match (entry.status, before, after) {
					(ChangeStatus::Added, _, Some(after)) => {
						if scriptable(after) {
							period.added_open += 1;
						} else {
							period.added_restricted += 1;
						}
					}
					(ChangeStatus::Changed, Some(before), Some(after)) => {
						if scriptable(before) != scriptable(after) {
							if scriptable(after) {
								period.opened += 1;
							} else {
								period.restricted += 1;
							}
						}
						if thread_safe(before) != thread_safe(after) {
							if thread_safe(after) {
								period.made_safe += 1;
							} else {
								period.made_unsafe += 1;
							}
						}
					}
					_ => {}
				}
			}
		}
	}
	let members: Vec<&Member> = api.classes.values().flat_map(|item| &item.members).collect();
	AccessStats {
		now: AccessNow {
			members: u32::try_from(members.len()).unwrap_or(u32::MAX),
			scriptable: count(&members, scriptable),
			thread_safe: count(&members, thread_safe),
		},
		tiers: tier_rows(&members)
			.into_iter()
			.map(|(tier, kinds)| TierRow { tier, kinds })
			.collect(),
		years: years.into_values().collect(),
	}
}
