use indexmap::IndexMap;
use rapi_routes::MemberKind;

use super::docs::apply_api_docs;
use super::facets::collect_facets;
use crate::dump::{
	ApiDocs, Dump, DumpCapabilities, DumpEnumItem, DumpMember, DumpParameter, DumpReturns,
	DumpSecurity, DumpTag, DumpType, RobloxVersion,
};
use crate::js::{compare_code_units, compare_names};
use crate::model::{
	Api, ApiType, Capabilities, Class, DataType, DataTypeCategory, Enum, EnumItem, Member,
	MemberDetail, Parameter, PreferredTarget, Security, TypeCategory, TypeRole, TypeUsage,
};

const ROOT_SUPERCLASS: &str = "<<<ROOT>>>";

pub fn string_tags(tags: Option<&[DumpTag]>) -> Vec<String> {
	tags.unwrap_or_default()
		.iter()
		.filter_map(|tag| match tag {
			DumpTag::Name(name) => Some(name.clone()),
			DumpTag::Detail(_) => None,
		})
		.collect()
}

pub fn normalize_tags(tags: Option<&[DumpTag]>) -> Vec<String> {
	let mut names = string_tags(tags);
	if names.iter().any(|tag| tag == "ReadOnly") {
		names.retain(|tag| tag != "NotReplicated");
	}
	let mut unique: Vec<String> = Vec::with_capacity(names.len());
	for name in names {
		if !unique.contains(&name) {
			unique.push(name);
		}
	}
	unique.sort_by(|a, b| compare_code_units(a, b));
	unique
}

pub fn preferred_name(tags: Option<&[DumpTag]>) -> Option<String> {
	tags.unwrap_or_default().iter().find_map(|tag| match tag {
		DumpTag::Detail(detail) => detail.preferred_descriptor_name.clone(),
		DumpTag::Name(_) => None,
	})
}

fn sorted(mut names: Vec<String>) -> Vec<String> {
	names.sort_by(|a, b| compare_code_units(a, b));
	names
}

pub fn normalize_capabilities(capabilities: Option<&DumpCapabilities>) -> Capabilities {
	match capabilities {
		None => Capabilities::default(),
		Some(DumpCapabilities::List(list)) => {
			let list = sorted(list.clone());
			Capabilities {
				read: list.clone(),
				write: list,
			}
		}
		Some(DumpCapabilities::Split { read, write }) => Capabilities {
			read: sorted(read.clone().unwrap_or_default()),
			write: sorted(write.clone().unwrap_or_default()),
		},
	}
}

pub fn normalize_security(security: &str) -> Security {
	Security::parse(security).unwrap_or(Security::None)
}

fn type_category(category: Option<&str>) -> TypeCategory {
	match category {
		Some("Class") => TypeCategory::Class,
		Some("DataType") => TypeCategory::DataType,
		Some("Enum") => TypeCategory::Enum,
		Some("Group") => TypeCategory::Group,
		_ => TypeCategory::Primitive,
	}
}

pub fn normalize_type(ty: &DumpType) -> ApiType {
	let optional = ty.name.ends_with('?');
	ApiType {
		category: type_category(ty.category.as_deref()),
		name: ty.name.strip_suffix('?').unwrap_or(&ty.name).to_owned(),
		optional,
		members: None,
	}
}

fn normalize_returns(returns: Option<&DumpReturns>) -> ApiType {
	match returns {
		Some(DumpReturns::One(ty)) => normalize_type(ty),
		Some(DumpReturns::Many(types)) => ApiType {
			category: TypeCategory::Group,
			name: "Tuple".to_owned(),
			optional: false,
			members: Some(types.iter().map(normalize_type).collect()),
		},
		None => ApiType {
			category: TypeCategory::Primitive,
			name: String::new(),
			optional: false,
			members: None,
		},
	}
}

fn normalize_parameter(parameter: &DumpParameter) -> Parameter {
	Parameter {
		name: parameter.name.clone(),
		ty: normalize_type(&parameter.ty),
		default: parameter.default.clone(),
		description: None,
	}
}

fn securities(security: Option<&DumpSecurity>) -> (Security, Security) {
	match security {
		Some(DumpSecurity::Single(level)) => {
			let level = normalize_security(level);
			(level, level)
		}
		Some(DumpSecurity::Split { read, write }) => {
			(normalize_security(read), normalize_security(write))
		}
		None => (Security::None, Security::None),
	}
}

fn parameters(member: &DumpMember) -> Vec<Parameter> {
	member
		.parameters
		.iter()
		.flatten()
		.map(normalize_parameter)
		.collect()
}

