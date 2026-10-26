import { announce } from "../announce.ts"
import { readParam } from "../url-state.ts"

const loadSearch = () => import("./mount-search.tsx")

const unavailable =
	"Search could not load. Check your connection, then reload the page."

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

async function show(query?: string, reloadOnFailure = true): Promise<void> {
	const search = await loadSearch().catch(() => undefined)
	if (search !== undefined) {
		search.showSearch(query)
	} else if (reloadOnFailure && navigator.onLine) {
		reloadWithSearch(query)
	} else {
		announce(unavailable)
	}
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
		void show(initial, false)
	}
}
