import type { ApiType, Member } from "../../src/api/types.ts"

export type TypeKey = {
	key: string
	display: string
}

function flatten(type: ApiType): ApiType[] {
	return [type, ...(type.members ?? []).flatMap(flatten)]
}

function keyOf(type: ApiType): TypeKey | undefined {
	if (type.name === "Tuple" && (type.members?.length ?? 0) > 0) {
		return undefined
	}
	return {
		key: `${type.category}:${type.name}`,
		display: type.category === "Enum" ? `Enum.${type.name}` : type.name,
	}
}

function keysOf(types: ApiType[]): TypeKey[] {
	const seen = new Set<string>()
	return types
		.flatMap(flatten)
		.flatMap((type) => keyOf(type) ?? [])
		.filter((key) => {
			if (seen.has(key.key)) {
				return false
			}
			seen.add(key.key)
			return true
		})
}

export function valueTypeKeys(member: Member): TypeKey[] {
	switch (member.kind) {
		case "Property":
			return keysOf([member.type])
		case "Method":
		case "Callback":
			return keysOf([member.returns])
		case "Event":
			return []
	}
}

export function parameterTypeKeys(member: Member): TypeKey[] {
	return member.kind === "Property"
		? []
		: keysOf(member.parameters.map((parameter) => parameter.type))
}
