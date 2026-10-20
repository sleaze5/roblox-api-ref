import { compareNames } from "../../src/api/compare-names.ts"
import type {
	Api,
	Class,
	DataType,
	DataTypeCategory,
	Enum,
} from "../../src/api/types.ts"
import type { RobloxVersion } from "../roblox-version.ts"
import { applyApiDocs } from "./api-docs/apply-api-docs.ts"
import type { ApiDocs } from "./api-docs/api-docs-types.ts"
import type { Dump } from "./dump-types.ts"
import { collectFacets } from "./facets/collect-facets.ts"
import { normalizeMember, normalizeTags } from "./normalize-member.ts"
import { resolvePreferredMembers } from "./resolve-preferred.ts"
import {
	indexTypeUsage,
	usageOf,
	usedTypeKeys,
	type TypeUsageIndex,
} from "./type-usage.ts"

const rootSuperclass = "<<<ROOT>>>"
const dataTypeCategories = new Set<string>(["Primitive", "DataType", "Group"])

function byName(a: { name: string }, b: { name: string }): number {
	return compareNames(a.name, b.name)
}

function normalizeClasses(dump: Dump): Record<string, Class> {
	const classes: Record<string, Class> = {}
	for (const dumpClass of dump.Classes) {
		const normalized: Class = {
			name: dumpClass.Name,
			memoryCategory: dumpClass.MemoryCategory,
			tags: normalizeTags(dumpClass.Tags),
			members: dumpClass.Members.map(normalizeMember).sort(byName),
			subclasses: [],
		}
		if (dumpClass.Superclass !== rootSuperclass) {
			normalized.superclass = dumpClass.Superclass
		}
		classes[dumpClass.Name] = normalized
	}
	for (const item of Object.values(classes)) {
		if (item.superclass !== undefined) {
			classes[item.superclass]?.subclasses.push(item.name)
		}
	}
	for (const item of Object.values(classes)) {
		item.subclasses.sort((a, b) => compareNames(a, b))
	}
	return classes
}

function normalizeEnums(dump: Dump, usage: TypeUsageIndex): Record<string, Enum> {
	const enums: Record<string, Enum> = {}
	for (const dumpEnum of dump.Enums) {
		enums[dumpEnum.Name] = {
			name: dumpEnum.Name,
			tags: normalizeTags(dumpEnum.Tags),
			items: dumpEnum.Items.map((item) => ({
				name: item.Name,
				value: item.Value,
				tags: normalizeTags(item.Tags),
				legacyNames: item.LegacyNames ?? [],
			})).sort((a, b) => a.value - b.value),
			usedBy: usageOf(usage, "Enum", dumpEnum.Name),
		}
	}
	return enums
}

function collectDataTypes(usage: TypeUsageIndex): Record<string, DataType> {
	const dataTypes: Record<string, DataType> = {}
	const found = usedTypeKeys(usage)
		.filter((key) => dataTypeCategories.has(key.category))
		.sort(byName)
	for (const { category, name } of found) {
		dataTypes[name] = {
			name,
			category: category as DataTypeCategory,
			usedBy: usageOf(usage, category as DataTypeCategory, name),
		}
	}
	return dataTypes
}

export function normalizeApi(dump: Dump, docs: ApiDocs, version: RobloxVersion): Api {
	const classes = normalizeClasses(dump)
	resolvePreferredMembers(classes)
	const usage = indexTypeUsage(Object.values(classes))
	const enums = normalizeEnums(dump, usage)
	const dataTypes = collectDataTypes(usage)
	applyApiDocs({ classes, enums, dataTypes }, docs)
	return {
		version: version.version,
		hash: version.hash,
		channel: version.channel,
		published: version.published,
		classes,
		enums,
		dataTypes,
		facets: collectFacets(Object.values(classes), Object.values(enums)),
	}
}
