import { releaseText } from "../../api/release.ts"
import { enumDocsHref } from "../../api/docs-links.ts"
import { enumHref } from "../../api/routes.ts"
import { plainTags } from "../../api/tags.ts"
import type { Api, Enum, EnumItem } from "../../api/types.ts"
import { docMarkdown } from "./doc-markdown.ts"
import { factList, joinBlocks } from "./document.ts"
import { historyFacts, removedMarkdown } from "./history-markdown.ts"
import { escapeText, inlineCode, tableCell } from "./escape.ts"
import { absoluteUrl } from "../../site/origin.ts"
import { usageMarkdown } from "./usage-markdown.ts"

function itemRow(item: EnumItem): string {
	const notes = [
		...plainTags(item.tags).map((tag) => tag.label),
		...item.legacyNames.map((name) => `Legacy name: ${inlineCode(name)}`),
	]
	const description = [notes.join(", "), docMarkdown(item.description)]
		.filter((part) => part !== "")
		.join(". ")
	return `| ${inlineCode(item.name)} | ${item.value} | ${tableCell(description)} |`
}

function itemTable(items: EnumItem[]): string {
	if (items.length === 0) {
		return "This enum has no items."
	}
	const rows = [...items].sort((a, b) => a.value - b.value).map(itemRow)
	return `## Items\n\n| Name | Value | Description |\n| --- | --- | --- |\n${rows.join("\n")}`
}

export function enumMarkdown(api: Api, item: Enum): string {
	return joinBlocks([
		`# Enum.${escapeText(item.name)}`,
		`Roblox Engine API enum from ${releaseText(api)}.`,
		factList([
			[
				"Tags",
				plainTags(item.tags)
					.map((tag) => tag.label)
					.join(", "),
			],
			...historyFacts(item.history, true),
			["HTML page", absoluteUrl(enumHref(item.name))],
			["Roblox documentation", enumDocsHref(item.name)],
		]),
		docMarkdown(item.description),
		itemTable(item.items),
		usageMarkdown(item.usedBy),
		removedMarkdown("Removed items", "Enum", item.name, item.removedItems),
	])
}
