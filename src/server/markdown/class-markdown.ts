import { accessTierLabels, accessTiers, tierCounts } from "../../api/access-tiers.ts"
import { releaseText } from "../../api/release.ts"
import { classDocsHref } from "../../api/docs-links.ts"
import {
	ancestorsOf,
	memberKinds,
	memberKindTitles,
	membersInPageOrder,
} from "../../api/members.ts"
import { classHref } from "../../api/routes.ts"
import { plainTags } from "../../api/tags.ts"
import type { Api, Class, MemberKind } from "../../api/types.ts"
import { docMarkdown } from "./doc-markdown.ts"
import { factList, joinBlocks } from "./document.ts"
import { historyFacts, removedMarkdown } from "./history-markdown.ts"
import { usageMarkdown } from "./usage-markdown.ts"
import { escapeText, inlineCode, link } from "./escape.ts"
import { classMarkdownUrl } from "./markdown-href.ts"
import { memberMarkdown } from "./member-markdown.ts"
import { absoluteUrl } from "../../site/origin.ts"

function classLinks(names: string[]): string {
	return names.map((name) => link(name, classMarkdownUrl(name))).join(", ")
}

function memberSection(item: Class, kind: MemberKind): string | undefined {
	const members = membersInPageOrder(item.members, kind)
	if (members.length === 0) {
		return undefined
	}
	const entries = members.map((member) => memberMarkdown(member, item.name))
	return `## ${memberKindTitles[kind]}\n\n${entries.join("\n\n")}`
}

function inheritedSection(ancestors: Class[]): string | undefined {
	const lines = ancestors
		.filter((ancestor) => ancestor.members.length > 0)
		.map((ancestor) => {
			const names = memberKinds.flatMap((kind) =>
				membersInPageOrder(ancestor.members, kind).map((member) =>
					inlineCode(member.name),
				),
			)
			return `- From ${link(ancestor.name, classMarkdownUrl(ancestor.name))}: ${names.join(", ")}`
		})
	return lines.length === 0
		? undefined
		: `## Inherited members\n\n${lines.join("\n")}`
}

export function classMarkdown(api: Api, item: Class): string {
	const ancestors = ancestorsOf(api, item)
	return joinBlocks([
		`# ${escapeText(item.name)}`,
		`Roblox Engine API class from ${releaseText(api)}.`,
		factList([
			[
				"Inherits from, nearest first",
				classLinks(ancestors.map((ancestor) => ancestor.name)),
			],
			["Subclasses", classLinks(item.subclasses)],
			[
				"Tags",
				plainTags(item.tags)
					.map((tag) => tag.label)
					.join(", "),
			],
			["Memory category", item.memoryCategory],
			[
				"Access, by own members",
				item.members.length === 0
					? undefined
					: accessTiers
							.map(
								(tier) =>
									`${accessTierLabels[tier]}: ${tierCounts(item.members)[tier]}`,
							)
							.join(", "),
			],
			...historyFacts(item.history, true),
			["HTML page", absoluteUrl(classHref(item.name))],
			["Roblox documentation", classDocsHref(item.name)],
		]),
		docMarkdown(item.description),
		...memberKinds.map((kind) => memberSection(item, kind)),
		inheritedSection(ancestors),
		usageMarkdown(item.usedBy),
		removedMarkdown("Removed members", "Class", item.name, item.removedMembers),
	])
}
