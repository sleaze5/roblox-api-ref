import {
	classesHref,
	classHref,
	enumHref,
	enumsHref,
	typeHref,
	typesHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import type { Route } from "../match-route.ts"
import { classMarkdown } from "./class-markdown.ts"
import { enumMarkdown } from "./enum-markdown.ts"
import {
	classListMarkdown,
	enumListMarkdown,
	typeListMarkdown,
} from "./list-markdown.ts"
import { markdownHref } from "./markdown-href.ts"
import { typeMarkdown } from "./type-markdown.ts"

export type MarkdownPage = {
	href: string
	content: string
}

function page(href: string, content: string): MarkdownPage {
	return { href: markdownHref(href), content }
}

export function pageMarkdown(api: Api, route: Route): MarkdownPage | undefined {
	switch (route.page) {
		case "classes":
			return page(classesHref(), classListMarkdown(api))
		case "enums":
			return page(enumsHref(), enumListMarkdown(api))
		case "types":
			return page(typesHref(), typeListMarkdown(api))
		case "class":
			return page(classHref(route.item.name), classMarkdown(api, route.item))
		case "enum":
			return page(enumHref(route.item.name), enumMarkdown(api, route.item))
		case "type":
			return page(typeHref(route.item.name), typeMarkdown(api, route.item))
		default:
			return undefined
	}
}
