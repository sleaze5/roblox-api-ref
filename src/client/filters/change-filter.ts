import type { NameMatch } from "./name-match.ts"
import type { TreeFilter } from "./tree-filter.ts"

type ChangeGroup = {
	element: HTMLElement
	name: string
	counted: number
	rows: HTMLElement[]
}

function changeGroups(list: HTMLElement): ChangeGroup[] {
	return [...list.querySelectorAll<HTMLElement>("[data-change-group]")].map(
		(element) => ({
			element,
			name: element.dataset["name"] ?? "",
			counted: element.querySelector(":scope > [data-change]") === null ? 0 : 1,
			rows: [...element.querySelectorAll<HTMLElement>("li[data-change]")],
		}),
	)
}

function filterGroup(group: ChangeGroup, match: NameMatch): number {
	const whole = match(group.name)
	let shown = 0
	for (const row of group.rows) {
		row.hidden = !whole && !match(row.dataset["name"] ?? "")
		shown += row.hidden ? 0 : 1
	}
	group.element.hidden = !whole && shown === 0
	return group.element.hidden ? 0 : shown + group.counted
}

export function createChangeFilter(list: HTMLElement): TreeFilter {
	const groups = changeGroups(list)
	const sections = [...list.querySelectorAll<HTMLElement>("[data-list-section]")]
	return {
		total: list.querySelectorAll("[data-change]").length,
		names: groups.flatMap((group) => [
			group.name,
			...group.rows.map((row) => row.dataset["name"] ?? ""),
		]),
		apply(match, text) {
			let matched = 0
			for (const group of groups) {
				matched += filterGroup(group, match)
			}
			for (const section of sections) {
				section.hidden =
					text !== "" &&
					section.querySelector("[data-change-group]:not([hidden])") === null
			}
			return matched
		},
	}
}
