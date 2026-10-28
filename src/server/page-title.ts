import { facetLabels, facetTitles } from "../api/facets/facet-info.ts"
import { brandTitle } from "../site/brand.ts"
import { sitePages } from "../site/site-pages.ts"
import {
	isItemRoute,
	type IndexList,
	type ItemRoute,
	type Route,
} from "./match-route.ts"

function itemName(route: ItemRoute): string {
	switch (route.page) {
		case "class":
		case "type":
			return route.item.name
		case "enum":
			return `Enum.${route.item.name}`
		case "facet":
			return `${facetLabels[route.facet.kind]}: ${route.facet.name}`
		case "update":
			return `Update ${route.item.version}`
	}
}

function pageName(route: Route): string | undefined {
	if (isItemRoute(route)) {
		return itemName(route)
	}
	switch (route.page) {
		case "home":
			return undefined
		case "classes":
			return "Classes"
		case "enums":
			return "Enums"
		case "index":
			return "Index"
		case "index-list":
			return indexListTitles[route.list]
		case "facet-index":
			return facetTitles[route.kind]
		case "updates":
			return "Updates"
		case "stats":
			return "Statistics"
		case "site":
			return sitePages[route.name].title
		case "not-found":
			return "Page not found"
	}
}

const indexListTitles: Record<IndexList, string> = {
	types: "Data types",
	removed: "Removed",
}

export function pageTitle(route: Route): string {
	return brandTitle(pageName(route))
}
