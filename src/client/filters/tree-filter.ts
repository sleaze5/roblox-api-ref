import { matchesName } from "./name-match.ts"

function nodeName(node: HTMLElement): string {
	const link = node.querySelector(":scope > a, :scope > details > summary > a")
	return link?.textContent ?? ""
}

function childNodes(node: HTMLElement): HTMLElement[] {
	return [...node.querySelectorAll<HTMLElement>(":scope > details > ul > li")]
}

function filterNode(node: HTMLElement, text: string): number {
	let matches = 0
	for (const child of childNodes(node)) {
		matches += filterNode(child, text)
	}
	const selfMatch = matchesName(text, nodeName(node))
	node.hidden = !selfMatch && matches === 0
	const details = node.querySelector<HTMLDetailsElement>(":scope > details")
	if (details !== null && text !== "") {
		details.open = matches > 0
	}
	return matches + (selfMatch ? 1 : 0)
}

export type TreeFilter = {
	total: number
	apply: (text: string) => number
}

export function createTreeFilter(tree: HTMLElement): TreeFilter {
	const roots = [...tree.querySelectorAll<HTMLElement>(":scope > li")]
	const initiallyOpen = new Map(
		[...tree.querySelectorAll("details")].map((details) => [details, details.open]),
	)
	return {
		total: tree.querySelectorAll("li").length,
		apply(text) {
			const matched = roots.reduce((sum, root) => sum + filterNode(root, text), 0)
			if (text === "") {
				for (const [details, open] of initiallyOpen) {
					details.open = open
				}
			}
			return matched
		},
	}
}
