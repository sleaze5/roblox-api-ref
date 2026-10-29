use indexmap::IndexMap;

use super::tags::{MemberTagGroup, grouped_member_tags};
use crate::js::compare_names;
use crate::model::Member;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterTag {
	pub key: String,
	pub label: String,
	pub short_label: String,
	pub count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterTagGroup {
	pub group: MemberTagGroup,
	pub title: &'static str,
	pub tags: Vec<FilterTag>,
}

pub fn tag_key(label: &str) -> String {
	let mut key = String::with_capacity(label.len());
	let mut dash = false;
	for character in label.to_lowercase().chars() {
		if character.is_ascii_lowercase() || character.is_ascii_digit() {
			if dash && !key.is_empty() {
				key.push('-');
			}
			dash = false;
			key.push(character);
		} else {
			dash = true;
		}
	}
	if dash && !key.is_empty() {
		key.push('-');
	}
	key.trim_end_matches('-').to_owned()
}

fn short_label(group: MemberTagGroup, label: &str) -> String {
	if group != MemberTagGroup::Capability {
		return label.to_owned();
	}
	let label = label.strip_prefix("Capabilities: ").unwrap_or(label);
	for access in ["Read", "Write"] {
		if let Some(rest) = label.strip_prefix(&format!("{access} capabilities: ")) {
			return format!("{access}: {rest}");
		}
	}
	label.to_owned()
}

pub fn member_tag_keys(member: &Member) -> Vec<String> {
	grouped_member_tags(member)
		.into_iter()
		.flat_map(|(_, tags)| tags)
		.map(|tag| tag_key(&tag.label))
		.collect()
}

pub fn filter_tag_groups<'a>(members: impl Iterator<Item = &'a Member>) -> Vec<FilterTagGroup> {
	let mut counts: Vec<IndexMap<String, FilterTag>> = MemberTagGroup::ALL
		.iter()
		.map(|_| IndexMap::new())
		.collect();
	for member in members {
		for (index, (group, tags)) in grouped_member_tags(member).into_iter().enumerate() {
			for tag in tags {
				let key = tag_key(&tag.label);
				counts[index]
					.entry(key.clone())
					.or_insert_with(|| FilterTag {
						key,
						short_label: short_label(group, &tag.label),
						label: tag.label,
						count: 0,
					})
					.count += 1;
			}
		}
	}
	MemberTagGroup::ALL
		.into_iter()
		.zip(counts)
		.map(|(group, tags)| {
			let mut tags: Vec<FilterTag> = tags.into_values().collect();
			tags.sort_by(|a, b| compare_names(&a.short_label, &b.short_label));
			FilterTagGroup {
				group,
				title: group.title(),
				tags,
			}
		})
		.filter(|group| !group.tags.is_empty())
		.collect()
}

pub fn with_all_tags(page: &[FilterTagGroup], all: &[FilterTagGroup]) -> Vec<FilterTagGroup> {
	all.iter()
		.filter_map(|group| {
			let own = page.iter().find(|item| item.group == group.group);
			if group.group == MemberTagGroup::Capability {
				return own.cloned();
			}
			let tags = group
				.tags
				.iter()
				.map(|tag| FilterTag {
					count: own
						.and_then(|own| own.tags.iter().find(|item| item.key == tag.key))
						.map_or(0, |item| item.count),
					..tag.clone()
				})
				.collect();
			Some(FilterTagGroup {
				tags,
				..group.clone()
			})
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn turns_labels_into_url_keys() {
		assert_eq!(
			tag_key("Read: Roblox script security"),
			"read-roblox-script-security"
		);
		assert_eq!(
			tag_key("Capabilities: Basic + Network"),
			"capabilities-basic-network"
		);
		assert_eq!(tag_key("  Not browsable!"), "not-browsable");
	}

	fn group(group: MemberTagGroup, tags: &[(&str, u32)]) -> FilterTagGroup {
		FilterTagGroup {
			group,
			title: group.title(),
			tags: tags
				.iter()
				.map(|(key, count)| FilterTag {
					key: (*key).to_owned(),
					label: (*key).to_owned(),
					short_label: (*key).to_owned(),
					count: *count,
				})
				.collect(),
		}
	}

	#[test]
	fn lists_every_tag_but_keeps_page_capabilities() {
		let all = [
			group(MemberTagGroup::Tag, &[("deprecated", 50), ("hidden", 40)]),
			group(MemberTagGroup::Capability, &[("basic", 9), ("ui", 3)]),
			group(MemberTagGroup::Other, &[("load-only", 2)]),
		];
		let page = [
			group(MemberTagGroup::Tag, &[("hidden", 4)]),
			group(MemberTagGroup::Capability, &[("ui", 1)]),
		];
		assert_eq!(
			with_all_tags(&page, &all),
			[
				group(MemberTagGroup::Tag, &[("deprecated", 0), ("hidden", 4)]),
				group(MemberTagGroup::Capability, &[("ui", 1)]),
				group(MemberTagGroup::Other, &[("load-only", 0)]),
			]
		);
		assert_eq!(
			with_all_tags(&[], &all),
			[
				group(MemberTagGroup::Tag, &[("deprecated", 0), ("hidden", 0)]),
				group(MemberTagGroup::Other, &[("load-only", 0)]),
			]
		);
	}
}
