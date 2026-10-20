export function holdScrollWhileOpen(popover: HTMLElement, content: HTMLElement): void {
	popover.addEventListener("toggle", (event) => {
		const open = event.newState === "open"
		content.style.minHeight = open
			? `${content.getBoundingClientRect().height}px`
			: ""
		content.style.overflowAnchor = open ? "none" : ""
	})
}
