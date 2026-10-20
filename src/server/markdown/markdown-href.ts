import { classHref, enumHref, typeHref } from "../../api/routes.ts"
import { siteUrl } from "./site-url.ts"

export function markdownHref(href: string): string {
	return `${href.replace(/\/$/, "")}.md`
}

export function classMarkdownUrl(name: string): string {
	return siteUrl(markdownHref(classHref(name)))
}

export function enumMarkdownUrl(name: string): string {
	return siteUrl(markdownHref(enumHref(name)))
}

export function typeMarkdownUrl(name: string): string {
	return siteUrl(markdownHref(typeHref(name)))
}
