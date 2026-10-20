import { placeTooltip } from "./place-tooltip.ts"
import { hintTarget, tooltipTarget, type TooltipTarget } from "./tooltip-target.ts"

const showDelay = 500
const warmWindow = 300

let tip: HTMLElement | null = null
let anchor: HTMLElement | undefined
let timer: ReturnType<typeof setTimeout> | undefined
let lastHidden = 0

function show(target: TooltipTarget | undefined): void {
	if (tip === null || target === undefined) {
		return
	}
	anchor = target.element
	tip.textContent = target.text
	if (!tip.matches(":popover-open")) {
		tip.showPopover()
	}
	placeTooltip(tip, target.element)
}

function hide(): void {
	clearTimeout(timer)
	if (tip === null || anchor === undefined) {
		return
	}
	anchor = undefined
	lastHidden = performance.now()
	if (tip.matches(":popover-open")) {
		tip.hidePopover()
	}
}

export function showTooltip(target: HTMLElement): void {
	clearTimeout(timer)
	show(hintTarget(target))
}

export function refreshTooltip(target: HTMLElement): void {
	if (target === anchor) {
		show(hintTarget(target))
	}
}

export function initTooltips(): void {
	tip = document.getElementById("tooltip")
	if (tip === null) {
		return
	}
	document.addEventListener("pointerover", (event) => {
		const target = tooltipTarget(event.target)
		if (
			event.pointerType !== "mouse" ||
			target === undefined ||
			target.element === anchor
		) {
			return
		}
		clearTimeout(timer)
		if (performance.now() - lastHidden < warmWindow) {
			show(target)
		} else {
			timer = setTimeout(() => show(target), showDelay)
		}
	})
	document.addEventListener("pointerout", (event) => {
		const target = tooltipTarget(event.target)
		if (
			target !== undefined &&
			!(
				event.relatedTarget instanceof Node &&
				target.element.contains(event.relatedTarget)
			)
		) {
			hide()
		}
	})
	document.addEventListener("focusin", (event) => {
		const target = tooltipTarget(event.target)
		if (target !== undefined && target.element.matches(":focus-visible")) {
			show(target)
		}
	})
	document.addEventListener("focusout", hide)
	document.addEventListener("pointerdown", hide)
	document.addEventListener("keydown", (event) => {
		if (event.key === "Escape") {
			hide()
		}
	})
	document.addEventListener("scroll", hide, { capture: true, passive: true })
}
