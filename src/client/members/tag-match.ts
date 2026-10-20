import type { MemberFilterState } from "./filter-state.ts"

export function matchesTags(row: HTMLElement, state: MemberFilterState): boolean {
	const tags = new Set((row.dataset["tags"] ?? "").split(" "))
	if ([...state.exclude].some((key) => tags.has(key))) {
		return false
	}
	if (state.require.size === 0) {
		return true
	}
	const required = [...state.require]
	return state.match === "all"
		? required.every((key) => tags.has(key))
		: required.some((key) => tags.has(key))
}
