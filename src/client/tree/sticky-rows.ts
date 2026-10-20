function stuckSummary(target: EventTarget | null): HTMLElement | undefined {
	const summary =
		target instanceof Element
			? target.closest<HTMLElement>("[data-sticky-tree] details > summary")
			: null
	const details = summary?.parentElement
	if (!summary || !details) {
		return undefined
	}
	const offset =
		summary.getBoundingClientRect().top - details.getBoundingClientRect().top
	return offset > 1 ? summary : undefined
}

function scrollToNode(summary: HTMLElement): void {
	const details = summary.parentElement
	const scroller = summary.closest<HTMLElement>("#sidebar")
	if (details && scroller) {
		scroller.scrollTop -=
			summary.getBoundingClientRect().top - details.getBoundingClientRect().top
	}
}

export function initStickyRows(): void {
	document.addEventListener("click", (event) => {
		const summary = stuckSummary(event.target)
		if (summary !== undefined) {
			event.preventDefault()
			scrollToNode(summary)
		}
	})
}
