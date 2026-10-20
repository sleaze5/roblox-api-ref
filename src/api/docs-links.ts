const docsBase = "https://create.roblox.com/docs"

const documentedDataTypes = new Set([
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
])

const otherTypePages: Record<string, string> = {
	CoordinateFrame: "reference/engine/datatypes/CFrame",
	OptionalCoordinateFrame: "reference/engine/datatypes/CFrame",
	buffer: "reference/engine/libraries/buffer",
	bool: "luau/booleans",
	int: "luau/numbers",
	int64: "luau/numbers",
	float: "luau/numbers",
	double: "luau/numbers",
	string: "luau/strings",
	ContentId: "luau/strings",
	ProtectedString: "luau/strings",
	BinaryString: "luau/strings",
	Array: "luau/tables#arrays",
	Instances: "luau/tables#arrays",
	Dictionary: "luau/tables#dictionaries",
	Map: "luau/tables#dictionaries",
	Tuple: "luau/tuples",
	Function: "luau/functions",
}

export function classDocsHref(className: string, memberName?: string): string {
	const page = `${docsBase}/reference/engine/classes/${className}`
	return memberName === undefined ? page : `${page}#${memberName}`
}

export function enumDocsHref(enumName: string): string {
	return `${docsBase}/reference/engine/enums/${enumName}`
}

export function dataTypeDocsHref(name: string): string | undefined {
	if (documentedDataTypes.has(name)) {
		return `${docsBase}/reference/engine/datatypes/${name}`
	}
	const page = otherTypePages[name]
	return page === undefined ? undefined : `${docsBase}/${page}`
}
