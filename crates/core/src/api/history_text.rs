use crate::model::{Deprecation, HistoryChanges, HistoryEntry, HistoryKind, MemberCounts};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryNoun {
	Member,
	Item,
}

impl HistoryNoun {
	const fn word(self) -> &'static str {
		match self {
			Self::Member => "member",
			Self::Item => "item",
		}
	}
}

fn plural(count: u32, noun: &str) -> String {
	format!("{count} {noun}{}", if count == 1 { "" } else { "s" })
}

fn member_parts(members: MemberCounts, deprecated: u32, noun: HistoryNoun) -> Vec<String> {
	[
		("added", members.added),
		("removed", members.removed),
		("changed", members.changed),
		("deprecated", deprecated),
	]
	.into_iter()
	.filter(|(_, count)| *count > 0)
	.enumerate()
	.map(|(index, (kind, count))| {
		if index == 0 {
			format!("{} {kind}", plural(count, noun.word()))
		} else {
			format!("{count} {kind}")
		}
	})
	.collect()
}

fn change_parts(changes: &HistoryChanges) -> Vec<String> {
	let mut parts = Vec::new();
	match changes.deprecation {
		Some(Deprecation::Set) => parts.push("deprecated".to_owned()),
		Some(Deprecation::Clear) => parts.push("no longer deprecated".to_owned()),
		None => {}
	}
	parts.extend(
		changes
			.fields
			.iter()
			.map(|field| format!("{field} updated")),
	);
	if changes.tags_added > 0 {
		parts.push(format!("{} added", plural(changes.tags_added, "tag")));
	}
	if changes.tags_removed > 0 {
		parts.push(format!("{} removed", plural(changes.tags_removed, "tag")));
	}
	parts
}

pub fn entry_text(entry: &HistoryEntry, noun: HistoryNoun) -> String {
	let has = |kind: HistoryKind| entry.kinds.contains(&kind);
	let mut parts = Vec::new();
	if has(HistoryKind::Added) {
		parts.push("added".to_owned());
	}
	if has(HistoryKind::Removed) {
		parts.push("removed".to_owned());
	}
	if has(HistoryKind::Changed) && entry.changes.is_none() {
		parts.push("changed".to_owned());
	}
	if let Some(members) = entry.members {
		let deprecated = entry
			.changes
			.as_ref()
			.and_then(|changes| changes.deprecated)
			.unwrap_or(0);
		parts.extend(member_parts(members, deprecated, noun));
	}
	if let Some(changes) = &entry.changes {
		parts.extend(change_parts(changes));
	}
	if has(HistoryKind::Described) {
		parts.push("description updated".to_owned());
	}
	let text = parts.join(", ");
	let mut characters = text.chars();
	characters.next().map_or_else(String::new, |first| {
		first.to_uppercase().chain(characters).collect()
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	fn entry(kinds: Vec<HistoryKind>) -> HistoryEntry {
		HistoryEntry {
			version: "1".into(),
			published: "2026-01-01".into(),
			kinds,
			members: None,
			changes: None,
		}
	}

	#[test]
	fn counts_members_then_names_what_changed_on_the_class() {
		let mut item = entry(vec![HistoryKind::Changed]);
		item.members = Some(MemberCounts {
			added: 2,
			removed: 0,
			changed: 1,
		});
		item.changes = Some(HistoryChanges {
			fields: vec!["superclass".into()],
			tags_added: 2,
			..HistoryChanges::default()
		});
		assert_eq!(
			entry_text(&item, HistoryNoun::Member),
			"2 members added, 1 changed, superclass updated, 2 tags added"
		);
	}

	#[test]
	fn names_what_changed_on_the_members_and_counts_deprecated_members() {
		let mut item = entry(Vec::new());
		item.members = Some(MemberCounts {
			added: 2,
			removed: 0,
			changed: 3,
		});
		item.changes = Some(HistoryChanges {
			fields: vec!["security".into()],
			tags_added: 2,
			deprecated: Some(1),
			..HistoryChanges::default()
		});
		assert_eq!(
			entry_text(&item, HistoryNoun::Member),
			"2 members added, 3 changed, 1 deprecated, security updated, 2 tags added"
		);
	}

	#[test]
	fn calls_the_members_of_an_enum_items() {
		let mut item = entry(Vec::new());
		item.members = Some(MemberCounts {
			added: 1,
			removed: 3,
			changed: 0,
		});
		assert_eq!(
			entry_text(&item, HistoryNoun::Item),
			"1 item added, 3 removed"
		);
	}

	#[test]
	fn puts_a_deprecation_first_and_the_description_last() {
		let mut item = entry(vec![HistoryKind::Changed, HistoryKind::Described]);
		item.changes = Some(HistoryChanges {
			fields: vec!["replacement".into(), "security".into()],
			tags_removed: 1,
			deprecation: Some(Deprecation::Set),
			..HistoryChanges::default()
		});
		assert_eq!(
			entry_text(&item, HistoryNoun::Member),
			"Deprecated, replacement updated, security updated, 1 tag removed, description updated"
		);
	}

	#[test]
	fn says_added_removed_or_changed_when_nothing_more_is_known() {
		assert_eq!(
			entry_text(&entry(vec![HistoryKind::Added]), HistoryNoun::Member),
			"Added"
		);
		assert_eq!(
			entry_text(&entry(vec![HistoryKind::Changed]), HistoryNoun::Member),
			"Changed"
		);
		assert_eq!(
			entry_text(&entry(vec![HistoryKind::Described]), HistoryNoun::Member),
			"Description updated"
		);
	}
}
