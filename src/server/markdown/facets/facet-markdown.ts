import { facetDescription, facetLabels } from "../../../api/facets/facet-info.ts"
import { releaseText } from "../../../api/release.ts"
import { facetHref } from "../../../api/routes.ts"
import { identitiesAllowing } from "../../../api/security-access.ts"
import {
	securityLevels,
	type Api,
	type Facet,
	type Security,
} from "../../../api/types.ts"
import { absoluteUrl } from "../../../site/origin.ts"
import { factList, joinBlocks } from "../document.ts"
import { escapeText } from "../escape.ts"
import { facetUsesMarkdown } from "./facet-use-markdown.ts"

function isSecurity(name: string): name is Security {
	return (securityLevels as readonly string[]).includes(name)
}

function identities(facet: Facet): string | undefined {
	return facet.kind === "security" && isSecurity(facet.name)
		? identitiesAllowing(facet.name).join(", ")
		: undefined
}

export function facetMarkdown(api: Api, facet: Facet): string {
	return joinBlocks([
		`# ${escapeText(facet.name)}`,
		`${facetDescription(facet)} From ${releaseText(api)}.`,
		factList([
			["Kind", facetLabels[facet.kind]],
			["Uses", String(facet.uses.length)],
			["Usable by identities", identities(facet)],
			["HTML page", absoluteUrl(facetHref(facet.kind, facet.name))],
		]),
		facetUsesMarkdown(facet.uses),
	])
}
