import { compareNames } from "../api/compare-names.ts"
import type { SearchEngine, SearchResult } from "./engine.ts"
import { accessFilters, kindAliases, tagAliases } from "./query.ts"

function filterOptions(engine: SearchEngine): Record<string, string[]> {
	return {
		"is:": Object.keys(kindAliases),
		"tag:": Object.keys(tagAliases),
		"access:": Object.keys(accessFilters),
		"type:": [...engine.index.typeNames].sort((a, b) => compareNames(a, b)),
		"uses:": [...engine.index.typeNames].sort((a, b) => compareNames(a, b)),
	}
}

function completeFilter(word: string, engine: SearchEngine): string | undefined {
	const negation = word.startsWith("!") || word.startsWith("-") ? word.charAt(0) : ""
	const body = word.slice(negation.length).toLowerCase()
	for (const [prefix, options] of Object.entries(filterOptions(engine))) {
		if (body.startsWith(prefix)) {
			const value = body.slice(prefix.length)
			const option = options.find((candidate) => {
				const lower = candidate.toLowerCase()
				return lower.startsWith(value) && lower !== value
			})
			return option === undefined ? undefined : `${negation}${prefix}${option}`
		}
		if (prefix.startsWith(body) && body.length > 0) {
			return `${negation}${prefix}`
		}
	}
	return undefined
}

function completeName(
	word: string,
	engine: SearchEngine,
	top: SearchResult | undefined,
): string | undefined {
	if (top === undefined) {
		return undefined
	}
	const dot = word.search(/[.:]/)
	const name = engine.nameOf(top.entry)
	if (dot === -1) {
		if (
			!engine.isPrimary(top.entry) ||
			!name.toLowerCase().startsWith(word.toLowerCase())
		) {
			return undefined
		}
		return `${name}.`
	}
	const scope = word.slice(0, dot)
	const separator = word.charAt(dot)
	const member = word.slice(dot + 1)
	if (
		engine.isPrimary(top.entry) ||
		!name.toLowerCase().startsWith(member.toLowerCase())
	) {
		return undefined
	}
	const scopeName = top.scope === undefined ? scope : engine.nameOf(top.scope)
	return `${scopeName}${separator}${name}`
}

export function complete(
	input: string,
	engine: SearchEngine,
	top: SearchResult | undefined,
): string | undefined {
	const start = input.lastIndexOf(" ") + 1
	const word = input.slice(start)
	if (word.length === 0) {
		return undefined
	}
	const completed = completeFilter(word, engine) ?? completeName(word, engine, top)
	if (completed === undefined || completed === word) {
		return undefined
	}
	return input.slice(0, start) + completed
}
