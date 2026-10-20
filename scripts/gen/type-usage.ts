import type {
	ApiType,
	Class,
	Member,
	TypeRole,
	TypeUsage,
} from "../../src/api/types.ts"

type TypeUse = {
	type: ApiType
	role: TypeRole
}

function flatten(type: ApiType): ApiType[] {
	return [type, ...(type.members ?? []).flatMap(flatten)]
}

function usesOf(types: ApiType[], role: TypeRole): TypeUse[] {
	return types.flatMap(flatten).map((type) => ({ type, role }))
}

function memberTypeUses(member: Member): TypeUse[] {
	switch (member.kind) {
		case "Property":
			return usesOf([member.type], "Value")
		case "Event":
			return usesOf(
				member.parameters.map((parameter) => parameter.type),
				"Parameter",
			)
		case "Method":
		case "Callback":
			return [
				...usesOf(
					member.parameters.map((parameter) => parameter.type),
					"Parameter",
				),
				...usesOf([member.returns], "Return"),
			]
	}
}

export type TypeUsageIndex = Map<string, TypeUsage[]>

function usageKey(type: ApiType): string {
	return `${type.category}:${type.name}`
}

export function indexTypeUsage(classes: Class[]): TypeUsageIndex {
	const index: TypeUsageIndex = new Map()
	for (const item of classes) {
		for (const member of item.members) {
			const seen = new Set<string>()
			for (const use of memberTypeUses(member)) {
				const key = `${usageKey(use.type)}:${use.role}`
				if (seen.has(key)) {
					continue
				}
				seen.add(key)
				const list = index.get(usageKey(use.type)) ?? []
				list.push({
					className: item.name,
					memberName: member.name,
					kind: member.kind,
					role: use.role,
				})
				index.set(usageKey(use.type), list)
			}
		}
	}
	return index
}

export function usageOf(
	index: TypeUsageIndex,
	category: ApiType["category"],
	name: string,
): TypeUsage[] {
	return index.get(`${category}:${name}`) ?? []
}

export function usedTypeKeys(
	index: TypeUsageIndex,
): { category: string; name: string }[] {
	return [...index.keys()].map((key) => {
		const separator = key.indexOf(":")
		return { category: key.slice(0, separator), name: key.slice(separator + 1) }
	})
}
