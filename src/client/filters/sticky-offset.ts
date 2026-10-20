export function trackFilterHeight(form: HTMLFormElement): void {
	const root = document.documentElement
	const update = () => {
		const sticky = getComputedStyle(form).position === "sticky"
		const height = sticky ? form.getBoundingClientRect().height : 0
		root.style.setProperty("--filter-height", `${height}px`)
	}
	new ResizeObserver(update).observe(form)
	update()
}
