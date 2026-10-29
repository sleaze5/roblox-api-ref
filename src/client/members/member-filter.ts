import { NameMatcher, type NameMatch } from "../filters/name-match.ts"
import {
	activeTagFilters,
	applyFilterState,
	readFilterState,
	type HideOption,
	type MemberFilterState,
} from "./filter-state.ts"
import { initGroupCollapse } from "./group-collapse.ts"
import { holdScrollWhileOpen } from "./hold-scroll.ts"
import { trackFilterHeight } from "../filters/sticky-offset.ts"
import { summaryText } from "../filters/summary-text.ts"
import { refitWhileOpen } from "./refit-menu.ts"
import { matchesTags } from "./tag-match.ts"

const toggles: Record<string, HideOption> = {
	"kind-property": "property",
	"kind-method": "method",
	"kind-event": "event",
	"kind-callback": "callback",
	"inherited": "inherited",
}

function isRowShown(
	row: HTMLElement,
	state: MemberFilterState,
	match: NameMatch,
): boolean {
	return match(row.dataset["name"] ?? "") && matchesTags(row, state)
}

function setCount(target: Element | null, count: number): void {
	if (target !== null) {
		target.textContent = String(count)
	}
}

function refreshGroup(group: HTMLElement, state: MemberFilterState): number {
	let count = 0
	for (const subgroup of group.querySelectorAll<HTMLElement>("[data-subgroup]")) {
		subgroup.hidden = subgroup.querySelector("[data-member]:not([hidden])") === null
	}
	for (const row of group.querySelectorAll<HTMLElement>("[data-member]")) {
		if (!row.hidden) {
			count++
		}
	}
	const inheritedHidden =
		state.hide.has("inherited") && group.hasAttribute("data-inherited-group")
	group.hidden = count === 0 || inheritedHidden
	setCount(group.querySelector(".group-head [data-count]"), count)
	return group.hidden ? 0 : count
}

function refreshSection(section: HTMLElement, state: MemberFilterState): number {
	const kind = (section.dataset["section"] ?? "").toLowerCase() as HideOption
	let total = 0
	for (const group of section.querySelectorAll<HTMLElement>("[data-group]")) {
		total += refreshGroup(group, state)
	}
	const kindHidden = state.hide.has(kind)
	section.hidden = kindHidden
	setCount(section.querySelector("h2 [data-count]"), total)
	const empty = section.querySelector<HTMLElement>("[data-empty]")
	if (empty !== null) {
		empty.hidden = total > 0
	}
	return kindHidden ? 0 : total
}

function syncControls(form: HTMLFormElement, state: MemberFilterState): void {
	for (const [name, option] of Object.entries(toggles)) {
		const input = form.elements.namedItem(name)
		if (input instanceof HTMLInputElement) {
			input.checked = !state.hide.has(option)
		}
	}
	for (const input of form.querySelectorAll<HTMLInputElement>(
		"input[name=require]",
	)) {
		input.checked = state.require.has(input.value)
	}
	for (const input of form.querySelectorAll<HTMLInputElement>(
		"input[name=exclude]",
	)) {
		input.checked = state.exclude.has(input.value)
	}
	for (const input of form.querySelectorAll<HTMLInputElement>("input[name=match]")) {
		input.checked = input.value === state.match
	}
	const text = form.elements.namedItem("text")
	if (text instanceof HTMLInputElement) {
		text.value = state.text
	}
}

function updateTag(target: HTMLInputElement, state: MemberFilterState): void {
	const [own, other] =
		target.name === "require"
			? [state.require, state.exclude]
			: [state.exclude, state.require]
	if (target.checked) {
		own.add(target.value)
		other.delete(target.value)
	} else {
		own.delete(target.value)
	}
}

function updateState(target: EventTarget | null, state: MemberFilterState): void {
	if (
		target instanceof HTMLInputElement &&
		(target.name === "require" || target.name === "exclude")
	) {
		updateTag(target, state)
	} else if (target instanceof HTMLInputElement && target.name === "match") {
		state.match = target.value === "all" ? "all" : "any"
	} else if (target instanceof HTMLInputElement && target.type === "checkbox") {
		const option = toggles[target.name]
		if (option === undefined) {
			return
		}
		if (target.checked) {
			state.hide.delete(option)
		} else {
			state.hide.add(option)
		}
	} else if (target instanceof HTMLInputElement && target.name === "text") {
		state.text = target.value.trim()
	}
}

export function initMemberFilter(form: HTMLFormElement): void {
	const article = form.closest("article") ?? document.body
	const rows = [...article.querySelectorAll<HTMLElement>("[data-member]")]
	const sections = [...article.querySelectorAll<HTMLElement>("[data-section]")]
	const summary = form.querySelector("[data-filter-summary]")
	const badge = form.querySelector<HTMLElement>("[data-tag-filter-count]")
	const state = readFilterState()
	const matcher = new NameMatcher(rows.map((row) => row.dataset["name"] ?? ""))
	let latest = 0

	const refresh = async () => {
		applyFilterState(state)
		const run = ++latest
		const match = await matcher.match(state.text.toLowerCase())
		if (run !== latest) {
			return
		}
		for (const row of rows) {
			row.hidden = !isRowShown(row, state, match)
		}
		const shown = sections.reduce(
			(sum, section) => sum + refreshSection(section, state),
			0,
		)
		if (summary !== null) {
			summary.textContent = summaryText(shown, rows.length)
		}
		const active = activeTagFilters(state)
		if (badge !== null) {
			badge.hidden = active === 0
			badge.textContent = String(active)
		}
	}

	form.addEventListener("submit", (event) => event.preventDefault())
	form.addEventListener("input", (event) => {
		updateState(event.target, state)
		if (event.target instanceof HTMLInputElement && event.target.name !== "text") {
			syncControls(form, state)
		}
		void refresh()
	})
	form.querySelector("[data-tag-filter-reset]")?.addEventListener("click", () => {
		state.require.clear()
		state.exclude.clear()
		state.match = "any"
		state.hide.delete("inherited")
		syncControls(form, state)
		void refresh()
	})
	const menu = form.querySelector<HTMLElement>("[popover]")
	if (menu !== null) {
		holdScrollWhileOpen(menu, article)
		refitWhileOpen(menu)
	}
	trackFilterHeight(form)
	initGroupCollapse(article)
	syncControls(form, state)
	form.querySelector("input[name=text]")?.addEventListener(
		"focus",
		() => matcher.preload(),
		{ once: true },
	)
	void refresh()
}
