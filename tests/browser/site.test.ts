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
