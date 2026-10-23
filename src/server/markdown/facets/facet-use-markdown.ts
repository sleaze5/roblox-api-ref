import { useKindTitles, useSections, useText } from "../../../api/facets/facet-use.ts"
import type { FacetUse } from "../../../api/types.ts"
import { link } from "../escape.ts"
import { classMarkdownUrl, enumMarkdownUrl } from "../markdown-href.ts"

function useMarkdownUrl(use: FacetUse): string {
	switch (use.kind) {
		case "Class":
			return classMarkdownUrl(use.name)
		case "Enum":
			return enumMarkdownUrl(use.name)
		case "EnumItem":
			return enumMarkdownUrl(use.owner)
		default:
			return classMarkdownUrl(use.owner)
	}
}

function useLine(use: FacetUse): string {
	const access =
		"access" in use && use.access !== undefined ? ` (${use.access} access)` : ""
	return `- ${link(useText(use), useMarkdownUrl(use))}${access}`
}

export function facetUsesMarkdown(uses: FacetUse[]): string {
	return useSections(uses)
		.map(
			(section) =>
				`## ${useKindTitles[section.kind]}\n\n${section.uses.map(useLine).join("\n")}`,
		)
		.join("\n\n")
}
