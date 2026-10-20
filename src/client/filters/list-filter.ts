import { trackFilterHeight } from "./sticky-offset.ts"
import { readParam, writeParams } from "../url-state.ts"
import { matchesName } from "./name-match.ts"
import { summaryText } from "./summary-text.ts"
import { createTreeFilter, type TreeFilter } from "./tree-filter.ts"

function createRowFilter(list: HTMLElement): TreeFilter {
	const rows = [...list.querySelectorAll<HTMLElement>("[data-name]")]
	const sections = [...list.querySelectorAll<HTMLElement>("[data-list-section]")]
	return {
		total: rows.length,
		apply(text) {
			let matched = 0
			for (const row of rows) {
				row.hidden = !matchesName(text, row.dataset["name"] ?? "")
				matched += row.hidden ? 0 : 1
			}
			for (const section of sections) {
				section.hidden =
					section.querySelector("[data-name]:not([hidden])") === null
			}
			return matched
		},
	}
}

export function initListFilter(form: HTMLFormElement): void {
	const list = document.getElementById(form.dataset["listFilter"] ?? "")
	const input = form.elements.namedItem("text")
	const param = form.dataset["param"] ?? "q"
	const summary = form.querySelector("[data-filter-summary]")
	if (list === null || !(input instanceof HTMLInputElement)) {
		return
	}
	const filter = list.hasAttribute("data-tree")
		? createTreeFilter(list)
		: createRowFilter(list)

	const refresh = () => {
		const value = input.value.trim()
		const matched = filter.apply(value.toLowerCase())
		writeParams({ [param]: value === "" ? undefined : value })
		if (summary !== null) {
			summary.textContent = summaryText(matched, filter.total)
		}
	}

	if (form.closest("#sidebar") === null) {
		trackFilterHeight(form)
	}
	input.value = readParam(param) ?? ""
	form.addEventListener("submit", (event) => event.preventDefault())
	input.addEventListener("input", refresh)
	refresh()
}
