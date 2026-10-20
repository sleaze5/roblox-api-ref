import { describe, expect, test } from "vite-plus/test"
import {
	classHref,
	enumHref,
	facetHref,
	facetIndexHref,
	typeHref,
	typesHref,
} from "../api/routes.ts"
import { facetKinds } from "../api/types.ts"
import { loadApi } from "./load-api.ts"
import { matchRoute, routePaths } from "./match-route.ts"

const api = loadApi()

describe("static routes", () => {
	test("resolves every generated page without duplicate paths", () => {
		const paths = routePaths(api)
		expect(new Set(paths).size).toBe(paths.length)
		expect(new Set(paths.map((path) => path.toLowerCase())).size).toBe(paths.length)
		expect(
			paths.filter((path) => matchRoute(api, path).page === "not-found"),
		).toEqual(["/404"])
	})

	test("gives classes, enums, and types matching listing and detail URLs", () => {
		expect(classHref("Part")).toBe("/classes/Part/")
		expect(enumHref("Material")).toBe("/enums/Material/")
		expect(typeHref("Array")).toBe("/index/types/Array/")
		expect(matchRoute(api, typesHref())).toEqual({ page: "types" })
		expect(matchRoute(api, classHref("Part"))).toEqual({
			page: "class",
			item: api.classes["Part"],
		})
		expect(matchRoute(api, enumHref("Material"))).toEqual({
			page: "enum",
			item: api.enums["Material"],
		})
		expect(matchRoute(api, typeHref("Array"))).toEqual({
			page: "type",
			item: api.dataTypes["Array"],
		})
	})

	test("resolves every facet's listing and encoded detail links", () => {
		const paths = new Set(routePaths(api))
		for (const kind of facetKinds) {
			expect(matchRoute(api, facetIndexHref(kind))).toEqual({
				page: "facet-index",
				kind,
			})
			for (const facet of Object.values(api.facets[kind])) {
				const href = facetHref(kind, facet.name)
				expect(paths.has(href)).toBe(true)
				expect(matchRoute(api, href)).toEqual({ page: "facet", facet })
			}
		}
		expect(facetHref("propertyCategory", "Body Parts")).toBe(
			"/index/property-categories/Body%20Parts/",
		)
	})

	test("rejects unknown values, extra path segments, and retired URL shapes", () => {
		for (const path of [
			"/index/unknown/",
			"/index/types/unknown/",
			"/index/tags/unknown/",
			"/index/types/Array/extra/",
			"/classes/Part/extra/",
			"/class/Part/",
			"/enum/Material/",
			"/type/Array/",
			"/tag/Deprecated/",
		]) {
			expect(matchRoute(api, path)).toEqual({ page: "not-found" })
		}
	})
})
