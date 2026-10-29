use indexmap::IndexMap;

use super::refs::{event_of, oldest_first};
use crate::js::compare_names;
use crate::model::{Api, Capabilities, ChangeDetail, EntryValues, TagTrend, Update, UpdateEvent};

#[derive(Default)]
struct TagTable {
	rows: IndexMap<String, TagTrend>,
}

impl TagTable {
	fn row(&mut self, name: &str) -> &mut TagTrend {
		self.rows
			.entry(name.to_owned())
			.or_insert_with(|| TagTrend {
				name: name.to_owned(),
				current: 0,
				gained: 0,
				lost: 0,
				first: None,
			})
	}

	fn count(&mut self, names: &[String]) {
		for name in names {
			self.row(name).current += 1;
		}
	}

	fn gain(&mut self, names: &[String], event: &UpdateEvent) {
		for name in names {
			let row = self.row(name);
			row.gained += 1;
			if row.first.is_none() {
				row.first = Some(event.clone());
			}
		}
	}

	fn lose(&mut self, names: &[String]) {
		for name in names {
			self.row(name).lost += 1;
		}
	}

	fn sorted(self) -> Vec<TagTrend> {
		let mut rows: Vec<TagTrend> = self.rows.into_values().collect();
		rows.sort_by(|a, b| {
			b.current
				.cmp(&a.current)
				.then_with(|| compare_names(&a.name, &b.name))
		});
		rows
	}
}

fn difference(a: &[String], b: &[String]) -> Vec<String> {
	a.iter().filter(|name| !b.contains(name)).cloned().collect()
}

fn detail_tags(details: &[ChangeDetail], labels: &[&str]) -> Vec<String> {
	details
		.iter()
		.flat_map(|detail| match detail {
			ChangeDetail::Tags { label, tags } if labels.contains(&label.as_str()) => tags.clone(),
			_ => Vec::new(),
		})
		.collect()
}

fn capabilities_of(capabilities: &Capabilities) -> Vec<String> {
	let mut names: Vec<String> = Vec::new();
	for name in capabilities.read.iter().chain(&capabilities.write) {
		if !names.contains(name) {
			names.push(name.clone());
		}
	}
	names
}

enum Values {
	Tags,
	Capabilities,
}

impl Values {
	fn of_entry(&self, values: &EntryValues, before: bool) -> Vec<String> {
		match (self, values) {
			(
				Self::Tags,
				EntryValues::Member {
					before: b,
					after: a,
				},
			) => (if before { b } else { a })
				.as_ref()
				.map(|member| member.tags.clone()),
			(
				Self::Tags,
				EntryValues::EnumItem {
					before: b,
					after: a,
				},
			) => (if before { b } else { a })
				.as_ref()
				.map(|item| item.tags.clone()),
			(
				Self::Capabilities,
				EntryValues::Member {
					before: b,
					after: a,
				},
			) => (if before { b } else { a })
				.as_ref()
				.map(|member| capabilities_of(&member.capabilities)),
			(
				Self::Capabilities,
				EntryValues::EnumItem {
					before: b,
					after: a,
				},
			) => (if before { b } else { a }).as_ref().map(|_| Vec::new()),
		}
		.unwrap_or_default()
	}
}

fn build(api: &Api, updates: &[Update], values: &Values) -> Vec<TagTrend> {
	let owners = matches!(values, Values::Tags);
	let mut table = TagTable::default();
	for update in oldest_first(updates) {
		let event = event_of(update);
		for owner in &update.owners {
			if owners {
				table.gain(
					&detail_tags(&owner.details, &["Tags", "Added tags"]),
					&event,
				);
				table.lose(&detail_tags(&owner.details, &["Removed tags"]));
			}
			for entry in &owner.entries {
				let before = values.of_entry(&entry.values, true);
				let after = values.of_entry(&entry.values, false);
				table.gain(&difference(&after, &before), &event);
				table.lose(&difference(&before, &after));
			}
		}
	}
	for item in api.classes.values() {
		if owners {
			table.count(&item.tags);
		}
		for member in &item.members {
			match values {
				Values::Tags => table.count(&member.tags),
				Values::Capabilities => table.count(&capabilities_of(&member.capabilities)),
			}
		}
	}
	for item in api.enums.values() {
		if owners {
			table.count(&item.tags);
		}
		for entry in &item.items {
			if owners {
				table.count(&entry.tags);
			}
		}
	}
	table.sorted()
}

pub fn tag_stats(api: &Api, updates: &[Update]) -> Vec<TagTrend> {
	build(api, updates, &Values::Tags)
}

pub fn capability_stats(api: &Api, updates: &[Update]) -> Vec<TagTrend> {
	build(api, updates, &Values::Capabilities)
}
