import { describe, expect, test } from "vite-plus/test"
import { loadApi } from "../server/load-api.ts"
import { loadUpdates } from "../server/load-updates.ts"
import { pageMarkdown } from "../server/markdown/page-markdown.ts"
import { matchRoute, routePaths } from "../server/match-route.ts"
import {
	homeMarkdownPath,
	markdownPath,
	notFoundMarkdownPath,
} from "./markdown-path.ts"

describe("markdownPath", () => {
	test("maps a page URL to its Markdown version", () => {
		expect(markdownPath("/")).toBe("/llms.txt")
		expect(markdownPath("/classes/Part/")).toBe("/classes/Part.md")
		expect(markdownPath("/classes/Part")).toBe("/classes/Part.md")
		expect(markdownPath("/index/property-categories/Physics%20Response/")).toBe(
			"/index/property-categories/Physics%20Response.md",
		)
		expect(markdownPath("/updates/0.744.0.7441077/")).toBe(
			"/updates/0.744.0.7441077.md",
		)
	})

	test("leaves files alone", () => {
		for (const path of ["/classes/Part.md", "/llms.txt", "/assets/index.js"]) {
			expect(markdownPath(path)).toBeUndefined()
		}
	})

	test("finds the Markdown version that the build writes for every page", () => {
		const api = loadApi()
		const updates = loadUpdates()
		const href = (path: string) =>
			pageMarkdown(api, updates, matchRoute(api, updates, path)).href
		const pages = routePaths(api, updates).filter((path) => path !== "/404")
		expect(pages.map(markdownPath)).toEqual(pages.map(href))
		expect(href("/")).toBe(homeMarkdownPath)
		expect(href("/404")).toBe(notFoundMarkdownPath)
	})
})
