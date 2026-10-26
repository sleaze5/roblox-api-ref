import { matchesName } from "./name-match.ts"

type TreeNode = {
	element: HTMLElement
	name: string
	details: HTMLDetailsElement | null
	children: TreeNode[]
}

function readNode(element: HTMLElement): TreeNode {
	const link = element.querySelector(":scope > a, :scope > details > summary > a")
	return {
		element,
		name: link?.textContent ?? "",
		details: element.querySelector<HTMLDetailsElement>(":scope > details"),
		children: [
			...element.querySelectorAll<HTMLElement>(":scope > details > ul > li"),
		].map(readNode),
	}
}

function filterNode(node: TreeNode, text: string): number {
	let matches = 0
	for (const child of node.children) {
		matches += filterNode(child, text)
	}
	const selfMatch = matchesName(text, node.name)
	node.element.hidden = !selfMatch && matches === 0
	if (node.details !== null && text !== "") {
		node.details.open = matches > 0
	}
	return matches + (selfMatch ? 1 : 0)
}

export type TreeFilter = {
	total: number
	apply: (text: string) => number
}

export function createTreeFilter(tree: HTMLElement): TreeFilter {
	const roots = [...tree.querySelectorAll<HTMLElement>(":scope > li")].map(readNode)
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
