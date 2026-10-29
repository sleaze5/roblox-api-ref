import { afterAll, beforeAll, describe, expect, test } from "bun:test"
import {
	openPage,
	origin,
	startPreview,
	stopPreview,
	type CheckedPage,
} from "./preview.ts"

beforeAll(startPreview)
afterAll(stopPreview)

async function openSearch(page: CheckedPage, key = "Control+k"): Promise<void> {
	await page.keyboard.press(key)
	await page.waitForSelector("dialog[open] [data-search-pick]")
}

async function search(page: CheckedPage, query: string): Promise<string[]> {
	await page.fill("dialog input", query)
	await page.waitForSelector("#search-results")
	return page.locator("[role=option]").allInnerTexts()
}

describe("search dialog", () => {
	test("opens with Ctrl+K and shows the syntax help with the newest update", async () => {
		const page = await openPage("/")
		await openSearch(page)
		const examples = await page.locator("[data-search-pick] code").allInnerTexts()
		expect(examples).toContain("Part.")
		expect(examples.at(-1)).toMatch(/^version-[0-9a-f]{16}$/)
		expect(page.problems).toEqual([])
	})

	test("finds members, highlights the match, and moves with the arrow keys", async () => {
		const page = await openPage("/")
		await openSearch(page, "/")
		const options = await search(page, "Humanoid.MoveTo")
		expect(options[0]).toContain("MoveTo")
		expect(
			await page.locator("#search-option-0 .text-fg-bright").first().innerText(),
		).not.toBe("")
		expect(await page.getAttribute("dialog input", "aria-activedescendant")).toBe(
			"search-option-0",
		)
		await page.keyboard.press("ArrowDown")
		expect(await page.getAttribute("[aria-selected=true]", "id")).toBe(
			"search-option-1",
		)
		await page.keyboard.press("ArrowUp")
		await page.keyboard.press("ArrowUp")
		expect(await page.getAttribute("[aria-selected=true]", "id")).toBe(
			`search-option-${options.length - 1}`,
		)
	})

	test("completes with Tab and opens the active result with Enter", async () => {
		const page = await openPage("/")
		await openSearch(page)
		await page.fill("dialog input", "Part.Touc")
		await page.waitForSelector("#search-results")
		expect(await page.locator("[data-search-ghost]").innerText()).toBe("hed")
		await page.keyboard.press("Tab")
		expect(await page.inputValue("dialog input")).toBe("Part.Touched")
		await Promise.all([
			page.waitForURL(/\/classes\/BasePart\/#Event-Touched$/),
			page.keyboard.press("Enter"),
		])
	})

	test("lists updates by hash and removed items", async () => {
		const page = await openPage("/")
		await openSearch(page)
		const upload = (
			await page.locator("[data-search-pick] code").last().innerText()
		).slice(8, 16)
		const updates = await search(page, upload)
		expect(updates[0]).toContain(upload)
		await page.fill("dialog input", "BulkImportService")
		await page.waitForSelector("[role=group][aria-label=Removed]")
		expect(
			await page
				.locator("[aria-label=Removed] [role=option]")
				.first()
				.innerText(),
		).toContain("Removed in")
	})

	test("keeps the query in the URL, and opens from a link with the query", async () => {
		const page = await openPage("/?search=Enum.Material.Plas")
		await page.waitForSelector("#search-results")
		expect(await page.locator("#search-option-0").innerText()).toContain("Plaster")
		await page.keyboard.press("Escape")
		await page.waitForFunction(() => !location.search.includes("search="))
		expect(page.url()).toBe(`${origin}/`)
	})

	test("says when nothing matches", async () => {
		const page = await openPage("/")
		await openSearch(page)
		await page.fill("dialog input", "zzqqxxnothing")
		await page.waitForSelector("text=No results for")
		expect(await page.getAttribute("dialog input", "aria-expanded")).toBe("false")
	})
})
