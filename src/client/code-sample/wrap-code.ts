import { refreshTooltip } from "../tooltip/tooltip.ts"

export function toggleWrap(button: HTMLButtonElement, sample: HTMLElement): void {
	const wrapped = !sample.hasAttribute("data-wrapped")
	sample.toggleAttribute("data-wrapped", wrapped)
	const label = wrapped ? "Unwrap lines" : "Wrap lines"
	button.setAttribute("aria-pressed", String(wrapped))
	button.setAttribute("aria-label", label)
	button.setAttribute("data-tooltip", label)
	refreshTooltip(button)
}
