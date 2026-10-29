import { createChangeFilter } from "./change-filter.ts"
import { trackFilterHeight } from "./sticky-offset.ts"
import { readParam, writeParams } from "../url-state.ts"
import { NameMatcher } from "./name-match.ts"
import { summaryText } from "./summary-text.ts"
import { createTreeFilter, type TreeFilter } from "./tree-filter.ts"

function createRowFilter(list: HTMLElement): TreeFilter {
	const rows = [...list.querySelectorAll<HTMLElement>("[data-name]")]
	const sections = [...list.querySelectorAll<HTMLElement>("[data-list-section]")]
	return {
		total: rows.length,
		names: rows.map((row) => row.dataset["name"] ?? ""),
		apply(match) {
			let matched = 0
			for (const row of rows) {
				row.hidden = !match(row.dataset["name"] ?? "")
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

function createFilter(list: HTMLElement): TreeFilter {
	if (list.hasAttribute("data-tree")) {
		return createTreeFilter(list)
	}
	return list.hasAttribute("data-changes")
		? createChangeFilter(list)
		: createRowFilter(list)
}

export function initListFilter(form: HTMLFormElement): void {
	const list = document.getElementById(form.dataset["listFilter"] ?? "")
	const input = form.elements.namedItem("text")
	const param = form.dataset["param"] ?? "q"
	const summary = form.querySelector("[data-filter-summary]")
	if (list === null || !(input instanceof HTMLInputElement)) {
		return
	}
	const filter = createFilter(list)
	const matcher = new NameMatcher(filter.names)
	let latest = 0

	const refresh = async () => {
		const value = input.value.trim()
		const run = ++latest
		writeParams({ [param]: value === "" ? undefined : value })
		const text = value.toLowerCase()
		const match = await matcher.match(text)
		if (run !== latest) {
			return
		}
		const matched = filter.apply(match, text)
		if (summary !== null) {
			const noun = form.dataset["noun"]
			const counts = summaryText(matched, filter.total)
			summary.textContent =
				noun === undefined ? counts : `${counts.replace(/ total$/, "")} ${noun}`
		}
	}

	if (form.closest("#sidebar") === null) {
		trackFilterHeight(form)
	}
	input.value = readParam(param) ?? ""
	form.addEventListener("submit", (event) => event.preventDefault())
	input.addEventListener("focus", () => matcher.preload(), { once: true })
	input.addEventListener("input", () => void refresh())
	void refresh()
}
