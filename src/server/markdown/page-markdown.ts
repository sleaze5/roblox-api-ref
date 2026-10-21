import {
	classesHref,
	classHref,
	enumHref,
	enumsHref,
	typeHref,
	typesHref,
	updateHref,
	updatesHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
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
import { updateListMarkdown, updateMarkdown } from "./update-markdown.ts"

export type MarkdownPage = {
	href: string
	content: string
}

function page(href: string, content: string): MarkdownPage {
	return { href: markdownHref(href), content }
}

export function pageMarkdown(
	api: Api,
	updates: Update[],
	route: Route,
): MarkdownPage | undefined {
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
		case "updates":
			return page(updatesHref(), updateListMarkdown(updates))
		case "update":
			return page(updateHref(route.item.version), updateMarkdown(route.item))
		default:
			return undefined
	}
}
