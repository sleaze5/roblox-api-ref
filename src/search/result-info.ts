import {
	classHref,
	enumHref,
	enumItemHref,
	memberHref,
	typeHref,
} from "../api/routes.ts"
import type { TypeRole } from "../api/types.ts"
import type { SearchEngine, SearchResult, TypeUsers } from "./engine.ts"
import type { EntryKind } from "./index-format.ts"

export const kindTitles: Record<EntryKind, string> = {
	Class: "Classes",
	DataType: "Data types",
	Enum: "Enums",
	Property: "Properties",
	Method: "Methods",
	Event: "Events",
	Callback: "Callbacks",
	EnumItem: "Enum items",
}

export function resultHref(engine: SearchEngine, result: SearchResult): string {
	const kind = engine.kindOf(result.entry)
	const name = engine.nameOf(result.entry)
	const parent = engine.nameOf(engine.parentOf(result.entry))
	switch (kind) {
		case "Class":
			return classHref(name)
		case "DataType":
			return typeHref(name)
		case "Enum":
			return enumHref(name)
		case "EnumItem":
			return enumItemHref(parent, name)
		case "Property":
		case "Method":
		case "Event":
		case "Callback":
			return memberHref(parent, kind, name)
	}
}

export const roleLabels: Record<TypeRole, string> = {
	Value: "Property type",
	Parameter: "Parameter",
	Return: "Return value",
}

export type ResultGroup = {
	key: string
	title: string
	start: number
	results: SearchResult[]
	typeName?: string
}

function withStarts(groups: Omit<ResultGroup, "start">[]): ResultGroup[] {
	let start = 0
	return groups.map((group) => {
		const placed = { ...group, start }
		start += group.results.length
		return placed
	})
}

function kindGroups(
	engine: SearchEngine,
	results: SearchResult[],
	perGroup: number,
): Omit<ResultGroup, "start">[] {
	const groups = new Map<EntryKind, SearchResult[]>()
	for (const result of results) {
		const kind = engine.kindOf(result.entry)
		const list = groups.get(kind) ?? []
		if (list.length < perGroup) {
			list.push(result)
		}
		groups.set(kind, list)
	}
	return [...groups].map(([kind, list]) => ({
		key: kind,
		title: kindTitles[kind],
		results: list,
	}))
}

export function groupResults(
	engine: SearchEngine,
	results: SearchResult[],
	perGroup: number,
	users: TypeUsers[] = [],
): ResultGroup[] {
	const groups = kindGroups(engine, results, perGroup)
	for (const usage of users) {
		const group = {
			key: `uses-${usage.entry}`,
			title: `Used by ${usage.typeName}`,
			results: usage.results,
			typeName: usage.typeName,
		}
		const owner = groups.findIndex(
			(candidate) => candidate.key === engine.kindOf(usage.entry),
		)
		const after = groups.findLastIndex((candidate) =>
			candidate.key.startsWith("uses-"),
		)
		groups.splice(Math.max(owner, after) + 1, 0, group)
	}
	return withStarts(groups)
}
