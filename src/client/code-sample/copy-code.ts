import { announce } from "../announce.ts"
import { hideTooltip } from "../tooltip/tooltip.ts"

const copiedDuration = 1000
const copiedLabel = "Copied!"

export async function copyCode(
	button: HTMLButtonElement,
	sample: HTMLElement,
): Promise<void> {
	if (button.hasAttribute("data-copied")) {
		return
	}
	const lines = Array.from(
		sample.querySelectorAll("[data-code-line]"),
		(line) => line.textContent,
	)
	try {
		await navigator.clipboard.writeText(lines.join("\n"))
	} catch {
		announce("The code could not be copied. Select it and copy it instead.")
		return
	}
	const tooltip = button.getAttribute("data-tooltip") ?? ""
	hideTooltip(button)
	button.removeAttribute("data-tooltip")
	button.setAttribute("data-copied", "")
	button.textContent = copiedLabel
	announce("Code copied")
	setTimeout(() => {
		button.removeAttribute("data-copied")
		button.textContent = ""
		button.setAttribute("data-tooltip", tooltip)
	}, copiedDuration)
}
