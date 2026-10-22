import { releaseText } from "../../api/release.ts"
import { dataTypeDocsHref } from "../../api/docs-links.ts"
import { luauName } from "../../api/luau-type.ts"
import { typeHref } from "../../api/routes.ts"
import type { Api, DataType } from "../../api/types.ts"
import { factList, joinBlocks } from "./document.ts"
import { escapeText, inlineCode } from "./escape.ts"
import { absoluteUrl } from "../../site/origin.ts"
import { usageMarkdown } from "./usage-markdown.ts"

export function typeMarkdown(api: Api, item: DataType): string {
	const luau = luauName(item.name)
	return joinBlocks([
		`# ${escapeText(item.name)}`,
		`Roblox Engine API data type from ${releaseText(api)}.`,
		factList([
			["Category", item.category],
			["Luau type", luau === item.name ? undefined : inlineCode(luau)],
			["HTML page", absoluteUrl(typeHref(item.name))],
			["Roblox documentation", dataTypeDocsHref(item.name)],
		]),
		usageMarkdown(item.usedBy) ?? "No class member uses this type.",
	])
}
