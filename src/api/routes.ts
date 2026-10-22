import { facetPaths } from "./facets/facet-info.ts"
import { facetSlug } from "./facets/facet-slug.ts"
import type { FacetKind, MemberKind } from "./types.ts"

const base = import.meta.env.BASE_URL

export function homeHref(): string {
	return base
}

export function classesHref(): string {
	return `${base}classes/`
}

export function enumsHref(): string {
	return `${base}enums/`
}

export function indexHref(): string {
	return `${base}index/`
}

export function updatesHref(): string {
	return `${base}updates/`
}

export function aboutHref(): string {
	return `${base}about/`
}

export function contactHref(): string {
	return `${base}contact/`
}

export function privacyHref(): string {
	return `${base}privacy/`
}

export function llmsTxtHref(): string {
	return `${base}llms.txt`
}

export function updateHref(version: string): string {
	return `${updatesHref()}${encodeURIComponent(version)}/`
}

export function facetHref(kind: FacetKind, name: string): string {
	return `${facetIndexHref(kind)}${encodeURIComponent(facetSlug(kind, name))}/`
}

export function facetIndexHref(kind: FacetKind): string {
	return `${indexHref()}${facetPaths[kind]}/`
}

export function typesHref(): string {
	return `${indexHref()}types/`
}

export function classHref(name: string): string {
	return `${classesHref()}${encodeURIComponent(name)}/`
}

export function enumHref(name: string): string {
	return `${enumsHref()}${encodeURIComponent(name)}/`
}

export function typeHref(name: string): string {
	return `${typesHref()}${encodeURIComponent(name)}/`
}

export function memberAnchor(kind: MemberKind, name: string): string {
	return `${kind}-${name}`
}

export function enumItemAnchor(name: string): string {
	return `Item-${name}`
}

export function memberHref(className: string, kind: MemberKind, name: string): string {
	return `${classHref(className)}#${memberAnchor(kind, name)}`
}

export function enumItemHref(enumName: string, itemName: string): string {
	return `${enumHref(enumName)}#${enumItemAnchor(itemName)}`
}
