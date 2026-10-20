import { facetKinds, type Api } from "../../../src/api/types.ts"

export function buildFacetSlugs(facets: Api["facets"]) {
	return Object.fromEntries(
		facetKinds.map((kind) => {
			const names = Object.keys(facets[kind]).sort()
			const reserved = new Set(names.map((name) => name.toLowerCase()))
			const seen = new Set<string>()
			const toSlug: Record<string, string> = {}
			const toName: Record<string, string> = {}
			for (const name of names) {
				const folded = name.toLowerCase()
				if (seen.has(folded)) {
					let suffix = 2
					while (reserved.has(`${folded}-${suffix}`)) {
						suffix += 1
					}
					const slug = `${name}-${suffix}`
					reserved.add(slug.toLowerCase())
					toSlug[name] = slug
					toName[slug] = name
				}
				seen.add(folded)
			}
			return [kind, { toSlug, toName }]
		}),
	)
}
