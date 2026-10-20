export type TooltipTarget = {
	element: HTMLElement
	text: string
}

export function hintTarget(element: Element): TooltipTarget | undefined {
	const hinted = element.closest<HTMLElement>("[data-tooltip]")
	const text = hinted?.getAttribute("data-tooltip") ?? ""
	return hinted === null || text === "" ? undefined : { element: hinted, text }
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
