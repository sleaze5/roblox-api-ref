use std::collections::HashMap;

use rapi_search::{EntryKind, IndexData, RemovedData, UpdateData, flags};

use crate::api::members::{access_rank, is_hidden, is_internal};
use crate::js::{compare_names, format_date};
use crate::model::{
	Api, ApiType, Class, HistoryEvent, Member, MemberDetail, RemovedOwner, TypeCategory, Update,
};

fn tag_flag(tag: &str) -> u16 {
	match tag {
		"Deprecated" => flags::DEPRECATED,
		"Hidden" | "NotBrowsable" => flags::HIDDEN,
		"NotScriptable" => flags::NOT_SCRIPTABLE,
		"Service" => flags::SERVICE,
		"ReadOnly" => flags::READ_ONLY,
		"Yields" => flags::YIELDS,
		"NotCreatable" => flags::NOT_CREATABLE,
		"NotReplicated" => flags::NOT_REPLICATED,
		_ => 0,
	}
}

fn flags_of(tags: &[String]) -> u16 {
	tags.iter().fold(0, |flags, tag| flags | tag_flag(tag))
}

struct TypeKey {
	key: String,
	display: String,
}

fn key_of(ty: &ApiType) -> Option<TypeKey> {
	if ty.name == "Tuple"
		&& ty
			.members
			.as_ref()
			.is_some_and(|members| !members.is_empty())
	{
		return None;
	}
	Some(TypeKey {
		key: format!("{}:{}", ty.category.name(), ty.name),
		display: if ty.category == TypeCategory::Enum {
			format!("Enum.{}", ty.name)
		} else {
			ty.name.clone()
		},
	})
}

fn keys_of(types: &[&ApiType]) -> Vec<TypeKey> {
	let mut seen: Vec<String> = Vec::new();
	types
		.iter()
		.flat_map(|ty| ty.flatten())
		.filter_map(key_of)
		.filter(|key| {
			if seen.contains(&key.key) {
				return false;
			}
			seen.push(key.key.clone());
			true
		})
		.collect()
}

fn value_type_keys(member: &Member) -> Vec<TypeKey> {
	match &member.detail {
		MemberDetail::Property { ty, .. } => keys_of(&[ty]),
		MemberDetail::Method { returns, .. } | MemberDetail::Callback { returns, .. } => {
			keys_of(&[returns])
		}
		MemberDetail::Event { .. } => Vec::new(),
	}
}

fn parameter_type_keys(member: &Member) -> Vec<TypeKey> {
	let types: Vec<&ApiType> = member
		.parameters()
		.iter()
		.map(|parameter| &parameter.ty)
		.collect();
	keys_of(&types)
}

#[derive(Default)]
struct Builder {
	data: IndexData,
	primary_entries: HashMap<String, u32>,
	type_ids_by_key: HashMap<String, u32>,
	type_keys: Vec<String>,
}

impl Builder {
	fn add(
		&mut self,
		kind: EntryKind,
		name: &str,
		parent: Option<u32>,
		tags: &[String],
		security: u8,
	) -> u32 {
		self.data.names.push(name.to_owned());
		self.data.kinds.push(kind);
		self.data.parents.push(parent);
		self.data.flags.push(flags_of(tags));
		self.data.security.push(security);
		self.data.value_types.push(Vec::new());
		self.data.parameter_types.push(Vec::new());
		u32::try_from(self.data.names.len() - 1).expect("entries fit in 32 bits")
	}

	fn type_ids(&mut self, keys: Vec<TypeKey>) -> Vec<u32> {
		keys.into_iter()
			.map(|TypeKey { key, display }| {
				*self.type_ids_by_key.entry(key.clone()).or_insert_with(|| {
					self.data.type_names.push(display);
					self.type_keys.push(key);
					u32::try_from(self.data.type_names.len() - 1).expect("types fit in 32 bits")
				})
			})
			.collect()
	}

	fn link_types(&mut self) {
		let names = std::mem::take(&mut self.data.type_names);
		let mut order: Vec<usize> = (0..names.len()).collect();
		order.sort_by(|a, b| compare_names(&names[*a], &names[*b]));
		let mut sorted_ids = vec![0; names.len()];
		for (sorted, id) in order.iter().enumerate() {
			sorted_ids[*id] = u32::try_from(sorted).expect("types fit in 32 bits");
		}
		let remap = |ids: &mut Vec<Vec<u32>>| {
			for list in ids {
				for id in list {
					*id = sorted_ids[*id as usize];
				}
			}
		};
		remap(&mut self.data.value_types);
		remap(&mut self.data.parameter_types);
		self.data.type_entries = order
			.iter()
			.map(|id| self.primary_entries.get(&self.type_keys[*id]).copied())
			.collect();
		self.data.type_names = order.iter().map(|id| names[*id].clone()).collect();
	}

