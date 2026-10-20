type SpyTarget = {
	element: HTMLElement
	section: HTMLElement
	entry?: HTMLElement
}

function collectTargets(rail: HTMLElement): SpyTarget[] {
	const targets: SpyTarget[] = []
	for (const section of rail.querySelectorAll<HTMLElement>("[data-rail-section]")) {
		const sectionElement = document.getElementById(
			section.dataset["railSection"] ?? "",
		)
		if (sectionElement !== null) {
			targets.push({ element: sectionElement, section })
		}
		for (const entry of section.querySelectorAll<HTMLElement>(
			"[data-rail-entry]",
		)) {
			const member = document.getElementById(entry.dataset["railEntry"] ?? "")
			if (member !== null) {
				targets.push({ element: member, section, entry })
			}
		}
	}
	return targets
}

function topmost(targets: SpyTarget[]): SpyTarget | undefined {
	let best: SpyTarget | undefined
	let bestTop = Number.POSITIVE_INFINITY
	for (const target of targets) {
		const top = target.element.getBoundingClientRect().top
		if (top < bestTop) {
			best = target
			bestTop = top
		}
	}
	return best
}

function keepVisible(entry: HTMLElement): void {
	const scroller = entry.closest("aside")
	if (scroller === null) {
		return
	}
	const box = scroller.getBoundingClientRect()
	const rect = entry.getBoundingClientRect()
	if (rect.top < box.top) {
		scroller.scrollTop -= box.top - rect.top
	} else if (rect.bottom > box.bottom) {
		scroller.scrollTop += rect.bottom - box.bottom
	}
}

function stickyOffset(): number {
	return Number.parseFloat(
		getComputedStyle(document.documentElement).scrollPaddingTop,
	)
}

export function initScrollSpy(rail: HTMLElement): void {
	const targets = collectTargets(rail)
	const byElement = new Map(targets.map((target) => [target.element, target]))
	const visible = new Set<SpyTarget>()
	let active: SpyTarget | undefined

	const activate = (next: SpyTarget | undefined) => {
		if (active !== undefined) {
			active.section.removeAttribute("data-active")
			active.entry?.removeAttribute("data-active")
			active.entry?.removeAttribute("aria-current")
		}
		active = next
		if (next === undefined) {
			return
		}
		next.section.setAttribute("data-active", "")
		if (next.entry !== undefined) {
			next.entry.setAttribute("data-active", "")
			next.entry.setAttribute("aria-current", "location")
			keepVisible(next.entry)
		}
	}

	const observer = new IntersectionObserver(
		(records) => {
			for (const record of records) {
				const target = byElement.get(record.target as HTMLElement)
				if (target === undefined) {
					continue
				}
				if (record.isIntersecting) {
					visible.add(target)
				} else {
					visible.delete(target)
				}
			}
			const shown = [...visible]
			const next =
				topmost(shown.filter((target) => target.entry !== undefined)) ??
				topmost(shown)
			if (next !== active) {
				activate(next)
			}
		},
		{ rootMargin: `-${Math.round(stickyOffset())}px 0px -50% 0px` },
	)
	for (const target of targets) {
		observer.observe(target.element)
	}
}
