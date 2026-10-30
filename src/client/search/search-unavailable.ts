import { announce } from "../announce.ts"
import { refreshTooltip, showTooltip } from "../tooltip/tooltip.ts"

const message = "Search could not load. Check your connection, then reload the page."
const messageDuration = 5000

export function reportSearchUnavailable(trigger: HTMLElement | null): void {
	announce(message)
	if (trigger === null || trigger.hasAttribute("data-search-failed")) {
		return
	}
	const idle = trigger.getAttribute("data-tooltip")
	trigger.setAttribute("data-search-failed", "")
	trigger.setAttribute("data-tooltip", message)
	showTooltip(trigger)
	setTimeout(() => {
		trigger.removeAttribute("data-search-failed")
		if (idle === null) {
			trigger.removeAttribute("data-tooltip")
		} else {
			trigger.setAttribute("data-tooltip", idle)
		}
		refreshTooltip(trigger)
	}, messageDuration)
}
