import { copyCode } from "./copy-code.ts"
import { toggleWrap } from "./wrap-code.ts"

export function initCodeSamples(): void {
	for (const actions of document.querySelectorAll<HTMLElement>(
		"[data-code-actions]",
	)) {
		actions.hidden = false
	}
	for (const button of document.querySelectorAll<HTMLButtonElement>(
		"[data-copy-code]",
	)) {
		button.hidden = !navigator.clipboard
	}
	document.addEventListener("click", (event) => {
		const button =
			event.target instanceof Element
				? event.target.closest("[data-copy-code], [data-wrap-code]")
				: null
		const sample = button?.closest<HTMLElement>("[data-code-sample]")
		if (!(button instanceof HTMLButtonElement) || !sample) {
			return
		}
		if (button.hasAttribute("data-wrap-code")) {
			toggleWrap(button, sample)
		} else {
			void copyCode(button, sample)
		}
	})
}
