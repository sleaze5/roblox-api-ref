use indexmap::IndexMap;
use rapi_routes::MemberKind;

use super::tags::is_internal_security;
use crate::js::compare_names;
use crate::model::{Member, MemberDetail};

pub const fn member_kind_title(kind: MemberKind) -> &'static str {
	match kind {
		MemberKind::Property => "Properties",
		MemberKind::Method => "Methods",
		MemberKind::Event => "Events",
		MemberKind::Callback => "Callbacks",
	}
}

pub fn is_deprecated(tags: &[String]) -> bool {
	tags.iter().any(|tag| tag == "Deprecated")
}

pub fn is_hidden(member: &Member) -> bool {
	member
		.tags
		.iter()
		.any(|tag| tag == "Hidden" || tag == "NotBrowsable")
		|| member
			.capabilities
			.read
			.iter()
			.any(|name| name == "InternalTest")
		|| member
			.capabilities
			.write
			.iter()
			.any(|name| name == "InternalTest")
}

pub const fn access_rank(member: &Member) -> usize {
	match member.detail {
		MemberDetail::Property { write_security, .. } => {
			let read = member.security.rank();
			let write = write_security.rank();
			if read < write { read } else { write }
		}
		_ => member.security.rank(),
	}
}

pub const fn is_internal(member: &Member) -> bool {
	match member.detail {
		MemberDetail::Property { write_security, .. } => {
			is_internal_security(member.security) && is_internal_security(write_security)
		}
		_ => is_internal_security(member.security),
	}
}

fn is_camel_case(name: &str) -> bool {
	name.chars().next().is_some_and(|first| {
		let upper: String = first.to_uppercase().collect();
		upper != first.to_string()
	})
}

pub fn compare_members(a: &Member, b: &Member) -> std::cmp::Ordering {
	is_deprecated(&a.tags)
		.cmp(&is_deprecated(&b.tags))
		.then_with(|| is_camel_case(&a.name).cmp(&is_camel_case(&b.name)))
		.then_with(|| compare_names(&a.name, &b.name))
}

pub fn members_of_kind(members: &[Member], kind: MemberKind) -> Vec<&Member> {
	let mut list: Vec<&Member> = members
		.iter()
		.filter(|member| member.kind() == kind)
		.collect();
	list.sort_by(|a, b| compare_members(a, b));
	list
}

pub fn property_groups(members: &[Member]) -> Vec<(String, Vec<&Member>)> {
	let mut groups: IndexMap<String, Vec<&Member>> = IndexMap::new();
	for member in members_of_kind(members, MemberKind::Property) {
		let category = match &member.detail {
			MemberDetail::Property { category, .. } => category.clone(),
			_ => String::new(),
		};
		groups.entry(category).or_default().push(member);
	}
	let mut groups: Vec<_> = groups.into_iter().collect();
	groups.sort_by(|a, b| compare_names(&a.0, &b.0));
	groups
}

pub fn members_in_page_order(members: &[Member], kind: MemberKind) -> Vec<&Member> {
	if kind == MemberKind::Property {
		property_groups(members)
			.into_iter()
			.flat_map(|(_, list)| list)
			.collect()
	} else {
		members_of_kind(members, kind)
	}
}