	fn rank_names(&mut self) {
		let names = &self.data.names;
		let mut order: Vec<usize> = (0..names.len()).collect();
		order.sort_by(|a, b| compare_names(&names[*a], &names[*b]));
		let mut ranks = vec![0; names.len()];
		let mut rank = 0;
		for (position, entry) in order.iter().enumerate() {
			if position == 0 || compare_names(&names[order[position - 1]], &names[*entry]).is_ne() {
				rank += 1;
			}
			ranks[*entry] = rank;
		}
		self.data.link_users(&ranks);
	}
}

fn removed_entry(
	kind: EntryKind,
	name: &str,
	parent: Option<&str>,
	removed: &HistoryEvent,
) -> RemovedData {
	RemovedData {
		kind,
		name: name.to_owned(),
		parent: parent.map(str::to_owned),
		version: removed.version.clone(),
		published: removed.published.clone(),
	}
}

fn removed_entries(api: &Api) -> Vec<RemovedData> {
	let owners = |kind, list: &[RemovedOwner]| {
		list.iter()
			.map(|owner| removed_entry(kind, &owner.name, None, &owner.removed))
			.collect::<Vec<_>>()
	};
	let mut entries = owners(EntryKind::Class, api.removed_classes());
	entries.extend(owners(EntryKind::Enum, api.removed_enums()));
	for item in api.classes.values() {
		for removed in item.removed_members.iter().flatten() {
			let kind = match removed.value.kind() {
				rapi_routes::MemberKind::Property => EntryKind::Property,
				rapi_routes::MemberKind::Method => EntryKind::Method,
				rapi_routes::MemberKind::Event => EntryKind::Event,
				rapi_routes::MemberKind::Callback => EntryKind::Callback,
			};
			entries.push(removed_entry(
				kind,
				&removed.value.name,
				Some(&item.name),
				&removed.removed,
			));
		}
	}
	for item in api.enums.values() {
		for removed in item.removed_items.iter().flatten() {
			entries.push(removed_entry(
				EntryKind::EnumItem,
				&removed.value.name,
				Some(&item.name),
				&removed.removed,
			));
		}
	}
	entries
}

fn member_entry_kind(member: &Member) -> EntryKind {
	match member.kind() {
		rapi_routes::MemberKind::Property => EntryKind::Property,
		rapi_routes::MemberKind::Method => EntryKind::Method,
		rapi_routes::MemberKind::Event => EntryKind::Event,
		rapi_routes::MemberKind::Callback => EntryKind::Callback,
	}
}

pub fn build_search_index(api: &Api, updates: &[Update]) -> IndexData {
	let mut builder = Builder::default();
	builder.data.removed = removed_entries(api);
	builder.data.updates = updates
		.iter()
		.map(|update| UpdateData {
			version: update.version.clone(),
			hash: update.hash.clone(),
			published: update.published.clone(),
			date: format_date(&update.published),
		})
		.collect();
	let mut classes: Vec<&Class> = api.classes.values().collect();
	classes.sort_by(|a, b| compare_names(&a.name, &b.name));
	let mut class_entries: HashMap<&str, u32> = HashMap::new();
	for item in &classes {
		let entry = builder.add(EntryKind::Class, &item.name, None, &item.tags, 0);
		builder.data.class_icons.push(item.icon);
		builder
			.primary_entries
			.insert(format!("Class:{}", item.name), entry);
		class_entries.insert(&item.name, entry);
	}
	for item in &classes {
		let entry = class_entries[item.name.as_str()];
		builder.data.parents[entry as usize] = item
			.superclass
			.as_deref()
			.and_then(|name| class_entries.get(name).copied());
	}
	for item in &classes {
		let parent = class_entries[item.name.as_str()];
		for member in &item.members {
			let security = u8::try_from(access_rank(member)).unwrap_or(u8::MAX);
			let entry = builder.add(
				member_entry_kind(member),
				&member.name,
				Some(parent),
				&member.tags,
				security,
			) as usize;
			let value_types = builder.type_ids(value_type_keys(member));
			let parameter_types = builder.type_ids(parameter_type_keys(member));
			builder.data.value_types[entry] = value_types;
			builder.data.parameter_types[entry] = parameter_types;
			builder.data.flags[entry] |= (if is_hidden(member) { flags::HIDDEN } else { 0 })
				| (if is_internal(member) {
					flags::INTERNAL
				} else {
					0
				});
		}
	}
	let mut enums: Vec<_> = api.enums.values().collect();
	enums.sort_by(|a, b| compare_names(&a.name, &b.name));
	for item in enums {
		let parent = builder.add(EntryKind::Enum, &item.name, None, &item.tags, 0);
		builder
			.primary_entries
			.insert(format!("Enum:{}", item.name), parent);
		for entry in &item.items {
			builder.add(
				EntryKind::EnumItem,
				&entry.name,
				Some(parent),
				&entry.tags,
				0,
			);
		}
	}
	let mut data_types: Vec<_> = api.data_types.values().collect();
	data_types.sort_by(|a, b| compare_names(&a.name, &b.name));
	for item in data_types {
		let entry = builder.add(EntryKind::DataType, &item.name, None, &[], 0);
		builder
			.primary_entries
			.insert(format!("{}:{}", item.category.name(), item.name), entry);
	}
	builder.link_types();
	builder.rank_names();
	builder.data
}
