import type {
	Class,
	Enum,
	Facet,
	FacetAccess,
	FacetKind,
	Member,
} from "../../../src/api/types.ts"
import {
	addUse,
	createFacetIndex,
	finishFacets,
	type FacetIndex,
} from "./facet-index.ts"
import { splitAccess } from "./split-access.ts"

function writeSecurity(member: Member): string {
	return member.kind === "Property" ? member.writeSecurity : member.security
}

function indexMember(index: FacetIndex, owner: string, member: Member): void {
	const target = { kind: member.kind, owner, name: member.name }
	const withAccess = (access: FacetAccess | undefined) =>
		access === undefined ? target : { ...target, access }
	if (member.kind === "Property") {
		addUse(index, "propertyCategory", member.category, target)
	}
	for (const tag of member.tags) {
		addUse(index, "tag", tag, target)
	}
	for (const value of splitAccess([member.security], [writeSecurity(member)])) {
		addUse(index, "security", value.name, withAccess(value.access))
	}
	const { read, write } = member.capabilities
	for (const value of splitAccess(read, write)) {
		addUse(index, "capability", value.name, withAccess(value.access))
	}
	if (member.threadSafety !== "Unknown") {
		addUse(index, "threadSafety", member.threadSafety, target)
	}
}

function indexClass(index: FacetIndex, item: Class): void {
	const target = { kind: "Class", name: item.name } as const
	for (const tag of item.tags) {
		addUse(index, "tag", tag, target)
	}
	addUse(index, "memoryCategory", item.memoryCategory, target)
	for (const member of item.members) {
		indexMember(index, item.name, member)
	}
}

function indexEnum(index: FacetIndex, item: Enum): void {
	for (const tag of item.tags) {
		addUse(index, "tag", tag, { kind: "Enum", name: item.name })
	}
	for (const enumItem of item.items) {
		for (const tag of enumItem.tags) {
			addUse(index, "tag", tag, {
				kind: "EnumItem",
				owner: item.name,
				name: enumItem.name,
			})
		}
	}
}

export function collectFacets(
	classes: Class[],
	enums: Enum[],
): Record<FacetKind, Record<string, Facet>> {
	const index = createFacetIndex()
	for (const item of classes) {
		indexClass(index, item)
	}
	for (const item of enums) {
		indexEnum(index, item)
	}
	return finishFacets(index)
}
