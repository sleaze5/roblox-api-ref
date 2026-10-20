import { compareNames } from "../../../src/api/compare-names.ts"
import {
	facetKinds,
	type Facet,
	type FacetKind,
	type FacetUse,
} from "../../../src/api/types.ts"

const useKindOrder: FacetUse["kind"][] = [
	"Class",
	"Enum",
	"EnumItem",
	"Property",
	"Method",
	"Event",
	"Callback",
]

export type FacetIndex = Map<FacetKind, Map<string, FacetUse[]>>

export function createFacetIndex(): FacetIndex {
	return new Map(facetKinds.map((kind) => [kind, new Map()]))
}

export function addUse(
	index: FacetIndex,
	kind: FacetKind,
	name: string,
	use: FacetUse,
): void {
	const values = index.get(kind)
	if (values === undefined || name === "") {
		return
	}
	const uses = values.get(name) ?? []
	uses.push(use)
	values.set(name, uses)
}

function ownerOf(use: FacetUse): string {
	return "owner" in use ? use.owner : ""
}

function byUse(a: FacetUse, b: FacetUse): number {
	return (
		useKindOrder.indexOf(a.kind) - useKindOrder.indexOf(b.kind) ||
		compareNames(ownerOf(a), ownerOf(b)) ||
		compareNames(a.name, b.name)
	)
}

export function finishFacets(
	index: FacetIndex,
): Record<FacetKind, Record<string, Facet>> {
	const result = {} as Record<FacetKind, Record<string, Facet>>
	for (const [kind, values] of index) {
		const facets: Record<string, Facet> = {}
		for (const name of [...values.keys()].sort((a, b) => compareNames(a, b))) {
			facets[name] = { kind, name, uses: (values.get(name) ?? []).sort(byUse) }
		}
		result[kind] = facets
	}
	return result
}
