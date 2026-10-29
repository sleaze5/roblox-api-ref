use indexmap::IndexMap;
use rapi_routes::OwnerKind;

use crate::js::compare_names;
use crate::model::{ChangeStatus, GrowthClass, GrowthPeriod, Update};

const SHOWN: usize = 5;

pub fn growth_stats(updates: &[Update]) -> Vec<GrowthPeriod> {
	let mut years: IndexMap<String, IndexMap<String, i64>> = IndexMap::new();
	for update in updates.iter().rev() {
		let classes = years.entry(update.published[..4].to_owned()).or_default();
		for owner in update
			.owners
			.iter()
			.filter(|owner| owner.kind == OwnerKind::Class)
		{
			let net: i64 = owner
				.entries
				.iter()
				.map(|entry| match entry.status {
					ChangeStatus::Added => 1,
					ChangeStatus::Removed => -1,
					ChangeStatus::Changed => 0,
				})
				.sum();
			*classes.entry(owner.name.clone()).or_insert(0) += net;
		}
	}
	years
		.into_iter()
		.map(|(period, classes)| {
			let mut grew: Vec<GrowthClass> = classes
				.into_iter()
				.filter(|(_, added)| *added > 0)
				.map(|(name, added)| GrowthClass { name, added })
				.collect();
			grew.sort_by(|a, b| {
				b.added
					.cmp(&a.added)
					.then_with(|| compare_names(&a.name, &b.name))
			});
			grew.truncate(SHOWN);
			GrowthPeriod {
				period,
				classes: grew,
			}
		})
		.collect()
}
