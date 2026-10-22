import {
	facetIndexHref,
	facetHref,
	classesHref,
	classHref,
	enumHref,
	enumsHref,
	homeHref,
	indexHref,
	typeHref,
	typesHref,
	updateHref,
	updatesHref,
} from "../api/routes.ts"
import { facetTitles } from "../api/facets/facet-info.ts"
import { sitePages } from "../site/site-pages.ts"
import type { Route } from "./match-route.ts"

export type Breadcrumb = {
	name: string
	href: string
}

const home: Breadcrumb = { name: "Roblox API Reference", href: homeHref() }
const classes: Breadcrumb = { name: "Classes", href: classesHref() }
const enums: Breadcrumb = { name: "Enums", href: enumsHref() }
const index: Breadcrumb = { name: "Index", href: indexHref() }
const types: Breadcrumb = { name: "Data types", href: typesHref() }
const updates: Breadcrumb = { name: "Updates", href: updatesHref() }

export function breadcrumbs(route: Route): Breadcrumb[] {
	switch (route.page) {
		case "home":
		case "not-found":
			return []
		case "classes":
			return [home, classes]
		case "enums":
			return [home, enums]
		case "index":
			return [home, index]
		case "types":
			return [home, index, types]
		case "updates":
			return [home, updates]
		case "site":
			return [
				home,
				{ name: sitePages[route.name].title, href: sitePages[route.name].href },
			]
		case "update":
			return [
				home,
				updates,
				{
					name: route.item.version,
					href: updateHref(route.item.version),
				},
			]
		case "facet-index":
			return [
				home,
				index,
				{ name: facetTitles[route.kind], href: facetIndexHref(route.kind) },
			]
		case "class":
			return [
				home,
				classes,
				{ name: route.item.name, href: classHref(route.item.name) },
			]
		case "enum":
			return [
				home,
				enums,
				{ name: `Enum.${route.item.name}`, href: enumHref(route.item.name) },
			]
		case "type":
			return [
				home,
				index,
				types,
				{ name: route.item.name, href: typeHref(route.item.name) },
			]
		case "facet":
			return [
				home,
				index,
				{
					name: facetTitles[route.facet.kind],
					href: facetIndexHref(route.facet.kind),
				},
				{
					name: route.facet.name,
					href: facetHref(route.facet.kind, route.facet.name),
				},
			]
	}
}
