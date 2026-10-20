import type { Doc, DocElement, DocLink, DocNode } from "../../api/doc-types.ts"
import { escapeText, inlineCode, link } from "./escape.ts"
import { classMarkdownUrl, enumMarkdownUrl, typeMarkdownUrl } from "./markdown-href.ts"

function linkUrl(target: DocLink): string {
	switch (target.kind) {
		case "class":
			return classMarkdownUrl(target.name)
		case "member":
			return classMarkdownUrl(target.owner)
		case "enum":
			return enumMarkdownUrl(target.name)
		case "enumItem":
			return enumMarkdownUrl(target.owner)
		case "type":
			return typeMarkdownUrl(target.name)
		case "external":
			return target.url
	}
}

function plainText(nodes: DocNode[]): string {
	return nodes
		.map((node) => (typeof node === "string" ? node : plainText(node.children)))
		.join("")
}

function linked(element: DocElement, text: string): string {
	return element.link === undefined ? text : link(text, linkUrl(element.link))
}

function listItems(element: DocElement): string {
	const items = element.children
		.filter((child): child is DocElement => typeof child !== "string")
		.map((child) => `- ${nodesMarkdown(child.children).trim()}`)
	return `\n\n${items.join("\n")}\n\n`
}

function elementMarkdown(element: DocElement): string {
	const children = (): string => nodesMarkdown(element.children)
	switch (element.tag) {
		case "a":
			return linked(element, children())
		case "br":
			return "\n\n"
		case "code":
			return linked(element, inlineCode(plainText(element.children)))
		case "em":
			return `*${children()}*`
		case "strong":
			return `**${children()}**`
		case "kbd":
			return inlineCode(plainText(element.children))
		case "sup":
			return children()
		case "li":
			return `\n- ${children().trim()}`
		case "pre":
			return `\n\n\`\`\`luau\n${plainText(element.children).trim()}\n\`\`\`\n\n`
		case "ul":
			return listItems(element)
	}
}

function nodesMarkdown(nodes: DocNode[]): string {
	return nodes
		.map((node) =>
			typeof node === "string" ? escapeText(node) : elementMarkdown(node),
		)
		.join("")
}

export function docMarkdown(doc: Doc | undefined): string {
	if (doc === undefined) {
		return ""
	}
	return nodesMarkdown(doc)
		.replaceAll(/[ \t]+\n/g, "\n")
		.replaceAll(/\n{3,}/g, "\n\n")
		.trim()
}
