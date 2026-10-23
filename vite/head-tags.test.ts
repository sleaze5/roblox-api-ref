import { describe, expect, test } from "vite-plus/test"
import type { RenderedPage } from "../src/server/page-module.ts"
import { siteDescription } from "../src/site/brand.ts"
import { siteOrigin } from "../src/site/origin.ts"
import { headTags } from "./head-tags.ts"

function page(overrides: Partial<RenderedPage>): RenderedPage {
	return {
		status: 200,
		title: "Part | Roblox API Reference",
		body: "",
		schemaType: "TechArticle",
		breadcrumbs: [],
		markdown: { href: "/classes/Part.md", content: "# Part\n" },
		modified: "2026-10-19T00:00:00.000Z",
		...overrides,
	}
}

function content(html: string, key: string): string | undefined {
	return new RegExp(`<meta (?:name|property)="${key}" content="([^"]*)">`).exec(
		html,
	)?.[1]
}

describe("headTags", () => {
	const html = headTags(page({}), "/classes/Part/")

	test("gives every social card the page title and the shared description", () => {
		for (const key of ["og:title", "twitter:title"]) {
			expect(content(html, key)).toBe("Part | Roblox API Reference")
		}
		for (const key of ["og:description", "twitter:description"]) {
			expect(content(html, key)).toBe(siteDescription)
		}
		expect(siteDescription).toMatch(/^Unofficial /)
		expect(siteDescription).not.toMatch(/\d+\.\d+/)
	})

	test("shows the large preview image on Open Graph, Twitter, and Discord", () => {
		expect(content(html, "twitter:card")).toBe("summary_large_image")
		for (const key of ["og:image", "og:image:secure_url", "twitter:image"]) {
			expect(content(html, key)).toBe(`${siteOrigin}/og-image.png`)
		}
		expect(content(html, "og:image:width")).toBe("1200")
		expect(content(html, "og:image:height")).toBe("630")
		expect(content(html, "og:image:alt")).toBe("Roblox API Reference")
		expect(content(html, "twitter:image:alt")).toBe("Roblox API Reference")
		expect(content(html, "og:site_name")).toBe("Roblox API Reference")
	})

	test("links the canonical URL, the Markdown version, and search", () => {
		expect(html).toContain(
			`<link rel="canonical" href="${siteOrigin}/classes/Part/">`,
		)
		expect(content(html, "og:url")).toBe(`${siteOrigin}/classes/Part/`)
		expect(html).toContain(
			'<link rel="alternate" type="text/markdown" href="/classes/Part.md">',
		)
		expect(html).toContain('rel="search"')
		expect(content(html, "robots")).toBe("index, follow, max-image-preview:large")
		expect(content(html, "og:type")).toBe("article")
	})

	test("keeps the not found page out of search results", () => {
		const missing = headTags(
			page({
				status: 404,
				schemaType: undefined,
				markdown: { href: "/404.md", content: "# Page not found\n" },
			}),
			"/404",
		)
		expect(content(missing, "robots")).toBe("noindex")
		expect(missing).not.toContain('rel="canonical"')
		expect(missing).not.toContain('rel="alternate"')
		expect(content(missing, "og:type")).toBe("website")
	})
})
