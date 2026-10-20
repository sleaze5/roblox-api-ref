import generated from "../../generated/facet-slugs.json"
import type { FacetKind } from "../types.ts"

const slugs = generated as Record<
	FacetKind,
	{
		toSlug: Record<string, string>
		toName: Record<string, string>
	}
>

export function facetSlug(kind: FacetKind, name: string): string {
	return slugs[kind].toSlug[name] ?? name
}

export function facetName(kind: FacetKind, slug: string): string {
	return slugs[kind].toName[slug] ?? slug
}
