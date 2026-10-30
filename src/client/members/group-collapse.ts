import { hashId } from "./hash-id.ts"

function toggleButtons(article: Element): HTMLButtonElement[] {
	return [...article.querySelectorAll<HTMLButtonElement>("[data-collapse-toggle]")]
}

function isCollapsed(button: HTMLButtonElement): boolean {
	return button.getAttribute("aria-expanded") === "false"
}

function setCollapsed(button: HTMLButtonElement, collapsed: boolean): void {
	button.setAttribute("aria-expanded", String(!collapsed))
	button.closest("[data-group]")?.toggleAttribute("data-collapsed", collapsed)
}

function syncCollapseAll(article: Element): void {
	const all = article.querySelector<HTMLButtonElement>("[data-collapse-all]")
	if (all === null) {
		return
	}
	const everyCollapsed = toggleButtons(article).every(isCollapsed)
	all.textContent = everyCollapsed ? "Expand all" : "Collapse all"
	all.dataset["collapseAll"] = everyCollapsed ? "expand" : "collapse"
}

function revealTarget(article: Element): void {
	const id = hashId(window.location.hash)
	const target = id === "" ? null : document.getElementById(id)
	const group = target?.closest("[data-collapsed]")
	const button = group?.querySelector<HTMLButtonElement>("[data-collapse-toggle]")
	if (target && button) {
		setCollapsed(button, false)
		syncCollapseAll(article)
		target.scrollIntoView()
	}
}

export function initGroupCollapse(article: Element): void {
	article.addEventListener("click", (event) => {
		const target = event.target instanceof Element ? event.target : null
		const toggle = target?.closest<HTMLButtonElement>("[data-collapse-toggle]")
		const all = target?.closest<HTMLElement>("[data-collapse-all]")
		if (toggle) {
			setCollapsed(toggle, !isCollapsed(toggle))
		} else if (all) {
			const collapse = all.dataset["collapseAll"] !== "expand"
			for (const button of toggleButtons(article)) {
				setCollapsed(button, collapse)
			}
		} else {
			return
		}
		syncCollapseAll(article)
	})
	window.addEventListener("hashchange", () => revealTarget(article))
	syncCollapseAll(article)
	revealTarget(article)
}
