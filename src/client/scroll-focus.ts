const selector = "pre, .stat-table"

function update(area: HTMLElement): void {
	if (area.scrollWidth > area.clientWidth) {
		area.tabIndex = 0
	} else {
		area.removeAttribute("tabindex")
	}
}

export function initScrollFocus(): void {
	const areas = document.querySelectorAll<HTMLElement>(selector)
	const observer = new ResizeObserver((entries) => {
		for (const entry of entries) {
			const area = entry.target.closest<HTMLElement>(selector)
			if (area !== null) {
				update(area)
			}
		}
	})
	for (const area of areas) {
		observer.observe(area)
		if (area.firstElementChild !== null) {
			observer.observe(area.firstElementChild)
		}
		update(area)
	}
}
