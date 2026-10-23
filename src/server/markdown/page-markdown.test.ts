import { describe, expect, test } from "vite-plus/test"
import { sitePages } from "../../site/site-pages.ts"
import { loadApi } from "../load-api.ts"
import { loadUpdates } from "../load-updates.ts"
import { matchRoute, routePaths } from "../match-route.ts"
import { llmsTxt } from "./llms-txt.ts"
import { pageMarkdown } from "./page-markdown.ts"
import { sitePageMarkdown } from "./site-page-markdown.ts"
import { absoluteUrl, siteOrigin } from "../../site/origin.ts"

const api = loadApi()
const updates = loadUpdates()

function markdownOf(path: string) {
	return pageMarkdown(api, updates, matchRoute(api, updates, path))
}

describe("page Markdown", () => {
	test("lives at the page URL without the trailing slash, plus .md", () => {
		const pages = routePaths(api, updates).flatMap((path) => {
			const href = markdownOf(path)?.href
			return href === undefined ? [] : [{ path, href }]
		})
		expect(pages.length).toBeGreaterThan(1000)
		expect(pages.map((page) => page.href)).toEqual(
			pages.map((page) =>
				page.path === "/" ? "/llms.txt" : page.path.slice(0, -1) + ".md",
			),
		)
	})

	test("gives the home page llms.txt as its Markdown version", () => {
		expect(markdownOf("/")).toEqual({ href: "/llms.txt", content: llmsTxt(api) })
	})

	test("links to other pages with absolute URLs on the site origin", () => {
		for (const path of ["/classes/", "/enums/", "/index/types/", "/updates/"]) {
			const content = markdownOf(path)?.content ?? ""
			const targets = [...content.matchAll(/\]\(([^)]+)\)/g)].map(
				(match) => match[1],
			)
			expect(targets.length).toBeGreaterThan(0)
			for (const target of targets) {
				expect(new URL(target ?? "").origin).toBe(siteOrigin)
			}
		}
	})

	test("lists the site pages in llms.txt", () => {
		const text = llmsTxt(api)
		for (const href of ["/about.md", "/contact.md", "/privacy.md"]) {
			expect(text).toContain(`(${absoluteUrl(href)})`)
		}
	})
})

describe("site pages", () => {
	test("each have at least 500 characters of text", () => {
		for (const page of Object.values(sitePages)) {
			const text = sitePageMarkdown(page.title, page.content(api))
			expect(
				text.replaceAll(/\]\([^)]*\)|[#[\]`*-]/g, "").length,
			).toBeGreaterThan(500)
		}
	})

	test("render as Markdown under their own paths", () => {
		expect(markdownOf("/about/")).toEqual({
			href: "/about.md",
			content: sitePageMarkdown("About", sitePages.about.content(api)),
		})
		expect(markdownOf("/privacy/")?.content).toMatch(/^# Privacy policy\n/)
	})

	test("do not send readers to Roblox", () => {
		const texts = [
			llmsTxt(api),
			...Object.values(sitePages).map((page) =>
				sitePageMarkdown(page.title, page.content(api)),
			),
		]
		for (const text of texts) {
			expect(text).not.toMatch(/roblox\.com/)
		}
	})
})
