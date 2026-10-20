import { compareNames } from "./compare-names.ts"
import type { DataType, DataTypeCategory } from "./types.ts"

export const dataTypeCategoryTitles: Record<DataTypeCategory, string> = {
	Primitive: "Primitive types",
	Group: "Group types",
	DataType: "Roblox types",
}

export const dataTypeCategories: readonly DataTypeCategory[] = [
	"Primitive",
	"Group",
	"DataType",
]

export function sortedDataTypes(dataTypes: Record<string, DataType>): DataType[] {
	return Object.values(dataTypes).sort((a, b) => compareNames(a.name, b.name))
}
