import { afterAll, beforeAll, describe, expect, test } from "bun:test"
import { openPage, origin, startPreview, stopPreview } from "./preview.ts"

beforeAll(startPreview)
afterAll(stopPreview)

describe("pages", () => {
	test.each([
		"/",
		"/classes/Part/",
		"/enums/Material/",
		"/index/types/CFrame/",
		"/updates/",
		"/stats/",
		"/404",
	])("%s loads without errors", async (path) => {
		const page = await openPage(path)
		await page.waitForLoadState("networkidle")
		const notFound = /status of 404/
		expect(page.problems.filter((problem) => !notFound.test(problem))).toEqual([])
	})

	test.each(["/classes/", "/enums/", "/updates/", "/index/"])(
		"%s names every landmark once",
		async (path) => {
			const page = await openPage(path)
			const names = await page
				.locator(
					"[role=search], nav, aside, section[aria-labelledby], section[aria-label]",
				)
				.evaluateAll((landmarks) =>
					landmarks.map(
						(landmark) =>
							landmark.getAttribute("aria-label") ??
							document.getElementById(
								landmark.getAttribute("aria-labelledby") ?? "",
							)?.textContent ??
							"",
					),
				)
			expect(new Set(names).size).toBe(names.length)
		},
	)

	test("makes sideways scrolling code reachable from the keyboard", async () => {
		const page = await openPage("/stats/")
		await page.setViewportSize({ width: 360, height: 740 })
		await page.waitForFunction(() =>
			document.querySelector(".stat-table[tabindex]"),
		)
		const flagged = await page
			.locator(".stat-table")
			.evaluateAll(
				(tables) =>
					tables.filter(
						(table) =>
							table.scrollWidth > table.clientWidth !==
							table.hasAttribute("tabindex"),
					).length,
			)
		expect(flagged).toBe(0)
	})

	test("serves the search engine as WebAssembly", async () => {
		const page = await openPage("/")
		const wasm = page.waitForResponse((response) =>
			response.url().endsWith(".wasm"),
		)
		await page.keyboard.press("Control+k")
		const response = await wasm
		expect(response.headers()["content-type"]).toBe("application/wasm")
	})

	test("negotiates Markdown from the Accept header", async () => {
		const response = await fetch(`${origin}/classes/Part/`, {
			headers: { Accept: "text/markdown" },
		})
		expect(response.headers.get("Content-Type")).toBe(
			"text/markdown; charset=utf-8",
		)
		expect(await response.text()).toStartWith("# Part\n")
	})

	test("copies a member link", async () => {
		const page = await openPage("/classes/Part/")
		await page.context().grantPermissions(["clipboard-read", "clipboard-write"])
		const card = page.locator("li.member-card[id]").first()
		const id = await card.getAttribute("id")
		await card.hover()
		await card.locator("[data-copy-link]").click()
		const copied = await page.evaluate(() => navigator.clipboard.readText())
		expect(copied).toBe(`${origin}/classes/Part/#${id}`)
	})
})
