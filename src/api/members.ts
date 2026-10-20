import { compareNames } from "./compare-names.ts"
import { isInternalSecurity, securityRank } from "./tags.ts"
import type { Api, Class, Member, MemberKind } from "./types.ts"

export const memberKinds: readonly MemberKind[] = [
	"Property",
	"Method",
	"Event",
	"Callback",
]

export const memberKindTitles: Record<MemberKind, string> = {
	Property: "Properties",
	Method: "Methods",
	Event: "Events",
	Callback: "Callbacks",
}

export function isDeprecated(member: { tags: string[] }): boolean {
	return member.tags.includes("Deprecated")
}

export function isHidden(member: Member): boolean {
	return (
		member.tags.includes("Hidden") ||
		member.tags.includes("NotBrowsable") ||
		member.capabilities.read.includes("InternalTest") ||
		member.capabilities.write.includes("InternalTest")
	)
}

export function accessRank(member: Member): number {
	if (member.kind === "Property") {
		return Math.min(
			securityRank(member.security),
			securityRank(member.writeSecurity),
		)
	}
	return securityRank(member.security)
}

export function isInternal(member: Member): boolean {
	if (member.kind === "Property") {
		return (
			isInternalSecurity(member.security) &&
			isInternalSecurity(member.writeSecurity)
		)
	}
	return isInternalSecurity(member.security)
}

function isCamelCase(name: string): boolean {
	const first = name.charAt(0)
	return first !== first.toUpperCase()
}

export function compareMembers(a: Member, b: Member): number {
	const deprecated = Number(isDeprecated(a)) - Number(isDeprecated(b))
	if (deprecated !== 0) {
		return deprecated
	}
	const camel = Number(isCamelCase(a.name)) - Number(isCamelCase(b.name))
	if (camel !== 0) {
		return camel
	}
	return compareNames(a.name, b.name)
}

export function membersOfKind(members: Member[], kind: MemberKind): Member[] {
	return members.filter((member) => member.kind === kind).sort(compareMembers)
}

export function ancestorsOf(api: Api, item: Class): Class[] {
	const ancestors: Class[] = []
	let current =
		item.superclass === undefined ? undefined : api.classes[item.superclass]
	while (current !== undefined) {
		ancestors.push(current)
		current =
			current.superclass === undefined
				? undefined
				: api.classes[current.superclass]
	}
	return ancestors
}

export type PropertyGroup = {
	category: string
	members: Member[]
}

export function propertyGroups(members: Member[]): PropertyGroup[] {
	const groups = new Map<string, Member[]>()
	for (const member of membersOfKind(members, "Property")) {
		const category = member.kind === "Property" ? member.category : ""
		const list = groups.get(category) ?? []
		list.push(member)
		groups.set(category, list)
	}
	return [...groups]
		.sort(([a], [b]) => compareNames(a, b))
		.map(([category, list]) => ({ category, members: list }))
}

export function membersInPageOrder(members: Member[], kind: MemberKind): Member[] {
	return kind === "Property"
		? propertyGroups(members).flatMap((group) => group.members)
		: membersOfKind(members, kind)
}
