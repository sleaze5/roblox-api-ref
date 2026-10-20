import { compareNames } from "../../api/compare-names.ts"
import type { TypeUsage } from "../../api/types.ts"
import { inlineCode, link } from "./escape.ts"
import { classMarkdownUrl } from "./markdown-href.ts"

function roleLabel(usage: TypeUsage): string {
	return usage.role === "Value"
		? usage.kind.toLowerCase()
		: `${usage.kind.toLowerCase()} ${usage.role.toLowerCase()}`
}

function classUsages(usages: TypeUsage[]): string {
	const byRole = Map.groupBy(usages, roleLabel)
	return [...byRole]
		.map(
			([role, list]) =>
				`${role} ${list.map((usage) => inlineCode(usage.memberName)).join(", ")}`,
		)
		.join("; ")
}

export function usageMarkdown(usages: TypeUsage[]): string | undefined {
	if (usages.length === 0) {
		return undefined
	}
	const byClass = Map.groupBy(usages, (usage) => usage.className)
	const lines = [...byClass]
		.sort(([a], [b]) => compareNames(a, b))
		.map(
			([className, list]) =>
				`- ${link(className, classMarkdownUrl(className))}: ${classUsages(list)}`,
		)
	return `## Used by\n\nEach entry lists the class, then the role and names of the members that use this type.\n\n${lines.join("\n")}`
}
