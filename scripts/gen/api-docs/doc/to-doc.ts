import {
	docTags,
	type Doc,
	type DocLink,
	type DocNode,
	type DocTag,
} from "../../../../src/api/doc-types.ts"
import type { HtmlElement, HtmlNode } from "../html/parse-html.ts"
import { normalizeNodes } from "./normalize-text.ts"

export type LinkResolver = {
	href(href: string): DocLink | undefined
	code(text: string): DocLink | undefined
}

type Context = {
	links: LinkResolver
	pre: boolean
	linked: boolean
}

const removedElements = new Set(["img", "script", "style"])
const blockElements = new Set<string>(["ul", "li", "pre"])
const allowedTags = new Set<string>(docTags)

function isDocTag(name: string): name is DocTag {
	return allowedTags.has(name)
}

function textOf(nodes: DocNode[]): string {
	return nodes
		.map((node) => (typeof node === "string" ? node : textOf(node.children)))
		.join("")
}

function append(nodes: DocNode[], node: DocNode): void {
	const last = nodes.at(-1)
	if (typeof node === "string" && typeof last === "string") {
		nodes[nodes.length - 1] = last + node
	} else if (node !== "") {
		nodes.push(node)
	}
}

type Edge = "start" | "end"

function trimNode(node: DocNode, edge: Edge): DocNode {
	if (typeof node === "string") {
		return edge === "start" ? node.trimStart() : node.trimEnd()
	}
	return { ...node, children: trimEdge(node.children, edge) }
}

function isBlank(node: DocNode): boolean {
	return typeof node === "string"
		? node === ""
		: node.tag === "br" || node.children.length === 0
}

function trimEdge(nodes: DocNode[], edge: Edge): DocNode[] {
	const trimmed = [...nodes]
	while (trimmed.length > 0) {
		const index = edge === "start" ? 0 : trimmed.length - 1
		const node = trimNode(trimmed[index] ?? "", edge)
		if (!isBlank(node)) {
			trimmed[index] = node
			break
		}
		trimmed.splice(index, 1)
	}
	return trimmed
}

function trimEdges(nodes: DocNode[]): DocNode[] {
	return trimEdge(trimEdge(nodes, "start"), "end")
}

function linkOf(
	element: HtmlElement,
	children: DocNode[],
	context: Context,
): DocLink | undefined {
	if (element.name === "a") {
		const href = element.attributes["href"]
		return href === undefined ? undefined : context.links.href(href)
	}
	if (element.name === "code" && !context.linked && !context.pre) {
		return context.links.code(textOf(children))
	}
	return undefined
}

function convertElement(element: HtmlElement, context: Context): DocNode[] {
	if (removedElements.has(element.name)) {
		return []
	}
	const inner: Context = {
		...context,
		pre: context.pre || element.name === "pre",
		linked: context.linked || element.name === "a",
	}
	const children = convertNodes(
		element.children,
		inner,
		blockElements.has(element.name),
	)
	if (!isDocTag(element.name)) {
		return children
	}
	const link = linkOf(element, children, context)
	if (element.name === "a" && link === undefined) {
		return children
	}
	return [{ tag: element.name, children, ...(link === undefined ? {} : { link }) }]
}

function convertNodes(nodes: HtmlNode[], context: Context, block: boolean): DocNode[] {
	const result: DocNode[] = []
	for (const node of nodes) {
		if (typeof node !== "string") {
			for (const converted of convertElement(node, context)) {
				append(result, converted)
			}
		} else if (context.pre) {
			append(result, node)
		} else if (!block || node.trim() !== "") {
			append(result, node.replaceAll(/\s+/g, " "))
		}
	}
	return block ? trimEdges(result) : result
}

export function toDoc(nodes: HtmlNode[], links: LinkResolver): Doc | undefined {
	const converted = convertNodes(nodes, { links, pre: false, linked: false }, false)
	const doc = trimEdges(normalizeNodes(converted))
	return textOf(doc).trim() === "" ? undefined : doc
}
