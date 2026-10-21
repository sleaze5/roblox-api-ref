import { describe, expect, test } from "vite-plus/test"
import {
	classHref,
	enumHref,
	facetHref,
	facetIndexHref,
	typeHref,
	typesHref,
	updateHref,
	updatesHref,
} from "../api/routes.ts"
import { facetKinds } from "../api/types.ts"
import { loadApi } from "./load-api.ts"
import { loadUpdates } from "./load-updates.ts"
import { matchRoute, routePaths } from "./match-route.ts"

const api = loadApi()
const updates = loadUpdates()

describe("static routes", () => {
	test("resolves every generated page without duplicate paths", () => {
		const paths = routePaths(api, updates)
		expect(new Set(paths).size).toBe(paths.length)
		expect(new Set(paths.map((path) => path.toLowerCase())).size).toBe(paths.length)
		expect(
			paths.filter((path) => matchRoute(api, updates, path).page === "not-found"),
		).toEqual(["/404"])
	})

	test("gives classes, enums, and types matching listing and detail URLs", () => {
		expect(classHref("Part")).toBe("/classes/Part/")
		expect(enumHref("Material")).toBe("/enums/Material/")
		expect(typeHref("Array")).toBe("/index/types/Array/")
		expect(matchRoute(api, updates, typesHref())).toEqual({ page: "types" })
		expect(matchRoute(api, updates, classHref("Part"))).toEqual({
			page: "class",
			item: api.classes["Part"],
		})
		expect(matchRoute(api, updates, enumHref("Material"))).toEqual({
			page: "enum",
			item: api.enums["Material"],
		})
		expect(matchRoute(api, updates, typeHref("Array"))).toEqual({
			page: "type",
			item: api.dataTypes["Array"],
		})
	})

	test("resolves every facet's listing and encoded detail links", () => {
		const paths = new Set(routePaths(api, updates))
		for (const kind of facetKinds) {
			expect(matchRoute(api, updates, facetIndexHref(kind))).toEqual({
				page: "facet-index",
				kind,
			})
			for (const facet of Object.values(api.facets[kind])) {
				const href = facetHref(kind, facet.name)
				expect(paths.has(href)).toBe(true)
				expect(matchRoute(api, updates, href)).toEqual({ page: "facet", facet })
			}
		}
		expect(facetHref("propertyCategory", "Body Parts")).toBe(
			"/index/property-categories/Body%20Parts/",
		)
	})

	test("gives updates a listing and a page for each version", () => {
		const paths = new Set(routePaths(api, updates))
		expect(updatesHref()).toBe("/updates/")
		expect(matchRoute(api, updates, updatesHref())).toEqual({ page: "updates" })
		for (const update of updates) {
			expect(paths.has(updateHref(update.version))).toBe(true)
			expect(matchRoute(api, updates, updateHref(update.version))).toEqual({
				page: "update",
				item: update,
			})
		}
		expect(matchRoute(api, updates, updateHref("0.0.0.0"))).toEqual({
			page: "not-found",
		})
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
			expect(matchRoute(api, updates, path)).toEqual({ page: "not-found" })
		}
	})
})
