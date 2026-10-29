const DOCS_BASE: &str = "https://create.roblox.com/docs";

const DOCUMENTED_DATA_TYPES: [&str; 37] = [
	"Axes",
	"BrickColor",
	"CatalogSearchParams",
	"CFrame",
	"Color3",
	"ColorSequence",
	"Content",
	"DateTime",
	"DockWidgetPluginGuiInfo",
	"Faces",
	"FloatCurveKey",
	"Font",
	"NumberRange",
	"NumberSequence",
	"OverlapParams",
	"Path2DControlPoint",
	"PhysicalProperties",
	"Ray",
	"RaycastParams",
	"RaycastResult",
	"RBXScriptConnection",
	"RBXScriptSignal",
	"Rect",
	"Region3",
	"Region3int16",
	"RotationCurveKey",
	"Secret",
	"SecurityCapabilities",
	"SharedTable",
	"TweenInfo",
	"UDim",
	"UDim2",
	"ValueCurveKey",
	"Vector2",
	"Vector2int16",
	"Vector3",
	"Vector3int16",
];

fn other_type_page(name: &str) -> Option<&'static str> {
	Some(match name {
		"CoordinateFrame" | "OptionalCoordinateFrame" => "reference/engine/datatypes/CFrame",
		"buffer" => "reference/engine/libraries/buffer",
		"bool" => "luau/booleans",
		"int" | "int64" | "float" | "double" => "luau/numbers",
		"string" | "ContentId" | "ProtectedString" | "BinaryString" => "luau/strings",
		"Array" | "Instances" => "luau/tables#arrays",
		"Dictionary" | "Map" => "luau/tables#dictionaries",
		"Tuple" => "luau/tuples",
		"Function" => "luau/functions",
		_ => return None,
	})
}

pub fn class_docs_href(class_name: &str, member_name: Option<&str>) -> String {
	let page = format!("{DOCS_BASE}/reference/engine/classes/{class_name}");
	match member_name {
		Some(member) => format!("{page}#{member}"),
		None => page,
	}
}

pub fn enum_docs_href(enum_name: &str) -> String {
	format!("{DOCS_BASE}/reference/engine/enums/{enum_name}")
}

pub fn data_type_docs_href(name: &str) -> Option<String> {
	if DOCUMENTED_DATA_TYPES.contains(&name) {
		return Some(format!("{DOCS_BASE}/reference/engine/datatypes/{name}"));
	}
	other_type_page(name).map(|page| format!("{DOCS_BASE}/{page}"))
}
