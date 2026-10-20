import { readParam } from "../url-state.ts"

const loadSearch = () => import("./mount-search.tsx")

function isTyping(target: EventTarget | null): boolean {
	return (
		target instanceof HTMLElement &&
		(target.isContentEditable ||
			["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName))
	)
}

async function show(query?: string): Promise<void> {
	const { showSearch } = await loadSearch()
	showSearch(query)
}

async function warm(): Promise<void> {
	const { warmSearch } = await loadSearch()
	warmSearch()
}

export function initSearchTrigger(): void {
	document.addEventListener("keydown", (event) => {
		const shortcut =
			(event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k"
		if (shortcut || (event.key === "/" && !isTyping(event.target))) {
			event.preventDefault()
			void show()
		}
	})

	document.addEventListener("click", (event) => {
		const trigger =
			event.target instanceof Element
				? event.target.closest("[data-search-open]")
				: null
		if (trigger !== null) {
			const preset = trigger.getAttribute("data-search-open") ?? ""
			void show(preset === "" ? undefined : preset)
		}
	})

	const initial = readParam("search")
	if (initial !== undefined) {
		void show(initial)
	} else if ("requestIdleCallback" in window) {
		requestIdleCallback(() => void warm(), { timeout: 3000 })
	} else {
		setTimeout(() => void warm(), 1500)
	}
}
