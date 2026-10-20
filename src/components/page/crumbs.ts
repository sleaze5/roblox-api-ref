import { facetTitles } from "../../api/facets/facet-info.ts"
import {
	classesHref,
	classHref,
	enumsHref,
	facetIndexHref,
	homeHref,
	indexHref,
	typesHref,
} from "../../api/routes.ts"
import type { IndexGroup } from "../../api/types.ts"
import type { Crumb } from "./Breadcrumbs.tsx"

const home: Crumb = { label: "Home", href: homeHref() }

export const rootCrumbs: Crumb[] = [home]

export function classCrumbs(ancestors: string[]): Crumb[] {
	return [
		home,
		{ label: "Classes", href: classesHref() },
		...ancestors.map((name) => ({
			label: name,
			href: classHref(name),
			code: true,
		})),
	]
}

export const enumCrumbs: Crumb[] = [home, { label: "Enums", href: enumsHref() }]

export const indexCrumbs: Crumb[] = [home, { label: "Index", href: indexHref() }]

export function indexGroupCrumbs(group: IndexGroup): Crumb[] {
	return [
		...indexCrumbs,
		group === "type"
			? { label: "Data types", href: typesHref() }
			: { label: facetTitles[group], href: facetIndexHref(group) },
	]
}
