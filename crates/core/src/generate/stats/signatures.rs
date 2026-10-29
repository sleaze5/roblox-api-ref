use indexmap::IndexMap;

use super::extremes::{count, highest, info, measured};
use super::refs::{current_entry_ref, oldest_first, removed_entry_ref};
use crate::generate::history::item_key;
use crate::model::{
	ApiType, ChangeStatus, EntryValues, Member, MemberDetail, RecordGroup, StatRef, Update,
};

#[derive(PartialEq)]
enum Signature<'a> {
	Property(&'a ApiType),
	Parameters(
		Vec<(&'a str, &'a ApiType, Option<&'a str>)>,
		Option<&'a ApiType>,
	),
}

fn signature(member: &Member) -> Signature<'_> {
	match &member.detail {
		MemberDetail::Property { ty, .. } => Signature::Property(ty),
		_ => Signature::Parameters(
			member
				.parameters()
				.iter()
				.map(|parameter| {
					(
						parameter.name.as_str(),
						&parameter.ty,
						parameter.default.as_deref(),
					)
				})
				.collect(),
			member.returns(),
		),
	}
}

struct Tally {
	item: StatRef,
	signature: usize,
	defaults: usize,
}

pub fn signature_records(api: &crate::model::Api, updates: &[Update]) -> RecordGroup {
	let mut tallies: IndexMap<String, Tally> = IndexMap::new();
	let mut total = 0;
	for update in oldest_first(updates) {
		for owner in &update.owners {
			for entry in &owner.entries {
				let EntryValues::Member {
					before: Some(before),
					after: Some(after),
				} = &entry.values
				else {
					continue;
				};
				if entry.status != ChangeStatus::Changed {
					continue;
				}
				let changed_signature = signature(before) != signature(after);
				let changed_default = before.property_default() != after.property_default();
				if !changed_signature && !changed_default {
					continue;
				}
				let tally = tallies
					.entry(item_key(owner.kind, &owner.name, &entry.name))
					.or_insert_with(|| Tally {
						item: current_entry_ref(api, owner, entry)
							.unwrap_or_else(|| removed_entry_ref(owner, entry, &update.version)),
						signature: 0,
						defaults: 0,
					});
				tally.signature += usize::from(changed_signature);
				tally.defaults += usize::from(changed_default);
				total += usize::from(changed_signature);
			}
		}
	}
	let all: Vec<Tally> = tallies.into_values().collect();
	RecordGroup {
		id: "signature-records".to_owned(),
		title: "Signature changes".to_owned(),
		records: vec![
			count(
				info(
					"signature-changes",
					"Signature changes",
					"Times an update changed the type, parameters, or return type of a member.",
				),
				total,
				None,
			),
			count(
				info(
					"members-with-signature-changes",
					"Members with a changed signature",
					"Members whose type, parameters, or return type changed at least once.",
				),
				all.iter().filter(|tally| tally.signature > 0).count(),
				None,
			),
			highest(
				info(
					"most-signature-changes",
					"Most signature changes",
					"The members whose type, parameters, or return type changed in the most updates.",
				),
				all.iter()
					.map(|tally| measured(tally.item.clone(), tally.signature))
					.collect(),
			),
			highest(
				info(
					"most-default-changes",
					"Most default value changes",
					"The properties whose default value changed in the most updates.",
				),
				all.iter()
					.map(|tally| measured(tally.item.clone(), tally.defaults))
					.collect(),
			),
		],
	}
}
