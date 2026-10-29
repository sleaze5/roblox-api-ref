use super::extremes::{highest, info, measured};
use crate::model::{Api, HistoryKind, ItemHistory, RecordGroup, StatRef, Update};

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

pub fn history_records(api: &Api, updates: &[Update]) -> RecordGroup {
	let classes = api.classes.values().map(|item| {
		measured(
			StatRef::Class {
				name: item.name.clone(),
			},
			api_changes(item.history.as_ref()),
		)
	});
	let members = api.classes.values().flat_map(|item| {
		item.members.iter().map(|member| {
			measured(
				StatRef::Member {
					kind: member.kind(),
					owner: item.name.clone(),
					name: member.name.clone(),
				},
				api_changes(member.history.as_ref()),
			)
		})
	});
	let enums = api.enums.values().map(|item| {
		measured(
			StatRef::Enum {
				name: item.name.clone(),
			},
			api_changes(item.history.as_ref()),
		)
	});
	RecordGroup {
		id: "history-records".to_owned(),
		title: "History".to_owned(),
		records: vec![
			highest(
				info(
					"largest-update",
					"Largest update",
					"The updates that added, removed, or changed the most classes, members, enums, and enum items.",
				),
				updates
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
					.collect(),
			),
			highest(
				info(
					"most-changed-class",
					"Most-changed class",
					"The classes that the most updates changed, counting changes to their members.",
				),
				classes.collect(),
			),
			highest(
				info(
					"most-changed-member",
					"Most-changed member",
					"The members that the most updates changed.",
				),
				members.collect(),
			),
			highest(
				info(
					"most-changed-enum",
					"Most-changed enum",
					"The enums that the most updates changed, counting changes to their items.",
				),
				enums.collect(),
			),
		],
	}
}
