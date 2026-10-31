use super::ty::{Ty, is_static_data_type};
use crate::api::code::CodeTone;
use crate::generate::docs::ApiNames;

const BUILTINS: [&str; 19] = [
	"_G",
	"_VERSION",
	"bit32",
	"buffer",
	"coroutine",
	"debug",
	"game",
	"math",
	"os",
	"plugin",
	"script",
	"self",
	"shared",
	"string",
	"table",
	"task",
	"utf8",
	"vector",
	"workspace",
];

pub fn global(names: &ApiNames<'_>, name: &str) -> Option<(CodeTone, Ty)> {
	if name == "Enum" {
		Some((CodeTone::Plain, Ty::EnumRoot))
	} else if BUILTINS.contains(&name) {
		Some((CodeTone::Constant, Ty::Unknown))
	} else if is_static_data_type(names, name) {
		Some((CodeTone::Type, Ty::Static))
	} else {
		None
	}
}
