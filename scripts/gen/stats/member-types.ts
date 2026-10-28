import type { ApiType, Member } from "../../../src/api/types.ts"

function flatten(type: ApiType): ApiType[] {
	return [type, ...(type.members ?? []).flatMap(flatten)]
}

export function memberTypes(member: Member): ApiType[] {
	const parameters = member.kind === "Property" ? [] : member.parameters
	const types = [
		...(member.kind === "Property" ? [member.type] : []),
		...parameters.map((parameter) => parameter.type),
		...(member.kind === "Method" || member.kind === "Callback"
			? [member.returns]
			: []),
	]
	return types.flatMap(flatten)
}
