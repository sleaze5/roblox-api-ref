import { readParam } from "../url-state.ts"
import { reportSearchUnavailable } from "./search-unavailable.ts"
import { showOrRecover } from "./show-or-recover.ts"

const loadSearch = () => import("./search-dialog.ts")

function isTyping(target: EventTarget | null): boolean {
	return (
		target instanceof HTMLElement &&
		(target.isContentEditable ||
			["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName))
	)
}

function reloadWithSearch(query: string | undefined): void {
	const url = new URL(window.location.href)
	url.searchParams.set("search", query ?? "")
	window.location.assign(url)
}

function headerTrigger(): HTMLElement | null {
	return document.querySelector<HTMLElement>("header [data-search-open]")
}

function show(
	query: string | undefined,
	trigger: HTMLElement | null,
	reloadOnFailure: boolean,
): void {
	void showOrRecover(query, {
		load: loadSearch,
		canReload: reloadOnFailure && navigator.onLine,
		reload: reloadWithSearch,
		report: () => reportSearchUnavailable(trigger ?? headerTrigger()),
	})
}

function setInstant(instant: boolean): void {
	document
		.querySelector("[data-search-root]")
		?.toggleAttribute("data-instant", instant)
}

export function initSearchTrigger(): void {
	document.addEventListener("pointerdown", () => setInstant(false), { capture: true })
	document.addEventListener("keydown", (event) => {
		if (event.key === "Escape") {
			setInstant(true)
		}
		const shortcut =
			(event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k"
		const open = document.querySelector<HTMLDialogElement>(
			"[data-search-root] dialog[open]",
		)
		if (shortcut && open !== null) {
			event.preventDefault()
			setInstant(true)
			open.close()
			return
		}
		if (shortcut || (event.key === "/" && !isTyping(event.target))) {
			event.preventDefault()
			setInstant(true)
			show(undefined, null, true)
		}
	})

	document.addEventListener("click", (event) => {
		const trigger =
			event.target instanceof Element
				? event.target.closest<HTMLElement>("[data-search-open]")
				: null
		if (trigger !== null) {
			const preset = trigger.getAttribute("data-search-open") ?? ""
			show(preset === "" ? undefined : preset, trigger, true)
		}
	})

	const initial = readParam("search")
	if (initial !== undefined) {
		show(initial, null, false)
	}
}
