use indexmap::IndexMap;

use crate::generate::history::{entry_deprecation, owner_deprecation};
use crate::model::{
	Api, ChangeStatus, Deprecation, EntryValues, OwnerChanges, PeriodTrend, Tallies, TrendKind,
	Update, UpdateTrend,
};

fn bump(tally: &mut crate::model::ChangeTally, status: ChangeStatus) {
	match status {
		ChangeStatus::Added => tally.added += 1,
		ChangeStatus::Removed => tally.removed += 1,
		ChangeStatus::Changed => tally.changed += 1,
	}
}

fn tally_owner(tallies: &mut Tallies, owner: &OwnerChanges) {
	let kind = match owner.kind {
		rapi_routes::OwnerKind::Class => TrendKind::Class,
		rapi_routes::OwnerKind::Enum => TrendKind::Enum,
	};
	if let Some(status) = owner.status {
		bump(tallies.get_mut(kind), status);
	}
	if owner_deprecation(owner) == Some(Deprecation::Set) {
		tallies.get_mut(kind).deprecated += 1;
	}
	for entry in &owner.entries {
		let entry_kind = match entry.values {
			EntryValues::Member { .. } => TrendKind::Member,
			EntryValues::EnumItem { .. } => TrendKind::EnumItem,
		};
		if entry.is_api_change() {
			bump(tallies.get_mut(entry_kind), entry.status);
		}
		if entry_deprecation(entry) == Some(Deprecation::Set) {
			tallies.get_mut(entry_kind).deprecated += 1;
		}
	}
}

fn update_tallies(update: &Update) -> Tallies {
	let mut tallies = Tallies::default();
	for owner in &update.owners {
		tally_owner(&mut tallies, owner);
	}
	tallies
}

pub fn update_trends(api: &Api, updates: &[Update]) -> Vec<UpdateTrend> {
	let mut classes = i64::try_from(api.classes.len()).unwrap_or_default();
	let mut enums = i64::try_from(api.enums.len()).unwrap_or_default();
	let mut trends = Vec::with_capacity(updates.len());
	for update in updates {
		let tallies = update_tallies(update);
		trends.push(UpdateTrend {
			version: update.version.clone(),
			hash: update.hash.clone(),
			published: update.published.clone(),
			tallies,
			classes: u32::try_from(classes).unwrap_or_default(),
			enums: u32::try_from(enums).unwrap_or_default(),
		});
		classes += i64::from(tallies.class.removed) - i64::from(tallies.class.added);
		enums += i64::from(tallies.enumeration.removed) - i64::from(tallies.enumeration.added);
	}
	trends.reverse();
	trends
}

fn group_trends(trends: &[UpdateTrend], period_of: impl Fn(&UpdateTrend) -> String) -> Vec<PeriodTrend> {
	let mut periods: IndexMap<String, PeriodTrend> = IndexMap::new();
	for trend in trends {
		let period = period_of(trend);
		let total = periods.entry(period.clone()).or_insert_with(|| PeriodTrend {
			period,
			updates: 0,
			tallies: Tallies::default(),
		});
		total.updates += 1;
		total.tallies.add(&trend.tallies);
	}
	let mut periods: Vec<PeriodTrend> = periods.into_values().collect();
	periods.sort_by(|a, b| a.period.cmp(&b.period));
	periods
}

pub fn year_trends(trends: &[UpdateTrend]) -> Vec<PeriodTrend> {
	group_trends(trends, |trend| trend.published[..4].to_owned())
}

pub fn month_trends(trends: &[UpdateTrend]) -> Vec<PeriodTrend> {
	group_trends(trends, |trend| trend.published[5..7].to_owned())
}
