import { facetKindOfPath, facetPaths } from "../api/facets/facet-info.ts"
import { facetName, facetSlug } from "../api/facets/facet-slug.ts"
import {
	facetKinds,
	type Api,
	type Class,
	type DataType,
	type Enum,
	type Facet,
	type FacetKind,
} from "../api/types.ts"
import type { Update } from "../api/update-types.ts"
import type { SitePageName } from "../site/site-pages.ts"

export type Route =
	| { page: "home" }
	| { page: "classes" }
	| { page: "enums" }
	| { page: "index" }
	| { page: "types" }
	| { page: "facet-index"; kind: FacetKind }
	| { page: "class"; item: Class }
	| { page: "enum"; item: Enum }
	| { page: "type"; item: DataType }
	| { page: "facet"; facet: Facet }
	| { page: "updates" }
	| { page: "update"; item: Update }
	| { page: "site"; name: SitePageName }
	| { page: "not-found" }

const topPages = {
	classes: { page: "classes" },
	enums: { page: "enums" },
	index: { page: "index" },
	updates: { page: "updates" },
	about: { page: "site", name: "about" },
	contact: { page: "site", name: "contact" },
	privacy: { page: "site", name: "privacy" },
} as const satisfies Record<string, Route>

export function routePaths(api: Api, updates: Update[]): string[] {
	return [
		"/",
		...Object.keys(topPages).map((name) => `/${name}/`),
		"/404",
		...Object.keys(api.classes).map(
			(name) => `/classes/${encodeURIComponent(name)}/`,
		),
		...Object.keys(api.enums).map((name) => `/enums/${encodeURIComponent(name)}/`),
		...updates.map((update) => `/updates/${encodeURIComponent(update.version)}/`),
		"/index/types/",
		...Object.keys(api.dataTypes).map(
			(name) => `/index/types/${encodeURIComponent(name)}/`,
		),
		...facetKinds.flatMap((kind) =>
			[`/index/${facetPaths[kind]}/`].concat(
				Object.keys(api.facets[kind]).map(
					(name) =>
						`/index/${facetPaths[kind]}/${encodeURIComponent(facetSlug(kind, name))}/`,
				),
			),
		),
	]
}

function itemRoute(api: Api, updates: Update[], section: string, name: string): Route {
	if (section === "classes") {
		const item = api.classes[name]
		return item === undefined ? { page: "not-found" } : { page: "class", item }
	}
	if (section === "enums") {
		const item = api.enums[name]
		return item === undefined ? { page: "not-found" } : { page: "enum", item }
	}
	if (section === "updates") {
		const item = updates.find((update) => update.version === name)
		return item === undefined ? { page: "not-found" } : { page: "update", item }
	}
	return { page: "not-found" }
}

function indexRoute(api: Api, section: string, name?: string): Route {
	if (section === "types") {
		if (name === undefined) {
			return { page: "types" }
		}
		const item = api.dataTypes[name]
		return item === undefined ? { page: "not-found" } : { page: "type", item }
	}
	const kind = facetKindOfPath(section)
	if (kind === undefined) {
		return { page: "not-found" }
	}
	if (name === undefined) {
		return { page: "facet-index", kind }
	}
	const facet = api.facets[kind][facetName(kind, name)]
	return facet === undefined ? { page: "not-found" } : { page: "facet", facet }
}

export function matchRoute(api: Api, updates: Update[], path: string): Route {
	const base = import.meta.env.BASE_URL
	const relative = path.startsWith(base)
		? path.slice(base.length)
		: path.replace(/^\//, "")
	const segments = relative
		.split("/")
		.filter((segment) => segment.length > 0)
		.map((segment) => decodeURIComponent(segment))
	const [section = "", name, value] = segments
	if (segments.length === 0) {
		return { page: "home" }
	}
	if (segments.length === 1 && Object.hasOwn(topPages, section)) {
		return topPages[section as keyof typeof topPages]
	}
	if (section === "index" && name !== undefined && segments.length <= 3) {
		return indexRoute(api, name, value)
	}
	if (segments.length === 2 && name !== undefined) {
		return itemRoute(api, updates, section, name)
	}
	return { page: "not-found" }
}
