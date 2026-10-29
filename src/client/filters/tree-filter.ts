import type { NameMatch } from "./name-match.ts"

type TreeNode = {
	element: HTMLElement
	names: string[]
	details: HTMLDetailsElement | null
	children: TreeNode[]
}

function readNode(element: HTMLElement): TreeNode {
	const link = element.querySelector(":scope > a, :scope > details > summary > a")
	return {
		element,
		names: [
			element.dataset["name"] ?? link?.textContent ?? "",
			...(element.dataset["aliases"]?.split(" ") ?? []),
		],
		details: element.querySelector<HTMLDetailsElement>(":scope > details"),
		children: [
			...element.querySelectorAll<HTMLElement>(":scope > details > ul > li"),
		].map(readNode),
	}
}

function filterNode(node: TreeNode, match: NameMatch, filtering: boolean): number {
	let matches = 0
	for (const child of node.children) {
		matches += filterNode(child, match, filtering)
	}
	const selfMatch = node.names.some(match)
	node.element.hidden = !selfMatch && matches === 0
	if (node.details !== null && filtering) {
		node.details.open = matches > 0
	}
	return matches + (selfMatch ? 1 : 0)
}

function collectNames(node: TreeNode, names: string[]): string[] {
	names.push(...node.names)
	for (const child of node.children) {
		collectNames(child, names)
	}
	return names
}

export type TreeFilter = {
	total: number
	/** Every name that {@link TreeFilter.apply} tests. */
	names: string[]
	/** Shows the rows that match, and returns how many match. */
	apply: (match: NameMatch, text: string) => number
}

export function createTreeFilter(tree: HTMLElement): TreeFilter {
	const roots = [...tree.querySelectorAll<HTMLElement>(":scope > li")].map(readNode)
	const initiallyOpen = new Map(
		[...tree.querySelectorAll("details")].map((details) => [details, details.open]),
	)
	return {
		total: tree.querySelectorAll("li").length,
		names: roots.flatMap((root) => collectNames(root, [])),
		apply(match, text) {
			const filtering = text !== ""
			const matched = roots.reduce(
				(sum, root) => sum + filterNode(root, match, filtering),
				0,
			)
			if (!filtering) {
				for (const [details, open] of initiallyOpen) {
					details.open = open
				}
			}
			return matched
		},
	}
}
