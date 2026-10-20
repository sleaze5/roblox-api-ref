import { entryFlags, type EntryKind } from "./index-format.ts"

export const accessFilters = { game: 0, plugin: 1, command: 2 } as const

export type AccessFilter = keyof typeof accessFilters

export type ParsedQuery = {
	words: string[]
	scope?: string
	exact: boolean
	kinds: Set<EntryKind>
	include: number
	exclude: number
	valueType?: string
	uses?: string
	enumScope?: boolean
	access?: AccessFilter
}

export const kindAliases: Record<string, EntryKind[]> = {
	class: ["Class"],
	enum: ["Enum"],
	type: ["DataType"],
	property: ["Property"],
	method: ["Method"],
	function: ["Method"],
	event: ["Event"],
	callback: ["Callback"],
	item: ["EnumItem"],
	member: ["Property", "Method", "Event", "Callback"],
}

export const tagAliases: Record<string, number> = {
	deprecated: entryFlags.deprecated,
	hidden: entryFlags.hidden,
	internal: entryFlags.internal,
	service: entryFlags.service,
	readonly: entryFlags.readOnly,
	yields: entryFlags.yields,
	notcreatable: entryFlags.notCreatable,
	notreplicated: entryFlags.notReplicated,
	notscriptable: entryFlags.notScriptable,
}

const filterPattern = /^([!-]?)(is|tag|type|uses|access):(.*)$/i
const enumPrefix = "enum."

export function emptyQuery(): ParsedQuery {
	return { words: [], exact: false, kinds: new Set(), include: 0, exclude: 0 }
}

function applyFilter(
	query: ParsedQuery,
	negated: boolean,
	field: string,
	value: string,
): boolean {
	const lower = value.toLowerCase()
	if (field === "type" || field === "uses") {
		if (value === "") {
			return false
		}
		if (field === "type") {
			query.valueType = value
		} else {
			query.uses = value
		}
		return true
	}
	if (field === "access") {
		if (!Object.hasOwn(accessFilters, lower)) {
			return false
		}
		query.access = lower as AccessFilter
		return true
	}
	if (field === "is") {
		const kinds = kindAliases[lower]
		for (const kind of kinds ?? []) {
			query.kinds.add(kind)
		}
		return kinds !== undefined
	}
	const flag = tagAliases[lower]
	if (flag === undefined) {
		return false
	}
	if (negated) {
		query.exclude |= flag
	} else {
		query.include |= flag
	}
	return true
}

function addEnumWord(query: ParsedQuery, word: string): void {
	const dot = word.indexOf(".")
	if (dot === -1) {
		query.kinds.add("Enum")
		if (word.length > 0) {
			query.words.push(word)
		}
		return
	}
	query.scope = word.slice(0, dot)
	query.enumScope = true
	const item = word.slice(dot + 1)
	if (item.length > 0) {
		query.words.push(item)
	}
}

function addWord(query: ParsedQuery, part: string): void {
	let word = part
	if (word.length >= 2 && word.startsWith('"') && word.endsWith('"')) {
		query.exact = true
		word = word.slice(1, -1)
	}
	if (query.scope === undefined && word.toLowerCase().startsWith(enumPrefix)) {
		addEnumWord(query, word.slice(enumPrefix.length))
		return
	}
	const dot = word.search(/[.:]/)
	if (dot !== -1 && query.scope === undefined) {
		query.scope = word.slice(0, dot)
		word = word.slice(dot + 1)
	}
	if (word.length > 0) {
		query.words.push(word)
	}
}

export function parseQuery(input: string): ParsedQuery {
	const query = emptyQuery()
	for (const part of input.trim().split(/\s+/)) {
		if (part.length === 0) {
			continue
		}
		const filter = filterPattern.exec(part)
		if (filter !== null) {
			const [, negation = "", field = "", value = ""] = filter
			if (applyFilter(query, negation !== "", field.toLowerCase(), value)) {
				continue
			}
		}
		addWord(query, part)
	}
	return query
}

export function hasFilters(query: ParsedQuery): boolean {
	return (
		query.kinds.size > 0 ||
		query.include !== 0 ||
		query.valueType !== undefined ||
		query.uses !== undefined ||
		query.access !== undefined
	)
}
