import { sitePages } from "../site/site-pages.ts"
import type { Route } from "./match-route.ts"
import type { SchemaType } from "./page-module.ts"

export function schemaType(route: Route): SchemaType | undefined {
	switch (route.page) {
		case "home":
			return "WebSite"
		case "class":
		case "enum":
		case "type":
		case "update":
			return "TechArticle"
		case "stats":
			return "WebPage"
		case "site":
			return sitePages[route.name].schemaType
		case "not-found":
			return undefined
		default:
			return "CollectionPage"
	}
}
