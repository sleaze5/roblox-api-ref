use std::cmp::Ordering;
use std::collections::HashMap;

use crate::format::RemovedData;
use crate::format::{EntryKind, Index, IndexError, Role, flags};
use crate::luau_name;
use crate::query::ParsedQuery;
use crate::score::{character_mask, score_name};
use crate::top::TopResults;

const SCOPE_THRESHOLD: i64 = 800;
const MAX_SCOPES: usize = 3;
const RELATED_KINDS: [EntryKind; 3] = [EntryKind::Enum, EntryKind::DataType, EntryKind::Class];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchResult {
	pub entry: u32,
	pub score: i64,
	pub scope: Option<u32>,
	pub role: Option<Role>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeUsers {
	pub entry: u32,
	pub type_name: String,
	pub results: Vec<SearchResult>,
}

struct Prepared {
	words: Vec<String>,
	mask: u64,
	value_types: Option<Vec<u32>>,
	used_types: Option<Vec<u32>>,
	max_access: u8,
}

pub struct Engine {
	index: Index,
	lower_names: String,
	name_offsets: Vec<usize>,
	masks: Vec<u64>,
	removed: Vec<(RemovedData, String, String)>,
	lower_type_names: Vec<String>,
	lower_luau_names: Vec<String>,
	type_ids_by_entry: HashMap<u32, u32>,
	children: HashMap<u32, Vec<u32>>,
	primaries: Vec<u32>,
}

fn has_any(list: impl Iterator<Item = u32>, ids: &[u32]) -> bool {
	list.into_iter().any(|id| ids.contains(&id))
}

fn entry_id(entry: usize) -> u32 {
	u32::try_from(entry).unwrap_or(u32::MAX)
}

impl Engine {
	pub fn new(bytes: &[u8]) -> Result<Self, IndexError> {
		let index = Index::new(bytes)?;
		let (names, offsets) = index.names_text();
		let lower_names = names.to_ascii_lowercase();
		let name_offsets: Vec<usize> = offsets.iter().map(|offset| *offset as usize).collect();
		let masks = name_offsets
			.windows(2)
			.map(|range| character_mask(&lower_names.as_bytes()[range[0]..range[1] - 1]))
			.collect();
		let removed = (0..index.removed_count())
			.map(|position| {
				let entry = index.removed(position);
				let name = entry.name.to_lowercase();
				let parent = entry.parent.as_deref().unwrap_or_default().to_lowercase();
				(entry, name, parent)
			})
			.collect();
		let lower_type_names = (0..index.type_count())
			.map(|id| index.type_name(id).to_lowercase())
			.collect();
		let lower_luau_names = (0..index.type_count())
			.map(|id| luau_name(index.type_name(id)).to_lowercase())
			.collect();
		let type_ids_by_entry = (0..index.type_count())
			.filter_map(|id| index.type_entry(id).map(|entry| (entry, entry_id(id))))
			.collect();
		let mut children: HashMap<u32, Vec<u32>> = HashMap::new();
		let mut primaries = Vec::new();
		for entry in 0..index.len() {
			if index.kind(entry).is_primary() {
				primaries.push(entry_id(entry));
			} else {
				children
					.entry(index.parent(entry).unwrap_or(u32::MAX))
					.or_default()
					.push(entry_id(entry));
			}
		}
		Ok(Self {
			index,
			lower_names,
			name_offsets,
			masks,
			removed,
			lower_type_names,
			lower_luau_names,
			type_ids_by_entry,
			children,
			primaries,
		})
	}

	pub const fn index(&self) -> &Index {
		&self.index
	}

	pub const fn len(&self) -> usize {
		self.index.len()
	}

	pub const fn is_empty(&self) -> bool {
		self.index.is_empty()
	}

	pub fn kind(&self, entry: u32) -> EntryKind {
		self.index.kind(entry as usize)
	}

	pub fn name(&self, entry: u32) -> &str {
		if entry as usize >= self.len() {
			return "";
		}
		self.index.name(entry as usize)
	}

	fn lower(&self, entry: u32) -> &str {
		let entry = entry as usize;
		&self.lower_names[self.name_offsets[entry]..self.name_offsets[entry + 1] - 1]
	}

	pub(crate) fn removed(&self) -> &[(RemovedData, String, String)] {
		&self.removed
	}

	pub fn parent(&self, entry: u32) -> Option<u32> {
		self.index.parent(entry as usize)
	}

	pub fn flags(&self, entry: u32) -> u16 {
		self.index.flags(entry as usize)
	}

	pub fn class_icon(&self, entry: u32) -> Option<u32> {
		self.index.class_icon(entry as usize)
	}

	pub fn is_primary(&self, entry: u32) -> bool {
		self.kind(entry).is_primary()
	}

	pub fn type_names(&self) -> impl Iterator<Item = &str> {
		(0..self.index.type_count()).map(|id| self.index.type_name(id))
	}

	fn matching_types(&self, value: &str) -> Vec<u32> {
		let lower = value.to_lowercase();
		let prefixed = format!("enum.{lower}");
		(0..self.index.type_count())
			.filter(|&id| {
				self.lower_type_names[id] == lower
					|| self.lower_type_names[id] == prefixed
					|| self.lower_luau_names[id] == lower
			})
			.map(entry_id)
			.collect()
	}

	fn role_of(&self, entry: u32, ids: &[u32]) -> Role {
		if !has_any(self.index.value_types(entry as usize), ids) {
			return Role::Parameter;
		}
		if self.kind(entry) == EntryKind::Property {
			Role::Value
		} else {
			Role::Return
		}
	}

	fn ancestors(&self, entry: u32) -> Vec<u32> {
		let mut chain = Vec::new();
		let mut current = Some(entry);
		while let Some(item) = current {
			if chain.len() >= 64 {
				break;
			}
			chain.push(item);
			current = if self.kind(item) == EntryKind::Class {
				self.parent(item)
			} else {
				None
			};
		}
		chain
	}

	fn members_of(&self, primary: u32) -> Vec<u32> {
		self.ancestors(primary)
			.into_iter()
			.flat_map(|owner| self.children.get(&owner).cloned().unwrap_or_default())
			.collect()
	}

	fn prepare(&self, query: &ParsedQuery) -> Prepared {
		Prepared {
			words: query.words.iter().map(|word| word.to_lowercase()).collect(),
			mask: query.words.iter().fold(0, |mask, word| {
				mask | character_mask(word.to_lowercase().as_bytes())
			}),
			value_types: query
				.value_type
				.as_deref()
				.map(|value| self.matching_types(value)),
			used_types: query
				.uses
				.as_deref()
				.map(|value| self.matching_types(value)),
			max_access: query.access.unwrap_or(u8::MAX),
		}
	}

	fn matches_filters(&self, entry: u32, query: &ParsedQuery, prepared: &Prepared) -> bool {
		if !query.has_kind(self.kind(entry)) {
			return false;
		}
		let entry_index = entry as usize;
		if let Some(ids) = &prepared.value_types
			&& !has_any(self.index.value_types(entry_index), ids)
		{
			return false;
		}
		if let Some(ids) = &prepared.used_types
			&& !has_any(self.index.value_types(entry_index), ids)
			&& !has_any(self.index.parameter_types(entry_index), ids)
		{
			return false;
		}
		if self.index.security(entry_index) > prepared.max_access {
			return false;
		}
		let entry_flags = self.flags(entry);
		entry_flags & query.include == query.include && entry_flags & query.exclude == 0
	}

	fn score_entry(&self, entry: u32, query: &ParsedQuery, prepared: &Prepared) -> i64 {
		if prepared.mask & !self.masks[entry as usize] != 0 {
			return 0;
		}
		let name = self.name(entry);
		let lower = self.lower(entry);
		let mut total = 0;
		for word in &prepared.words {
			let score = if query.exact {
				if lower == word { 1000 } else { 0 }
			} else {
				score_name(word, name, lower)
			};
			if score == 0 {
				return 0;
			}
			total += score;
		}
		let entry_flags = self.flags(entry);
		total += self.kind(entry).boost();
		if entry_flags & flags::DEPRECATED != 0 {
			total -= 60;
		}
		if entry_flags & flags::HIDDEN != 0 {
			total -= 40;
		}
		if entry_flags & flags::INTERNAL != 0 {
			total -= 40;
		}
		total.max(1)
	}

	pub fn resolve_scopes(&self, scope: &str, kind: Option<EntryKind>) -> Vec<u32> {
		let lower_scope = scope.to_lowercase();
		let primaries: Vec<u32> = self
			.primaries
			.iter()
			.copied()
			.filter(|entry| kind.is_none_or(|kind| self.kind(*entry) == kind))
			.collect();
		let exact: Vec<u32> = primaries
			.iter()
			.copied()
			.filter(|entry| self.lower(*entry) == lower_scope)
			.collect();
		if !exact.is_empty() {
			return exact;
		}
		let mut candidates: Vec<(u32, i64)> = primaries
			.into_iter()
			.map(|entry| {
				(
					entry,
					score_name(&lower_scope, self.name(entry), self.lower(entry)),
				)
			})
			.filter(|(_, score)| *score >= SCOPE_THRESHOLD)
			.collect();
		candidates.sort_by_key(|candidate| std::cmp::Reverse(candidate.1));
		candidates
			.into_iter()
			.take(MAX_SCOPES)
			.map(|(entry, _)| entry)
			.collect()
	}

	fn compare(&self, a: &SearchResult, b: &SearchResult) -> Ordering {
		b.score
			.cmp(&a.score)
			.then_with(|| self.kind(a.entry).order().cmp(&self.kind(b.entry).order()))
			.then_with(|| self.lower(a.entry).cmp(self.lower(b.entry)))
	}

	fn for_each_candidate(&self, query: &ParsedQuery, mut visit: impl FnMut(u32, Option<u32>)) {
		let Some(scope) = &query.scope else {
			if !query.words.is_empty() || query.has_filters() {
				for entry in 0..self.len() {
					visit(entry_id(entry), None);
				}
			}
			return;
		};
		if scope.is_empty() {
			for entry in (0..self.len()).map(entry_id) {
				if !self.is_primary(entry) {
					visit(entry, None);
				}
			}
			return;
		}
		let kind = query.enum_scope.then_some(EntryKind::Enum);
		for owner in self.resolve_scopes(scope, kind) {
			for entry in self.members_of(owner) {
				visit(entry, Some(owner));
			}
		}
	}

	pub fn search(&self, query: &ParsedQuery, limit: usize) -> Vec<SearchResult> {
		let prepared = self.prepare(query);
		let mut top = TopResults::new(limit, |a: &SearchResult, b: &SearchResult| {
			self.compare(a, b)
		});
		self.for_each_candidate(query, |entry, scope| {
			if !self.matches_filters(entry, query, &prepared) {
				return;
			}
			let score = self.score_entry(entry, query, &prepared);
			if score == 0 {
				return;
			}
			top.add(SearchResult {
				entry,
				score,
				scope,
				role: prepared
					.used_types
					.as_deref()
					.map(|ids| self.role_of(entry, ids)),
			});
		});
		top.into_vec()
	}

	fn named_types(&self, query: &ParsedQuery) -> Vec<u32> {
		let [word] = query.words.as_slice() else {
			return Vec::new();
		};
		if query.scope.is_some() {
			return Vec::new();
		}
		let lower = word.to_lowercase();
		RELATED_KINDS
			.into_iter()
			.filter(|kind| query.kinds.is_empty() || query.kinds.contains(kind))
			.flat_map(|kind| {
				self.primaries
					.iter()
					.copied()
					.filter(move |entry| self.kind(*entry) == kind)
					.filter(|entry| self.lower(*entry) == lower)
					.collect::<Vec<_>>()
			})
			.collect()
	}

	pub fn users_of_entry(&self, entry: u32, limit: usize) -> Vec<SearchResult> {
		let Some(id) = self.type_ids_by_entry.get(&entry) else {
			return Vec::new();
		};
		self.index
			.users(*id as usize)
			.take(limit)
			.map(|(entry, role)| SearchResult {
				entry,
				score: 0,
				scope: None,
				role: Some(role),
			})
			.collect()
	}

	pub fn type_users(&self, query: &ParsedQuery, limit: usize) -> Vec<TypeUsers> {
		self.named_types(query)
			.into_iter()
			.filter_map(|entry| {
				let results = self.users_of_entry(entry, limit);
				let id = self.type_ids_by_entry.get(&entry)?;
				(!results.is_empty()).then(|| TypeUsers {
					entry,
					type_name: self.index.type_name(*id as usize).to_owned(),
					results,
				})
			})
			.collect()
	}

	pub fn latest_upload(&self) -> Option<String> {
		(self.index.update_count() > 0).then(|| rapi_routes::upload_name(self.index.update_hash(0)))
	}
}
