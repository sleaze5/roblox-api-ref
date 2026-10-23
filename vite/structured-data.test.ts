import { describe, expect, test } from "vite-plus/test"
import type { RenderedPage, SchemaType } from "../src/server/page-module.ts"
import { siteOrigin } from "../src/site/origin.ts"
import { structuredData } from "./structured-data.ts"

function page(schemaType: SchemaType, name = "Part"): RenderedPage {
	return {
		status: 200,
		title: "Page",
		body: "",
		schemaType,
		breadcrumbs: [{ name, href: "/" }],
		markdown: { href: "/classes/Part.md", content: "# Part\n" },
		modified: "2026-10-19T00:00:00.000Z",
	}
}

function parse(html: string): Record<string, unknown> {
	const json = /<script type="application\/ld\+json">(.*)<\/script>/.exec(html)?.[1]
	return JSON.parse(json ?? "null") as Record<string, unknown>
}

function graphNode(
	data: Record<string, unknown>,
	type: string,
): Record<string, unknown> {
	const nodes = data["@graph"] as Record<string, unknown>[]
	const node = nodes.find((item) => item["@type"] === type)
	if (node === undefined) {
		throw new Error(`No ${type} node`)
	}
	return node
}

describe("structuredData", () => {
	test("publishes the Organization on the home page", () => {
		const data = parse(structuredData(page("WebSite"), `${siteOrigin}/`))
		expect(data["@context"]).toBe("https://schema.org")
		expect(graphNode(data, "WebSite")).toMatchObject({
			url: `${siteOrigin}/`,
			publisher: { "@id": `${siteOrigin}/#organization` },
		})
		expect(graphNode(data, "Organization")).toMatchObject({
			"@id": `${siteOrigin}/#organization`,
			"name": "Roblox API Reference",
			"url": `${siteOrigin}/`,
			"sameAs": ["https://github.com/sleaze5/roblox-api-ref"],
			"contactPoint": {
				"@type": "ContactPoint",
				"contactType": "customer support",
				"url": `${siteOrigin}/contact/`,
			},
		})
	})

	test("links About and Contact pages to the Organization", () => {
		for (const type of ["AboutPage", "ContactPage"] as const) {
			const data = parse(structuredData(page(type), `${siteOrigin}/about/`))
			expect(graphNode(data, type)["mainEntity"]).toEqual({
				"@id": `${siteOrigin}/#organization`,
			})
		}
	})

	test("keeps reference pages to a single node without the Organization", () => {
		const data = parse(
			structuredData(page("TechArticle"), `${siteOrigin}/classes/Part/`),
		)
		expect(data["@type"]).toBe("TechArticle")
		expect(data).not.toHaveProperty("@graph")
	})

	test("escapes < so a name cannot close the script element", () => {
		const html = structuredData(
			page("TechArticle", "a </script> b"),
			`${siteOrigin}/`,
		)
		expect(html).not.toContain("a </script>")
		expect(parse(html)["headline"]).toBe("a </script> b")
	})
})
