import { compareNames } from "../compare-names.ts"
import { securityRank } from "../tags.ts"
import {
	facetKinds,
	securityLevels,
	type Api,
	type Facet,
	type FacetKind,
	type Security,
} from "../types.ts"

export const facetTitles: Record<FacetKind, string> = {
	tag: "Tags",
	propertyCategory: "Property categories",
	security: "Security levels",
	capability: "Capabilities",
	threadSafety: "Thread safety",
	memoryCategory: "Memory categories",
}

export const facetLabels: Record<FacetKind, string> = {
	tag: "Tag",
	propertyCategory: "Property category",
	security: "Security level",
	capability: "Capability",
	threadSafety: "Thread safety",
	memoryCategory: "Memory category",
}

export const facetPaths: Record<FacetKind, string> = {
	tag: "tags",
	propertyCategory: "property-categories",
	security: "security-levels",
	capability: "capabilities",
	threadSafety: "thread-safety",
	memoryCategory: "memory-categories",
}

export const facetIndexDescriptions: Record<FacetKind, string> = {
	tag: "Every tag applied to classes, members, enums, and enum items.",
	propertyCategory:
		"Every property category, as grouped in Studio’s Properties window.",
	security: "Every security level required to read, write, or call a member.",
	capability: "Every capability required by a member.",
	threadSafety: "Every thread safety level assigned to a member.",
	memoryCategory: "Every memory category assigned to a class.",
}

export function facetKindOfPath(path: string): FacetKind | undefined {
	return facetKinds.find((kind) => facetPaths[kind] === path)
}

function isSecurity(name: string): name is Security {
	return (securityLevels as readonly string[]).includes(name)
}

function compareFacets(a: Facet, b: Facet): number {
	if (a.kind === "security" && isSecurity(a.name) && isSecurity(b.name)) {
		return securityRank(a.name) - securityRank(b.name)
	}
	return compareNames(a.name, b.name)
}

export function sortedFacets(api: Api, kind: FacetKind): Facet[] {
	return Object.values(api.facets[kind]).sort(compareFacets)
}

export function facetDescription(facet: Facet): string {
	switch (facet.kind) {
		case "propertyCategory":
			return `Every property in the ${facet.name} category.`
		case "tag":
			return `Every class, member, enum, and enum item that the API dump tags ${facet.name}.`
		case "security":
			return `Every member that needs ${facet.name} to read, write, or call it.`
		case "capability":
			return `Every member that needs the ${facet.name} capability.`
		case "threadSafety":
			return `Every member whose thread safety is ${facet.name}.`
		case "memoryCategory":
			return `Every class whose instances count toward the ${facet.name} memory category.`
	}
}
