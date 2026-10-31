export type TooltipIcon = {
	kind: string
	index: string | null
}

export type TooltipTarget = {
	element: HTMLElement
	text: string
	icon?: TooltipIcon
}

function hintIcon(element: HTMLElement): TooltipIcon | undefined {
	const kind = element.getAttribute("data-tooltip-kind")
	return kind === null
		? undefined
		: { kind, index: element.getAttribute("data-tooltip-icon") }
}

export function hintTarget(element: Element): TooltipTarget | undefined {
	const hinted = element.closest<HTMLElement>("[data-tooltip]")
	const text = hinted?.getAttribute("data-tooltip") ?? ""
	if (hinted === null || text === "") {
		return undefined
	}
	const icon = hintIcon(hinted)
	return icon === undefined
		? { element: hinted, text }
		: { element: hinted, text, icon }
}

function isClipped(element: HTMLElement): boolean {
	return (
		element.scrollWidth > element.clientWidth &&
		getComputedStyle(element).textOverflow === "ellipsis"
	)
}

function clippedTarget(element: Element): TooltipTarget | undefined {
	const link = element.closest<HTMLElement>("a")
	if (link === null) {
		return undefined
	}
	const clipped = [link, ...link.querySelectorAll<HTMLElement>("span")].find(
		isClipped,
	)
	const text = clipped?.textContent.trim() ?? ""
	return text === "" ? undefined : { element: link, text }
}

export function tooltipTarget(target: EventTarget | null): TooltipTarget | undefined {
	if (!(target instanceof Element)) {
		return undefined
	}
	return hintTarget(target) ?? clippedTarget(target)
}

export function tapTarget(target: EventTarget | null): TooltipTarget | undefined {
	if (!(target instanceof Element)) {
		return undefined
	}
	const hinted = hintTarget(target)
	return hinted?.element.closest("a, button") === null ? hinted : undefined
}
