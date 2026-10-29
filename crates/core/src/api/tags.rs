use crate::js::compare_names;
use crate::model::{Member, MemberDetail, Security};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagTone {
	Neutral,
	Warning,
	Danger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayTag {
	pub label: String,
	pub tone: TagTone,
	pub note: Option<String>,
	pub hint: Option<String>,
}

impl DisplayTag {
	fn new(label: impl Into<String>, tone: TagTone) -> Self {
		Self {
			label: label.into(),
			tone,
			note: None,
			hint: None,
		}
	}
}

pub const fn security_label(security: Security) -> &'static str {
	match security {
		Security::None => "No security",
		Security::PluginSecurity => "Plugin security",
		Security::LocalUserSecurity => "Local user security",
		Security::WritePlayerSecurity => "Write player security",
		Security::RobloxScriptSecurity => "Roblox script security",
		Security::RobloxSecurity => "Roblox engine security",
		Security::NotAccessibleSecurity => "Not accessible",
	}
}

pub const fn is_internal_security(security: Security) -> bool {
	security.rank() >= Security::RobloxScriptSecurity.rank()
}

pub fn tag_label(tag: &str) -> String {
	let known = match tag {
		"Deprecated" => "Deprecated",
		"Hidden" => "Hidden",
		"NotBrowsable" => "Not browsable",
		"NotCreatable" => "Not creatable",
		"NotReplicated" => "Not replicated",
		"NotScriptable" => "Not scriptable",
		"PlayerReplicated" => "Player replicated",
		"ReadOnly" => "Read only",
		"WriteOnly" => "Write only",
		"Service" => "Service",
		"Yields" => "Yields",
		"CanYield" => "Can yield",
		"NoYield" => "No yield",
		"CustomLuaState" => "Custom Lua state",
		_ => "",
	};
	if !known.is_empty() {
		return known.to_owned();
	}
	let mut label = String::with_capacity(tag.len() + 4);
	let mut previous: Option<char> = None;
	for character in tag.chars() {
		if previous.is_some_and(|before| before.is_ascii_lowercase())
			&& character.is_ascii_uppercase()
		{
			label.push(' ');
		}
		label.push(character);
		previous = Some(character);
	}
	label
}

pub fn plain_tags(tags: &[String]) -> Vec<DisplayTag> {
	let mut ordered: Vec<&String> = tags.iter().collect();
	ordered.sort_by(|a, b| match (a.as_str(), b.as_str()) {
		("Deprecated", _) => std::cmp::Ordering::Less,
		(_, "Deprecated") => std::cmp::Ordering::Greater,
		_ => compare_names(a, b),
	});
	ordered
		.into_iter()
		.map(|tag| {
			let tone = if tag == "Deprecated" {
				TagTone::Warning
			} else {
				TagTone::Neutral
			};
			DisplayTag::new(tag_label(tag), tone)
		})
		.collect()
}

fn security_tag(prefix: &str, security: Security) -> DisplayTag {
	let tone = if is_internal_security(security) {
		TagTone::Danger
	} else {
		TagTone::Neutral
	};
	DisplayTag::new(format!("{prefix}{}", security_label(security)), tone)
}

fn security_tags(member: &Member) -> Vec<DisplayTag> {
	let write = member.write_security();
	if !matches!(member.detail, MemberDetail::Property { .. }) || member.security == write {
		return if member.security == Security::None {
			Vec::new()
		} else {
			vec![security_tag("", member.security)]
		};
	}
	let mut tags = Vec::new();
	if member.security != Security::None {
		tags.push(security_tag("Read: ", member.security));
	}
	if write != Security::None {
		tags.push(security_tag("Write: ", write));
	}
	tags
}

fn thread_safety_tag(member: &Member) -> Vec<DisplayTag> {
	let safety = member.thread_safety.as_str();
	if safety == "Unknown" {
		return Vec::new();
	}
	if matches!(member.detail, MemberDetail::Property { .. }) {
		return if safety == "Unsafe" {
			vec![DisplayTag::new("Thread unsafe", TagTone::Neutral)]
		} else {
			Vec::new()
		};
	}
	if safety == "Unsafe" {
		return Vec::new();
	}
	let label = if safety == "LocalSafe" {
		"Local thread safe"
	} else {
		"Thread safe"
	};
	vec![DisplayTag::new(label, TagTone::Neutral)]
}

fn serialization_tag(member: &Member) -> Vec<DisplayTag> {
	match member.detail {
		MemberDetail::Property {
			can_load, can_save, ..
		} if can_load != can_save => {
			let label = if can_save { "Save only" } else { "Load only" };
			vec![DisplayTag::new(label, TagTone::Neutral)]
		}
		_ => Vec::new(),
	}
}

fn capability_tags(member: &Member) -> Vec<DisplayTag> {
	let read = member.capabilities.read.join(" + ");
	let write = member.capabilities.write.join(" + ");
	if read == write {
		return if read.is_empty() {
			Vec::new()
		} else {
			vec![DisplayTag::new(
				format!("Capabilities: {read}"),
				TagTone::Neutral,
			)]
		};
	}
	let mut tags = Vec::new();
	if !read.is_empty() {
		tags.push(DisplayTag::new(
			format!("Read capabilities: {read}"),
			TagTone::Neutral,
		));
	}
	if !write.is_empty() {
		tags.push(DisplayTag::new(
			format!("Write capabilities: {write}"),
			TagTone::Neutral,
		));
	}
	tags
}

fn simulation_tag(member: &Member) -> Vec<DisplayTag> {
	if member.simulation_access() {
		vec![DisplayTag::new("Simulation access", TagTone::Neutral)]
	} else {
		Vec::new()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemberTagGroup {
	Tag,
	Security,
	ThreadSafety,
	Capability,
	Other,
}

impl MemberTagGroup {
	pub const ALL: [Self; 5] = [
		Self::Tag,
		Self::Security,
		Self::ThreadSafety,
		Self::Capability,
		Self::Other,
	];

	pub const fn title(self) -> &'static str {
		match self {
			Self::Tag => "Tags",
			Self::Security => "Security",
			Self::ThreadSafety => "Thread safety",
			Self::Capability => "Capabilities",
			Self::Other => "Other",
		}
	}
}

pub fn grouped_member_tags(member: &Member) -> [(MemberTagGroup, Vec<DisplayTag>); 5] {
	let mut other = serialization_tag(member);
	other.extend(simulation_tag(member));
	[
		(MemberTagGroup::Tag, plain_tags(&member.tags)),
		(MemberTagGroup::Security, security_tags(member)),
		(MemberTagGroup::ThreadSafety, thread_safety_tag(member)),
		(MemberTagGroup::Capability, capability_tags(member)),
		(MemberTagGroup::Other, other),
	]
}

pub fn member_tags(member: &Member) -> Vec<DisplayTag> {
	grouped_member_tags(member)
		.into_iter()
		.flat_map(|(_, tags)| tags)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn splits_unknown_tags_into_words() {
		assert_eq!(tag_label("NotScriptable"), "Not scriptable");
		assert_eq!(tag_label("SomeNewTag"), "Some New Tag");
		assert_eq!(tag_label("ABCTag"), "ABCTag");
	}

	#[test]
	fn puts_deprecated_first() {
		let tags = plain_tags(&["Yields".into(), "Deprecated".into(), "Hidden".into()]);
		let labels: Vec<_> = tags.iter().map(|tag| tag.label.as_str()).collect();
		assert_eq!(labels, ["Deprecated", "Hidden", "Yields"]);
		assert_eq!(tags[0].tone, TagTone::Warning);
	}
}
