use super::ty::{Ty, is_static_data_type};
use crate::generate::docs::ApiNames;
use crate::model::DocLink;

pub struct Global {
	pub ty: Ty,
	pub link: Option<DocLink>,
	pub builtin: bool,
}

const LIBRARIES: [&str; 11] = [
	"bit32",
	"buffer",
	"coroutine",
	"debug",
	"math",
	"os",
	"string",
	"table",
	"task",
	"utf8",
	"vector",
];

const VARIABLES: [&str; 4] = ["_G", "_VERSION", "self", "shared"];

fn builtin(ty: Ty, class_name: Option<&str>) -> Global {
	Global {
		ty,
		link: class_name.map(|name| DocLink::Class {
			name: name.to_owned(),
		}),
		builtin: true,
	}
}

pub fn global(names: &ApiNames<'_>, name: &str) -> Option<Global> {
	Some(match name {
		"game" => builtin(Ty::Object("DataModel".to_owned()), Some("DataModel")),
		"workspace" => builtin(Ty::Object("Workspace".to_owned()), Some("Workspace")),
		"plugin" => builtin(Ty::Object("Plugin".to_owned()), Some("Plugin")),
		"script" => builtin(Ty::Object("LuaSourceContainer".to_owned()), None),
		"Enum" => Global {
			ty: Ty::EnumRoot,
			link: None,
			builtin: false,
		},
		_ if LIBRARIES.contains(&name) => builtin(Ty::Library, None),
		_ if VARIABLES.contains(&name) => builtin(Ty::Unknown, None),
		_ if is_static_data_type(names, name) => Global {
			ty: Ty::Static(name.to_owned()),
			link: names.type_link(name),
			builtin: false,
		},
		_ => return None,
	})
}
