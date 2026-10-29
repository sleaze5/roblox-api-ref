import { readFileSync } from "node:fs"
import { describe, expect, test } from "vite-plus/test"
import {
	homeMarkdownPath,
	markdownPath,
	notFoundMarkdownPath,
} from "./markdown-path.ts"

/** Every page path and its Markdown path, as `cargo rapi gen` writes them. */
const pages = JSON.parse(
	readFileSync(new URL("../generated/pages.json", import.meta.url), "utf8"),
) as [path: string, markdown: string][]

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
		const routes = pages.filter(([path]) => path !== "/404")
		expect(routes.length).toBeGreaterThan(1000)
		expect(routes.map(([path]) => markdownPath(path))).toEqual(
			routes.map(([, markdown]) => markdown),
		)
		expect(new Map(pages).get("/")).toBe(homeMarkdownPath)
		expect(new Map(pages).get("/404")).toBe(notFoundMarkdownPath)
	})
})
