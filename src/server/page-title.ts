import { facetLabels, facetTitles } from "../api/facets/facet-info.ts"
import { brandTitle } from "../site/brand.ts"
import { sitePages } from "../site/site-pages.ts"
import type { Route } from "./match-route.ts"

function pageName(route: Route): string | undefined {
	switch (route.page) {
		case "home":
			return undefined
		case "classes":
			return "Classes"
		case "enums":
			return "Enums"
		case "index":
			return "Index"
		case "types":
			return "Data types"
		case "facet-index":
			return facetTitles[route.kind]
		case "class":
		case "type":
			return route.item.name
		case "enum":
			return `Enum.${route.item.name}`
		case "facet":
			return `${facetLabels[route.facet.kind]}: ${route.facet.name}`
		case "updates":
			return "Updates"
		case "update":
			return `Update ${route.item.version}`
		case "site":
			return sitePages[route.name].title
		case "not-found":
			return "Page not found"
	}
}

export function pageTitle(route: Route): string {
	return brandTitle(pageName(route))
}
