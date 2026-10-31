use crate::generate::docs::ApiNames;
use crate::model::DataTypeCategory;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ty {
	Unknown,
	Static,
	EnumRoot,
	Enum,
}

pub fn is_static_data_type(names: &ApiNames<'_>, name: &str) -> bool {
	name == "Instance"
		|| names
			.data_types
			.get(name)
			.is_some_and(|item| item.category == DataTypeCategory::DataType)
}
