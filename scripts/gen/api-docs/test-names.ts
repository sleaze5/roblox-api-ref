import type { Class, Enum, Property } from "../../../src/api/types.ts"
import type { ApiNames } from "./doc/api-names.ts"

function property(name: string): Property {
	return {
		kind: "Property",
		name,
		category: "Data",
		type: { category: "Primitive", name: "bool", optional: false },
		tags: [],
		security: "None",
		writeSecurity: "None",
		threadSafety: "Safe",
		capabilities: { read: [], write: [] },
		canLoad: true,
		canSave: true,
		simulationAccess: false,
	}
}

function owner(name: string, members: Class["members"], superclass?: string): Class {
	return {
		name,
		members,
		memoryCategory: "Instances",
		tags: [],
		subclasses: [],
		...(superclass === undefined ? {} : { superclass }),
	}
}

export function testNames(): ApiNames {
	const material: Enum = {
		name: "Material",
		tags: [],
		items: [{ name: "Plastic", value: 256, tags: [], legacyNames: [] }],
		usedBy: [],
	}
	return {
		classes: {
			Instance: owner("Instance", [
				{
					kind: "Method",
					name: "FindFirstChild",
					parameters: [
						{
							name: "name",
							type: {
								category: "Primitive",
								name: "string",
								optional: false,
							},
						},
						{
							name: "recursive",
							type: {
								category: "Primitive",
								name: "bool",
								optional: false,
							},
							default: "false",
						},
					],
					returns: { category: "Class", name: "Instance", optional: true },
					tags: [],
					security: "None",
					threadSafety: "Safe",
					capabilities: { read: [], write: [] },
					simulationAccess: false,
				},
			]),
			BasePart: owner("BasePart", [property("Anchored")], "Instance"),
			Part: owner("Part", [], "BasePart"),
		},
		enums: { Material: material },
		dataTypes: { Vector3: { name: "Vector3", category: "DataType", usedBy: [] } },
	}
}
