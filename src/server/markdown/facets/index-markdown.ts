import {
	facetIndexDescriptions,
	facetTitles,
	indexDescription,
	sortedFacets,
} from "../../../api/facets/facet-info.ts"
import { releaseText } from "../../../api/release.ts"
import { facetIndexHref, indexHref, typesHref } from "../../../api/routes.ts"
import { facetKinds, type Api, type FacetKind } from "../../../api/types.ts"
import { absoluteUrl } from "../../../site/origin.ts"
import { joinBlocks } from "../document.ts"
import { link } from "../escape.ts"
import {
	facetIndexMarkdownUrl,
	facetMarkdownUrl,
	markdownUrl,
} from "../markdown-href.ts"

function count(value: number, noun: string): string {
	return `${value} ${noun}${value === 1 ? "" : "s"}`
}

function facetLines(api: Api, kind: FacetKind): string {
	return sortedFacets(api, kind)
		.map(
			(facet) =>
				`- ${link(facet.name, facetMarkdownUrl(kind, facet.name))}: ${count(facet.uses.length, "use")}`,
		)
		.join("\n")
}

function intro(api: Api, description: string, href: string): string {
	return `${description} From ${releaseText(api)}. The HTML version of this list is at ${absoluteUrl(href)}.`
}

export function facetIndexMarkdown(api: Api, kind: FacetKind): string {
	return joinBlocks([
		`# ${facetTitles[kind]}`,
		intro(api, facetIndexDescriptions[kind], facetIndexHref(kind)),
		facetLines(api, kind),
	])
}

export function indexMarkdown(api: Api): string {
	return joinBlocks([
		"# Index",
		intro(api, indexDescription, indexHref()),
		`## Data types\n\n- ${link("Data types", markdownUrl(typesHref()))}: ${count(Object.keys(api.dataTypes).length, "data type")}`,
		...facetKinds.map(
			(kind) =>
				`## ${facetTitles[kind]}\n\n${link(`All ${facetTitles[kind].toLowerCase()}`, facetIndexMarkdownUrl(kind))}\n\n${facetLines(api, kind)}`,
		),
	])
}
