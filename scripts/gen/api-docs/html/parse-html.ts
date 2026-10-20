import { decodeEntities } from "./decode-entities.ts"

export type HtmlElement = {
	name: string
	attributes: Record<string, string>
	children: HtmlNode[]
}

export type HtmlNode = string | HtmlElement

const tokenPattern =
	/<!--[\s\S]*?(?:-->|$)|<(\/?)([a-z][a-z0-9]*)((?:\s+[^\s"'>/=]+(?:\s*=\s*(?:"[^"]*"|'[^']*'|[^\s"'>]+))?)*)\s*(\/?)>|([^<]+|<)/giy
const attributePattern = /([^\s"'>/=]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'>]+)))?/g
const voidElements = new Set(["br", "hr", "img", "wbr"])

function parseAttributes(source: string): Record<string, string> {
	const attributes: Record<string, string> = {}
	for (const [, name = "", double, single, bare] of source.matchAll(
		attributePattern,
	)) {
		attributes[name.toLowerCase()] = decodeEntities(double ?? single ?? bare ?? "")
	}
	return attributes
}

function closeElement(stack: HtmlElement[], name: string): void {
	const index = stack.findLastIndex((element) => element.name === name)
	if (index > 0) {
		stack.length = index
	}
}

function openElement(
	stack: HtmlElement[],
	name: string,
	attributes: string,
	selfClosing: boolean,
): void {
	const element: HtmlElement = {
		name,
		attributes: parseAttributes(attributes),
		children: [],
	}
	stack.at(-1)?.children.push(element)
	if (!selfClosing && !voidElements.has(name)) {
		stack.push(element)
	}
}

export function parseHtml(source: string): HtmlNode[] {
	const root: HtmlElement = { name: "#root", attributes: {}, children: [] }
	const stack = [root]
	for (const [
		,
		closing,
		rawName,
		attributes = "",
		selfClosing,
		text,
	] of source.matchAll(tokenPattern)) {
		if (text !== undefined) {
			stack.at(-1)?.children.push(decodeEntities(text))
		} else if (rawName !== undefined) {
			const name = rawName.toLowerCase()
			if (closing === "/") {
				closeElement(stack, name)
			} else {
				openElement(stack, name, attributes, selfClosing === "/")
			}
		}
	}
	return root.children
}
