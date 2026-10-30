use indexmap::IndexMap;

use super::extremes::{Measured, Pair, highest, info, kind_row, measured, nonzero, pair_by_kind};
use super::refs::{current_entry_ref, oldest_first, removed_entry_ref};
use crate::api::stats_text::{kind_plural, kind_singular, kind_slug};
use crate::generate::history::item_key;
use crate::model::{
	ApiType, ChangeStatus, CountKind, EntryValues, KindTable, Member, MemberDetail, RecordGroup,
	StatRef, Update,
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
						item: current_entry_ref(api, owner, entry).unwrap_or_else(|| {
							removed_entry_ref(api, owner, entry, &update.version)
						}),
						signature: 0,
						defaults: 0,
					});
				tally.signature += usize::from(changed_signature);
				tally.defaults += usize::from(changed_default);
			}
		}
	}
	let all: Vec<Tally> = tallies.into_values().collect();
	let kinds = CountKind::MEMBERS.to_vec();
	let per_kind = |value: &dyn Fn(&Tally) -> usize| {
		kinds
			.iter()
			.map(|kind| {
				Some(
					all.iter()
						.filter(|tally| tally.item.count_kind() == Some(*kind))
						.map(value)
						.sum(),
				)
			})
			.collect::<Vec<_>>()
	};
	let table = KindTable {
		rows: vec![
			kind_row(
				info(
					"signature-changes",
					"Signature changes",
					"Times an update changed the type, parameters, or return type of a member.",
				),
				None,
				per_kind(&|tally| tally.signature),
			),
			kind_row(
				info(
					"members-with-signature-changes",
					"Members with a changed signature",
					"Members whose type, parameters, or return type changed at least once.",
				),
				None,
				per_kind(&|tally| usize::from(tally.signature > 0)),
			),
			kind_row(
				info(
					"default-changes",
					"Default value changes",
					"Times an update changed the default value of a property.",
				),
				None,
				per_kind(&|tally| tally.defaults)
					.into_iter()
					.zip(&kinds)
					.map(|(value, kind)| value.filter(|_| *kind == CountKind::Property)),
			),
		],
		kinds,
	};
	let signature: Vec<Measured> = all
		.iter()
		.map(|tally| measured(tally.item.clone(), tally.signature))
		.collect();
	let mut records = pair_by_kind(
		&CountKind::MEMBERS,
		&nonzero(signature),
		&Pair {
			high: &|kind| {
				info(
					&format!("most-signature-changes-{}", kind_slug(kind)),
					&format!("Most-changed {} signature", kind_singular(kind)),
					&format!(
						"The {} whose type, parameters, or return type changed in the most updates.",
						kind_plural(kind)
					),
				)
			},
			low: None,
			unit: None,
		},
	);
	records.push(highest(
		info(
			"most-default-changes",
			"Most default value changes",
			"The properties whose default value changed in the most updates.",
		),
		all.iter()
			.map(|tally| measured(tally.item.clone(), tally.defaults))
			.collect(),
	));
	RecordGroup {
		id: "signature-records".to_owned(),
		title: "Signature changes".to_owned(),
		table: Some(table),
		records,
	}
}