pub fn normalize_member(member: &DumpMember) -> Member {
	let (read, write) = securities(member.security.as_ref());
	let simulation_access = member.simulation_access.unwrap_or(false);
	let detail = match member.member_type.as_str() {
		"Property" => MemberDetail::Property {
			ty: member
				.value_type
				.as_ref()
				.map_or_else(|| normalize_returns(None), normalize_type),
			category: member.category.clone().unwrap_or_default(),
			default: member
				.default
				.clone()
				.filter(|value| !value.starts_with("__api_dump")),
			write_security: write,
			can_load: member
				.serialization
				.is_some_and(|serialization| serialization.can_load),
			can_save: member
				.serialization
				.is_some_and(|serialization| serialization.can_save),
			simulation_access,
		},
		"Function" => MemberDetail::Method {
			parameters: parameters(member),
			returns: normalize_returns(member.return_type.as_ref()),
			return_description: None,
			simulation_access,
		},
		"Event" => MemberDetail::Event {
			parameters: parameters(member),
		},
		_ => MemberDetail::Callback {
			parameters: parameters(member),
			returns: normalize_returns(member.return_type.as_ref()),
			return_description: None,
		},
	};
	Member {
		name: member.name.clone(),
		history: None,
		tags: normalize_tags(member.tags.as_deref()),
		security: read,
		thread_safety: member
			.thread_safety
			.clone()
			.unwrap_or_else(|| "Unknown".to_owned()),
		capabilities: normalize_capabilities(member.capabilities.as_ref()),
		preferred: preferred_name(member.tags.as_deref()),
		preferred_target: None,
		description: None,
		detail,
	}
}

pub fn normalize_enum_item(item: &DumpEnumItem) -> EnumItem {
	EnumItem {
		name: item.name.clone(),
		history: None,
		value: item.value,
		tags: normalize_tags(item.tags.as_deref()),
		legacy_names: item.legacy_names.clone().unwrap_or_default(),
		description: None,
	}
}

fn normalize_classes(dump: &Dump) -> IndexMap<String, Class> {
	let mut classes: IndexMap<String, Class> = IndexMap::new();
	for item in &dump.classes {
		let mut members: Vec<Member> = item.members.iter().map(normalize_member).collect();
		members.sort_by(|a, b| compare_names(&a.name, &b.name));
		classes.insert(
			item.name.clone(),
			Class {
				name: item.name.clone(),
				history: None,
				icon: None,
				superclass: (item.superclass != ROOT_SUPERCLASS).then(|| item.superclass.clone()),
				memory_category: item.memory_category.clone(),
				tags: normalize_tags(item.tags.as_deref()),
				members,
				removed_members: None,
				subclasses: Vec::new(),
				used_by: Vec::new(),
				description: None,
			},
		);
	}
	let links: Vec<(String, String)> = classes
		.values()
		.filter_map(|item| {
			item.superclass
				.clone()
				.map(|parent| (parent, item.name.clone()))
		})
		.collect();
	for (parent, child) in links {
		if let Some(parent) = classes.get_mut(&parent) {
			parent.subclasses.push(child);
		}
	}
	for item in classes.values_mut() {
		item.subclasses.sort_by(|a, b| compare_names(a, b));
	}
	classes
}

fn preferred_target(
	classes: &IndexMap<String, Class>,
	owner: &Class,
	name: &str,
) -> Option<PreferredTarget> {
	let mut current = Some(owner);
	while let Some(item) = current {
		if let Some(member) = item.members.iter().find(|candidate| candidate.name == name) {
			return Some(PreferredTarget {
				owner: item.name.clone(),
				kind: member.kind(),
			});
		}
		current = item
			.superclass
			.as_ref()
			.and_then(|parent| classes.get(parent));
	}
	None
}

pub fn resolve_preferred_members(classes: &mut IndexMap<String, Class>) {
	let targets: Vec<Vec<Option<PreferredTarget>>> = classes
		.values()
		.map(|owner| {
			owner
				.members
				.iter()
				.map(|member| {
					member
						.preferred
						.as_deref()
						.and_then(|name| preferred_target(classes, owner, name))
				})
				.collect()
		})
		.collect();
	for (owner, targets) in classes.values_mut().zip(targets) {
		for (member, target) in owner.members.iter_mut().zip(targets) {
			if target.is_some() {
				member.preferred_target = target;
			}
		}
	}
}

type UsageIndex = IndexMap<(TypeCategory, String), Vec<TypeUsage>>;

