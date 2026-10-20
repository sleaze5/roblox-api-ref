import {
	facetDescription,
	facetIndexDescriptions,
	facetLabels,
	facetTitles,
} from "../api/facets/facet-info.ts"
import type { Route } from "./match-route.ts"

export type PageMeta = {
	title: string
	description: string
}

const siteName = "Roblox API Reference"

function titled(title: string, description: string): PageMeta {
	return { title: `${title} | ${siteName}`, description }
}

export function describeRoute(route: Route, version: string): PageMeta {
	switch (route.page) {
		case "home":
			return {
				title: siteName,
				description: `Search and browse the Roblox Engine API from Studio ${version}.`,
			}
		case "classes":
			return titled("Classes", "Every Roblox class, arranged by inheritance.")
		case "enums":
			return titled("Enums", "Every Roblox enum.")
		case "index":
			return titled(
				"Index",
				"Every data type, tag, property category, security level, capability, thread safety level, and memory category in the Roblox Engine API.",
			)
		case "types":
			return titled(
				"Data types",
				"Every data type: primitive types, group types, and Roblox types.",
			)
		case "facet-index":
			return titled(facetTitles[route.kind], facetIndexDescriptions[route.kind])
		case "class":
			return titled(
				route.item.name,
				`Properties, methods, events, and callbacks of the Roblox ${route.item.name} class.`,
			)
		case "enum":
			return titled(
				`Enum.${route.item.name}`,
				`Items of the Roblox ${route.item.name} enum and the members that use it.`,
			)
		case "type":
			return titled(
				route.item.name,
				`Every Roblox class member that uses the ${route.item.name} type.`,
			)
		case "facet":
			return titled(
				`${facetLabels[route.facet.kind]}: ${route.facet.name}`,
				facetDescription(route.facet),
			)
		case "not-found":
			return titled("Page not found", "This page does not exist.")
	}
}
