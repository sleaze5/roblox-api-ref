function scrollToCurrent(drawer: HTMLElement): void {
	const current = drawer.querySelector("[data-tree] [aria-current=page]")
	if (current !== null) {
		drawer.scrollTop +=
			current.getBoundingClientRect().top -
			drawer.getBoundingClientRect().top -
			drawer.clientHeight / 3
	}
}

export function initDrawer(): void {
	const drawer = document.getElementById("sidebar")
	drawer?.addEventListener("toggle", (event) => {
		if (event.newState === "open") {
			scrollToCurrent(drawer)
		}
	})
}
