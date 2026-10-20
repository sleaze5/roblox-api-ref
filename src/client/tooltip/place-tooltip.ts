const gap = 8
const margin = 8

export function placeTooltip(tip: HTMLElement, target: Element): void {
	const box = target.getBoundingClientRect()
	const width = tip.offsetWidth
	const height = tip.offsetHeight
	const above = box.top - gap - height
	const top = above >= margin ? above : box.bottom + gap
	const centered = box.left + box.width / 2 - width / 2
	const left = Math.min(
		Math.max(centered, margin),
		window.innerWidth - width - margin,
	)
	tip.style.top = `${Math.round(top)}px`
	tip.style.left = `${Math.round(left)}px`
}
