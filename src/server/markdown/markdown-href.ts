import { classHref, enumHref, typeHref } from "../../api/routes.ts"
import { absoluteUrl } from "../../site/origin.ts"

export function markdownHref(href: string): string {
	return `${href.replace(/\/$/, "")}.md`
}

export function classMarkdownUrl(name: string): string {
	return absoluteUrl(markdownHref(classHref(name)))
}

export function enumMarkdownUrl(name: string): string {
	return absoluteUrl(markdownHref(enumHref(name)))
}

export function typeMarkdownUrl(name: string): string {
	return absoluteUrl(markdownHref(typeHref(name)))
}