fn flatten_uses(types: Vec<&ApiType>, role: TypeRole) -> Vec<(&ApiType, TypeRole)> {
	types
		.into_iter()
		.flat_map(ApiType::flatten)
		.map(|ty| (ty, role))
		.collect()
}

fn member_type_uses(member: &Member) -> Vec<(&ApiType, TypeRole)> {
	let parameters: Vec<&ApiType> = member
		.parameters()
		.iter()
		.map(|parameter| &parameter.ty)
		.collect();
	match &member.detail {
		MemberDetail::Property { ty, .. } => flatten_uses(vec![ty], TypeRole::Value),
		MemberDetail::Event { .. } => flatten_uses(parameters, TypeRole::Parameter),
		MemberDetail::Method { returns, .. } | MemberDetail::Callback { returns, .. } => {
			let mut uses = flatten_uses(parameters, TypeRole::Parameter);
			uses.extend(flatten_uses(vec![returns], TypeRole::Return));
			uses
		}
	}
}

fn index_type_usage(classes: &IndexMap<String, Class>) -> UsageIndex {
	let mut index = UsageIndex::new();
	for item in classes.values() {
		for member in &item.members {
			let mut seen: Vec<(TypeCategory, &str, TypeRole)> = Vec::new();
			for (ty, role) in member_type_uses(member) {
				let key = (ty.category, ty.name.as_str(), role);
				if seen.contains(&key) {
					continue;
				}
				seen.push(key);
				index
					.entry((ty.category, ty.name.clone()))
					.or_default()
					.push(TypeUsage {
						class_name: item.name.clone(),
						member_name: member.name.clone(),
						kind: member.kind(),
						role,
					});
			}
		}
	}
	index
}

fn usage_of(index: &UsageIndex, category: TypeCategory, name: &str) -> Vec<TypeUsage> {
	index
		.get(&(category, name.to_owned()))
		.cloned()
		.unwrap_or_default()
}

fn normalize_enums(dump: &Dump, usage: &UsageIndex) -> IndexMap<String, Enum> {
	dump.enums
		.iter()
		.map(|item| {
			let mut items: Vec<EnumItem> = item.items.iter().map(normalize_enum_item).collect();
			items.sort_by_key(|entry| entry.value);
			(
				item.name.clone(),
				Enum {
					name: item.name.clone(),
					history: None,
					tags: normalize_tags(item.tags.as_deref()),
					items,
					removed_items: None,
					used_by: usage_of(usage, TypeCategory::Enum, &item.name),
					description: None,
				},
			)
		})
		.collect()
}

fn data_type_category(category: TypeCategory) -> Option<DataTypeCategory> {
	match category {
		TypeCategory::Primitive => Some(DataTypeCategory::Primitive),
		TypeCategory::DataType => Some(DataTypeCategory::DataType),
		TypeCategory::Group => Some(DataTypeCategory::Group),
		_ => None,
	}
}

fn collect_data_types(usage: &UsageIndex) -> IndexMap<String, DataType> {
	let mut found: Vec<(DataTypeCategory, &str)> = usage
		.keys()
		.filter_map(|(category, name)| {
			data_type_category(*category).map(|kind| (kind, name.as_str()))
		})
		.collect();
	found.sort_by(|a, b| compare_names(a.1, b.1));
	let mut data_types = IndexMap::new();
	for (category, name) in found {
		data_types.insert(
			name.to_owned(),
			DataType {
				name: name.to_owned(),
				category,
				used_by: usage_of(usage, category.type_category(), name),
			},
		);
	}
	data_types
}

pub fn normalize_api(dump: &Dump, docs: &ApiDocs, version: &RobloxVersion) -> Api {
	let mut classes = normalize_classes(dump);
	resolve_preferred_members(&mut classes);
	let usage = index_type_usage(&classes);
	for item in classes.values_mut() {
		item.used_by = usage_of(&usage, TypeCategory::Class, &item.name);
	}
	let mut enums = normalize_enums(dump, &usage);
	let data_types = collect_data_types(&usage);
	apply_api_docs(&mut classes, &mut enums, &data_types, docs);
	let facets = collect_facets(classes.values(), enums.values());
	Api {
		version: version.version.clone(),
		hash: version.hash.clone(),
		channel: version.channel.clone(),
		published: version.published.clone(),
		classes,
		enums,
		data_types,
		facets,
		history_start: None,
		removed: None,
	}
}

pub const fn member_kind_name(kind: MemberKind) -> &'static str {
	kind.name()
}
