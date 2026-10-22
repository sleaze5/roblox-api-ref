import { releaseText } from "../../api/release.ts"
import { compareNames } from "../../api/compare-names.ts"
import { classesHref, enumsHref, typesHref } from "../../api/routes.ts"
import { plainTags } from "../../api/tags.ts"
import type { Api } from "../../api/types.ts"
import { joinBlocks } from "./document.ts"
import { link } from "./escape.ts"
import { classMarkdownUrl, enumMarkdownUrl, typeMarkdownUrl } from "./markdown-href.ts"
import { absoluteUrl } from "../../site/origin.ts"

function sortedNames(items: Record<string, unknown>): string[] {
	return Object.keys(items).sort(compareNames)
}

function tagSuffix(tags: string[]): string {
	const labels = plainTags(tags).map((tag) => tag.label)
	return labels.length === 0 ? "" : ` (${labels.join(", ")})`
}

function intro(kind: string, api: Api, href: string): string {
	return `Every ${kind} in the Roblox Engine API from ${releaseText(api)}. The HTML version of this list is at ${absoluteUrl(href)}.`
}

export function classListMarkdown(api: Api): string {
	const lines = sortedNames(api.classes).map((name) => {
		const item = api.classes[name]
		const parent =
			item?.superclass === undefined ? "" : `: inherits from ${item.superclass}`
		return `- ${link(name, classMarkdownUrl(name))}${parent}${tagSuffix(item?.tags ?? [])}`
	})
	return joinBlocks([
		"# Classes",
		intro("class", api, classesHref()),
		lines.join("\n"),
	])
}

export function enumListMarkdown(api: Api): string {
	const lines = sortedNames(api.enums).map(
		(name) =>
			`- ${link(`Enum.${name}`, enumMarkdownUrl(name))}${tagSuffix(api.enums[name]?.tags ?? [])}`,
	)
	return joinBlocks(["# Enums", intro("enum", api, enumsHref()), lines.join("\n")])
}

export function typeListMarkdown(api: Api): string {
	const lines = sortedNames(api.dataTypes).map(
		(name) =>
			`- ${link(name, typeMarkdownUrl(name))}: ${api.dataTypes[name]?.category ?? ""}`,
	)
	return joinBlocks([
		"# Data types",
		intro("data type", api, typesHref()),
		lines.join("\n"),
	])
}
