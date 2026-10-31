use full_moon::ast::luau::{IndexedTypeInfo, TypeInfo};

use crate::generate::docs::ApiNames;
use crate::model::{ApiType, DataTypeCategory, Member, TypeCategory};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
	Unknown,
	Object(String),
	Value(String),
	Static(String),
	Constructor(String),
	EnumRoot,
	Enum(String),
	EnumItem(String),
	Event(Vec<Ty>),
	Library,
}

pub fn find_member<'a>(
	names: &ApiNames<'a>,
	class_name: &str,
	member_name: &str,
) -> Option<&'a Member> {
	let mut current = names.classes.get(class_name);
	while let Some(item) = current {
		if let Some(member) = item.members.iter().find(|member| member.name == member_name) {
			return Some(member);
		}
		current = item
			.superclass
			.as_ref()
			.and_then(|name| names.classes.get(name));
	}
	None
}

pub fn is_service(names: &ApiNames<'_>, class_name: &str, child: &str) -> bool {
	class_name == "DataModel"
		&& names
			.classes
			.get(child)
			.is_some_and(|item| item.tags.iter().any(|tag| tag == "Service"))
}

pub fn of_api_type(ty: &ApiType) -> Ty {
	match ty.category {
		TypeCategory::Class => Ty::Object(ty.name.clone()),
		TypeCategory::DataType => Ty::Value(ty.name.clone()),
		TypeCategory::Enum => Ty::EnumItem(ty.name.clone()),
		TypeCategory::Primitive | TypeCategory::Group => Ty::Unknown,
	}
}

pub fn is_static_data_type(names: &ApiNames<'_>, name: &str) -> bool {
	name == "Instance"
		|| names
			.data_types
			.get(name)
			.is_some_and(|item| item.category == DataTypeCategory::DataType)
}

pub fn named(names: &ApiNames<'_>, name: &str) -> Ty {
	if names.classes.contains_key(name) {
		Ty::Object(name.to_owned())
	} else if names.data_types.contains_key(name) {
		Ty::Value(name.to_owned())
	} else {
		Ty::Unknown
	}
}

pub fn of_annotation(names: &ApiNames<'_>, info: &TypeInfo) -> Ty {
	match info {
		TypeInfo::Basic(token) => named(names, &token.token().to_string()),
		TypeInfo::Optional { base, .. } => of_annotation(names, base),
		TypeInfo::Module {
			module, type_info, ..
		} if module.token().to_string() == "Enum" => match type_info.as_ref() {
			IndexedTypeInfo::Basic(token) => Ty::EnumItem(token.token().to_string()),
			_ => Ty::Unknown,
		},
		_ => Ty::Unknown,
	}
}
