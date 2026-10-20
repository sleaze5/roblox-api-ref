import type { DocLink } from "../../../../src/api/doc-types.ts"
import type { Class, DataType, Enum } from "../../../../src/api/types.ts"

export type ApiNames = {
	classes: Record<string, Class>
	enums: Record<string, Enum>
	dataTypes: Record<string, DataType>
}

export function memberLink(
	names: ApiNames,
	className: string,
	memberName: string,
): DocLink | undefined {
	let current = names.classes[className]
	while (current !== undefined) {
		const member = current.members.find((item) => item.name === memberName)
		if (member !== undefined) {
			return {
				kind: "member",
				owner: current.name,
				member: member.kind,
				name: member.name,
			}
		}
		current =
			current.superclass === undefined
				? undefined
				: names.classes[current.superclass]
	}
	return undefined
}

export function enumLink(
	names: ApiNames,
	enumName: string,
	itemName?: string,
): DocLink | undefined {
	const item = names.enums[enumName]
	if (item === undefined) {
		return undefined
	}
	if (itemName === undefined) {
		return { kind: "enum", name: enumName }
	}
	return item.items.some((entry) => entry.name === itemName)
		? { kind: "enumItem", owner: enumName, name: itemName }
		: undefined
}

export function typeLink(names: ApiNames, name: string): DocLink | undefined {
	if (Object.hasOwn(names.classes, name)) {
		return { kind: "class", name }
	}
	return Object.hasOwn(names.dataTypes, name) ? { kind: "type", name } : undefined
}
