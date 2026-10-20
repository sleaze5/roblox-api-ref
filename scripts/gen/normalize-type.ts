import type { ApiType, Security, TypeCategory } from "../../src/api/types.ts"
import { securityLevels } from "../../src/api/types.ts"
import type { DumpType } from "./dump-types.ts"

const categories = new Set<string>(["Primitive", "Class", "DataType", "Enum", "Group"])

export function normalizeType(type: DumpType): ApiType {
	const optional = type.Name.endsWith("?")
	const name = optional ? type.Name.slice(0, -1) : type.Name
	const category: TypeCategory =
		type.Category !== undefined && categories.has(type.Category)
			? (type.Category as TypeCategory)
			: "Primitive"
	return { category, name, optional }
}

export function normalizeReturns(returnType: DumpType | DumpType[]): ApiType {
	if (!Array.isArray(returnType)) {
		return normalizeType(returnType)
	}
	return {
		category: "Group",
		name: "Tuple",
		optional: false,
		members: returnType.map(normalizeType),
	}
}

export function normalizeSecurity(security: string): Security {
	return securityLevels.find((level) => level === security) ?? "None"
}
