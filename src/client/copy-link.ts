import { announce } from "./announce.ts"
import { refreshTooltip, showTooltip } from "./tooltip/tooltip.ts"
import { flushParams } from "./url-state.ts"

const copiedDuration = 1500
const copiedLabel = "Link copied"

async function copy(link: HTMLAnchorElement): Promise<void> {
	if (link.hasAttribute("data-copied")) {
		return
	}
	flushParams()
	const url = new URL(link.href)
	if (url.pathname === window.location.pathname) {
		url.search = window.location.search
		window.location.hash = url.hash
	}
	try {
		await navigator.clipboard.writeText(url.href)
	} catch {
		announce("The link could not be copied. Copy it from the address bar.")
		return
	}
	const idle = link.getAttribute("data-tooltip") ?? ""
	link.setAttribute("data-copied", "")
	link.setAttribute("data-tooltip", copiedLabel)
	showTooltip(link)
	announce(copiedLabel)
	setTimeout(() => {
		link.removeAttribute("data-copied")
		link.setAttribute("data-tooltip", idle)
		refreshTooltip(link)
	}, copiedDuration)
}

export function initCopyLinks(): void {
	document.addEventListener("click", (event) => {
		const link =
			event.target instanceof Element
				? event.target.closest("[data-copy-link]")
				: null
		if (!(link instanceof HTMLAnchorElement) || !navigator.clipboard) {
			return
		}
		event.preventDefault()
		void copy(link)
	})
}
