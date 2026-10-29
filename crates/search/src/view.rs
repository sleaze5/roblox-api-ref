use rapi_routes::{
	OwnerKind, change_anchor, class_href, enum_href, enum_item_href, member_href, owner_anchor,
	type_href, update_href, upload_name,
};

use crate::complete::complete;
use crate::engine::{Engine, SearchResult, TypeUsers};
use crate::format::{EntryKind, RemovedData, USAGE_LIMIT, flags};
use crate::json::Json;
use crate::lookup::{match_removed, match_updates};
use crate::query::{ParsedQuery, parse_query};
use crate::score::match_positions;

const GLOBAL_LIMIT: usize = 80;
const SCOPED_LIMIT: usize = 300;
const PER_GROUP_LIMIT: usize = 8;
const UPDATE_LIMIT: usize = 8;
const REMOVED_LIMIT: usize = 8;

pub const fn kind_title(kind: EntryKind) -> &'static str {
	match kind {
		EntryKind::Class => "Classes",
		EntryKind::DataType => "Data types",
		EntryKind::Enum => "Enums",
		EntryKind::Property => "Properties",
		EntryKind::Method => "Methods",
		EntryKind::Event => "Events",
		EntryKind::Callback => "Callbacks",
		EntryKind::EnumItem => "Enum items",
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
	pub key: GroupKey,
	pub title: String,
	pub results: Vec<SearchResult>,
	pub type_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupKey {
	Kind(EntryKind),
	Uses(u32),
}

pub fn result_href(engine: &Engine, result: &SearchResult) -> String {
	let name = engine.name(result.entry);
	let parent = engine
		.parent(result.entry)
		.map_or("", |parent| engine.name(parent));
	match engine.kind(result.entry) {
		EntryKind::Class => class_href(name),
		EntryKind::DataType => type_href(name),
		EntryKind::Enum => enum_href(name),
		EntryKind::EnumItem => enum_item_href(parent, name),
		kind => member_href(
			parent,
			kind.member().unwrap_or(rapi_routes::MemberKind::Property),
			name,
		),
	}
}

pub fn removed_href(entry: &RemovedData) -> String {
	let anchor = match &entry.parent {
		None => owner_anchor(
			if entry.kind == EntryKind::Enum {
				OwnerKind::Enum
			} else {
				OwnerKind::Class
			},
			&entry.name,
		),
		Some(parent) => change_anchor(
			if entry.kind == EntryKind::EnumItem {
				OwnerKind::Enum
			} else {
				OwnerKind::Class
			},
			parent,
			&entry.name,
		),
	};
	format!("{}#{anchor}", update_href(&entry.version))
}

pub fn group_results(
	engine: &Engine,
	results: &[SearchResult],
	per_group: usize,
	users: Vec<TypeUsers>,
) -> Vec<Group> {
	let mut groups: Vec<Group> = Vec::new();
	for result in results {
		let kind = engine.kind(result.entry);
		let position = groups
			.iter()
			.position(|group| group.key == GroupKey::Kind(kind))
			.unwrap_or_else(|| {
				groups.push(Group {
					key: GroupKey::Kind(kind),
					title: kind_title(kind).to_owned(),
					results: Vec::new(),
					type_name: None,
				});
				groups.len() - 1
			});
		if groups[position].results.len() < per_group {
			groups[position].results.push(*result);
		}
	}
	for usage in users {
		let owner = groups
			.iter()
			.position(|group| group.key == GroupKey::Kind(engine.kind(usage.entry)));
		let after = groups
			.iter()
			.rposition(|group| matches!(group.key, GroupKey::Uses(_)));
		let at = match (owner, after) {
			(None, None) => 0,
			(owner, after) => owner.max(after).map_or(0, |index| index + 1),
		};
		groups.insert(
			at,
			Group {
				key: GroupKey::Uses(usage.entry),
				title: format!("Used by {}", usage.type_name),
				results: usage.results,
				type_name: Some(usage.type_name),
			},
		);
	}
	groups
}

pub struct SearchView {
	pub query: ParsedQuery,
	pub updates: Vec<usize>,
	pub groups: Vec<Group>,
	pub removed: Vec<RemovedData>,
	pub completion: Option<String>,
}

impl SearchView {
	pub fn compute(engine: &Engine, input: &str) -> Self {
		let query = parse_query(input);
		let scoped = query.scope.is_some();
		let results = engine.search(&query, if scoped { SCOPED_LIMIT } else { GLOBAL_LIMIT });
		let groups = group_results(
			engine,
			&results,
			if scoped {
				SCOPED_LIMIT
			} else {
				PER_GROUP_LIMIT
			},
			engine.type_users(&query, USAGE_LIMIT),
		);
		let top = groups
			.iter()
			.flat_map(|group| &group.results)
			.next()
			.copied();
		Self {
			updates: match_updates(engine.index(), input, UPDATE_LIMIT),
			removed: match_removed(engine, &query, REMOVED_LIMIT),
			completion: complete(input, engine, top.as_ref()),
			groups,
			query,
		}
	}
}

fn context(engine: &Engine, result: &SearchResult) -> String {
	let parent = engine.parent(result.entry);
	let parent_name = parent.map_or("", |parent| engine.name(parent));
	if engine.is_primary(result.entry) {
		return parent_name.to_owned();
	}
	match result.scope {
		Some(scope) if Some(scope) != parent => format!("inherited from {parent_name}"),
		_ => parent_name.to_owned(),
	}
}

fn marks(json: &mut Json, positions: &[usize]) {
	json.raw("[");
	let mut first = true;
	let mut index = 0;
	while index < positions.len() {
		let start = positions[index];
		let mut end = start + 1;
		while index + 1 < positions.len() && positions[index + 1] == end {
			index += 1;
			end += 1;
		}
		json.separator(first).number(start).raw(",").number(end);
		first = false;
		index += 1;
	}
	json.raw("]");
}

fn write_result(json: &mut Json, engine: &Engine, result: &SearchResult, word: &str) {
	let name = engine.name(result.entry);
	json.raw("[")
		.number(engine.kind(result.entry).code())
		.raw(",")
		.number(engine.class_icon(result.entry).map_or(-1, i64::from))
		.raw(",")
		.string(name)
		.raw(",");
	let positions: Vec<usize> = match_positions(word, name)
		.into_iter()
		.map(|byte| {
			name.get(..byte)
				.map_or(byte, |prefix| prefix.encode_utf16().count())
		})
		.collect();
	marks(json, &positions);
	json.raw(",")
		.string(&context(engine, result))
		.raw(",")
		.string(result.role.map_or("", |role| role.label()))
		.raw(",")
		.number(u8::from(
			engine.flags(result.entry) & flags::DEPRECATED != 0,
		))
		.raw(",")
		.string(&result_href(engine, result))
		.raw("]");
}

pub fn render_view(engine: &Engine, input: &str) -> String {
	let view = SearchView::compute(engine, input);
	let word = view
		.query
		.words
		.first()
		.map(|word| word.to_lowercase())
		.unwrap_or_default();
	let index = engine.index();
	let mut json = Json::new();
	json.raw("{\"u\":[");
	for (position, update) in view.updates.iter().enumerate() {
		let version = index.update_version(*update);
		json.separator(position == 0)
			.raw("[")
			.string(version)
			.raw(",")
			.string(&upload_name(index.update_hash(*update)))
			.raw(",")
			.string(index.update_date(*update))
			.raw(",")
			.string(&update_href(version))
			.raw("]");
	}
	json.raw("],\"g\":[");
	for (position, group) in view.groups.iter().enumerate() {
		let highlight = if group.type_name.is_some() {
			""
		} else {
			word.as_str()
		};
		json.separator(position == 0)
			.raw("[")
			.string(&group.title)
			.raw(",[");
		for (offset, result) in group.results.iter().enumerate() {
			json.separator(offset == 0);
			write_result(&mut json, engine, result, highlight);
		}
		json.raw("]]");
	}
	json.raw("],\"x\":[");
	for (position, entry) in view.removed.iter().enumerate() {
		json.separator(position == 0)
			.raw("[")
			.number(entry.kind.code())
			.raw(",")
			.string(&entry.name)
			.raw(",")
			.string(entry.parent.as_deref().unwrap_or_default())
			.raw(",")
			.string(&entry.version)
			.raw(",")
			.string(&removed_href(entry))
			.raw("]");
	}
	json.raw("],\"c\":");
	match &view.completion {
		Some(completion) => json.string(completion),
		None => json.raw("null"),
	};
	json.raw("}");
	json.0
}
