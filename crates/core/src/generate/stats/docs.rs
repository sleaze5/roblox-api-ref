use indexmap::IndexMap;

use crate::model::{
	Api, ChangeStatus, CoverageKind, CoverageRow, DescriptionDetail, DocPeriod, DocStats,
	MemberDetail, Update,
};

fn coverage(api: &Api) -> Vec<CoverageRow> {
	let mut rows: IndexMap<CoverageKind, CoverageRow> = IndexMap::new();
	let mut add = |kind: CoverageKind, described: bool| {
		let row = rows.entry(kind).or_insert(CoverageRow {
			kind,
			described: 0,
			total: 0,
		});
		row.total += 1;
		row.described += u32::from(described);
	};
	for item in api.classes.values() {
		add(CoverageKind::Class, item.description.is_some());
		for member in &item.members {
			add(
				CoverageKind::member(member.kind()),
				member.description.is_some(),
			);
			for parameter in member.parameters() {
				add(CoverageKind::Parameter, parameter.description.is_some());
			}
			if let MemberDetail::Method {
				returns,
				return_description,
				..
			}
			| MemberDetail::Callback {
				returns,
				return_description,
				..
			} = &member.detail
				&& returns.name != "void"
				&& returns.name != "null"
			{
				add(CoverageKind::Return, return_description.is_some());
			}
		}
	}
	for item in api.enums.values() {
		add(CoverageKind::Enum, item.description.is_some());
		for entry in &item.items {
			add(CoverageKind::EnumItem, entry.description.is_some());
		}
	}
	CoverageKind::ALL
		.into_iter()
		.filter_map(|kind| rows.get(&kind).cloned())
		.collect()
}

fn tally(period: &mut DocPeriod, details: &[DescriptionDetail]) {
	for detail in details {
		match detail.status {
			ChangeStatus::Added => period.added += 1,
			ChangeStatus::Removed => period.removed += 1,
			ChangeStatus::Changed => period.changed += 1,
		}
	}
}

fn years(updates: &[Update]) -> Vec<DocPeriod> {
	let mut periods: IndexMap<String, DocPeriod> = IndexMap::new();
	for update in updates.iter().rev() {
		let year = update.published[..4].to_owned();
		let period = periods.entry(year.clone()).or_insert_with(|| DocPeriod {
			period: year,
			added: 0,
			changed: 0,
			removed: 0,
		});
		for owner in &update.owners {
			tally(period, &owner.descriptions);
			for entry in &owner.entries {
				tally(period, &entry.descriptions);
			}
		}
	}
	periods.into_values().collect()
}

pub fn doc_stats(api: &Api, updates: &[Update]) -> DocStats {
	DocStats {
		coverage: coverage(api),
		years: years(updates),
	}
}
