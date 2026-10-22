import {
	isExternal,
	type Block,
	type Inline,
	type PageContent,
} from "../../site/content/page-content.ts"
import { joinBlocks } from "./document.ts"
import { inlineCode, link } from "./escape.ts"
import { absoluteUrl } from "../../site/origin.ts"

function inlineMarkdown(content: Inline[]): string {
	return content
		.map((part) => {
			if (typeof part === "string") {
				return part
			}
			if ("code" in part) {
				return inlineCode(part.code)
			}
			return link(
				part.text,
				isExternal(part.href) ? part.href : absoluteUrl(part.href),
			)
		})
		.join("")
}

function blockMarkdown(block: Block): string {
	return block.kind === "paragraph"
		? inlineMarkdown(block.content)
		: block.items.map((item) => `- ${inlineMarkdown(item)}`).join("\n")
}

export function sitePageMarkdown(title: string, content: PageContent): string {
	return joinBlocks([
		`# ${title}`,
		content.lead,
		...content.sections.flatMap((section) => [
			`## ${section.heading}`,
			...section.blocks.map(blockMarkdown),
		]),
	])
}
