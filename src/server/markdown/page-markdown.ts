import {
	llmsTxtHref,
	notFoundMarkdownHref,
	classesHref,
	classHref,
	enumHref,
	enumsHref,
	facetHref,
	facetIndexHref,
	indexHref,
	typeHref,
	removedHref,
	typesHref,
	updateHref,
	updatesHref,
	statsHref,
} from "../../api/routes.ts"
import { loadStats } from "../load-stats.ts"
import { removedMarkdown } from "./removed-markdown.ts"
import type { Api } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
import { sitePages } from "../../site/site-pages.ts"
import {
	isItemRoute,
	type IndexList,
	type ItemRoute,
	type Route,
} from "../match-route.ts"
import { classMarkdown } from "./class-markdown.ts"
import { enumMarkdown } from "./enum-markdown.ts"
import { facetMarkdown } from "./facets/facet-markdown.ts"
import { facetIndexMarkdown, indexMarkdown } from "./facets/index-markdown.ts"
import {
	classListMarkdown,
	enumListMarkdown,
	typeListMarkdown,
} from "./list-markdown.ts"
import { llmsTxt } from "./llms-txt.ts"
import { markdownHref } from "./markdown-href.ts"
import { notFoundMarkdown } from "./not-found-markdown.ts"
import { sitePageMarkdown } from "./site-page-markdown.ts"
import { statsMarkdown } from "./stats-markdown.ts"
import { typeMarkdown } from "./type-markdown.ts"
import { updateListMarkdown, updateMarkdown } from "./update-markdown.ts"

export type MarkdownPage = {
	href: string
	content: string
}

function page(href: string, content: string): MarkdownPage {
	return { href: markdownHref(href), content }
}

const indexListMarkdown: Record<IndexList, (api: Api) => MarkdownPage> = {
	types: (api) => page(typesHref(), typeListMarkdown(api)),
	removed: (api) => page(removedHref(), removedMarkdown(api)),
}

function itemMarkdown(api: Api, route: ItemRoute): MarkdownPage {
	switch (route.page) {
		case "class":
			return page(classHref(route.item.name), classMarkdown(api, route.item))
		case "enum":
			return page(enumHref(route.item.name), enumMarkdown(api, route.item))
		case "type":
			return page(typeHref(route.item.name), typeMarkdown(api, route.item))
		case "update":
			return page(updateHref(route.item.version), updateMarkdown(route.item))
		case "facet":
			return page(
				facetHref(route.facet.kind, route.facet.name),
				facetMarkdown(api, route.facet),
			)
	}
}

export function pageMarkdown(api: Api, updates: Update[], route: Route): MarkdownPage {
	if (isItemRoute(route)) {
		return itemMarkdown(api, route)
	}
	switch (route.page) {
		case "home":
			return { href: llmsTxtHref(), content: llmsTxt(api) }
		case "site": {
			const site = sitePages[route.name]
			return page(site.href, sitePageMarkdown(site.title, site.content(api)))
		}
		case "classes":
			return page(classesHref(), classListMarkdown(api))
		case "enums":
			return page(enumsHref(), enumListMarkdown(api))
		case "index-list":
			return indexListMarkdown[route.list](api)
		case "updates":
			return page(updatesHref(), updateListMarkdown(updates))
		case "stats":
			return page(statsHref(), statsMarkdown(api, loadStats()))
		case "index":
			return page(indexHref(), indexMarkdown(api))
		case "facet-index":
			return page(facetIndexHref(route.kind), facetIndexMarkdown(api, route.kind))
		case "not-found":
			return { href: notFoundMarkdownHref(), content: notFoundMarkdown(api) }
	}
}
