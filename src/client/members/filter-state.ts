import { readParam, writeParams } from "../url-state.ts"

export const hideOptions = [
	"property",
	"method",
	"event",
	"callback",
	"inherited",
] as const

export type HideOption = (typeof hideOptions)[number]

export type MatchMode = "any" | "all"

export type MemberFilterState = {
	text: string
	hide: Set<HideOption>
	require: Set<string>
	exclude: Set<string>
	match: MatchMode
}

const tagKeyPattern = /^[a-z0-9-]+$/

function parseList(value: string | undefined): string[] {
	return (value ?? "").split(",").filter((item) => item !== "")
}

function parseHide(value: string | undefined): Set<HideOption> {
	const items = parseList(value)
	return new Set(hideOptions.filter((option) => items.includes(option)))
}

function parseTags(value: string | undefined): Set<string> {
	return new Set(parseList(value).filter((key) => tagKeyPattern.test(key)))
}

function listText(values: Iterable<string>): string | undefined {
	const text = [...values].join(",")
	return text === "" ? undefined : text
}

export function readFilterState(): MemberFilterState {
	return {
		text: readParam("q") ?? "",
		hide: parseHide(readParam("hide")),
		require: parseTags(readParam("tags")),
		exclude: parseTags(readParam("not")),
		match: readParam("match") === "all" ? "all" : "any",
	}
}

export function activeTagFilters(state: MemberFilterState): number {
	return (
		state.require.size + state.exclude.size + (state.hide.has("inherited") ? 1 : 0)
	)
}

export function applyFilterState(state: MemberFilterState): void {
	const root = document.documentElement
	const hide = hideOptions.filter((option) => state.hide.has(option))
	root.dataset["hide"] = hide.join(" ")
	document.getElementById("tag-filter")?.remove()
	writeParams({
		q: state.text === "" ? undefined : state.text,
		hide: listText(hide),
		tags: listText(state.require),
		not: listText(state.exclude),
		match: state.match === "all" ? "all" : undefined,
	})
}
