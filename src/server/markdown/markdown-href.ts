import {
	classHref,
	enumHref,
	facetHref,
	facetIndexHref,
	typeHref,
} from "../../api/routes.ts"
import type { FacetKind } from "../../api/types.ts"
import { absoluteUrl } from "../../site/origin.ts"

export function markdownHref(href: string): string {
	return `${href.replace(/\/$/, "")}.md`
}

export function markdownUrl(href: string): string {
	return absoluteUrl(markdownHref(href))
}

export function classMarkdownUrl(name: string): string {
	return markdownUrl(classHref(name))
}

export function enumMarkdownUrl(name: string): string {
	return markdownUrl(enumHref(name))
}

export function typeMarkdownUrl(name: string): string {
	return markdownUrl(typeHref(name))
}

export function facetMarkdownUrl(kind: FacetKind, name: string): string {
	return markdownUrl(facetHref(kind, name))
}

export function facetIndexMarkdownUrl(kind: FacetKind): string {
	return markdownUrl(facetIndexHref(kind))
}
