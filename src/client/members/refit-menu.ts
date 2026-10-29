export function refitWhileOpen(popover: HTMLElement): void {
	let frame = 0
	const refit = () => {
		frame = 0
		popover.style.maxHeight = "none"
		void popover.offsetHeight
		popover.style.removeProperty("max-height")
	}
	const schedule = () => {
		if (frame === 0) {
			frame = requestAnimationFrame(refit)
		}
	}
	popover.addEventListener("toggle", (event) => {
		if (event.newState === "open") {
			window.addEventListener("scroll", schedule, { passive: true })
		} else {
			window.removeEventListener("scroll", schedule)
			cancelAnimationFrame(frame)
			frame = 0
		}
	})
}
