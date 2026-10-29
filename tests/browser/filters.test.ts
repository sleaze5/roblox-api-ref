import { afterAll, beforeAll, describe, expect, test } from "bun:test"
import { openPage, startPreview, stopPreview, type CheckedPage } from "./preview.ts"

beforeAll(startPreview)
afterAll(stopPreview)

function summary(page: CheckedPage, form: string): Promise<string> {
	return page.locator(`${form} output`).innerText()
}

async function waitForSummary(
	page: CheckedPage,
	form: string,
	pattern: RegExp,
): Promise<string> {
	await page.waitForFunction(
		([selector, source]) =>
			new RegExp(source ?? "").test(
				document.querySelector(`${selector} output`)?.textContent ?? "",
			),
		[form, pattern.source],
	)
	return summary(page, form)
}

describe("member filters", () => {
	const form = "form[data-member-filter]"

	test("filters members by name with the search scoring", async () => {
		const page = await openPage("/classes/Part/")
		expect(await summary(page, form)).toMatch(/^\d+ total$/)
		await page.fill(`${form} input[name=text]`, "touch")
		const filtered = await waitForSummary(page, form, / of /)
		expect(filtered).toMatch(/^\d+ of \d+$/)
		expect(
			await page
				.locator("[data-member]:not([hidden])[data-name=Touched]")
				.count(),
		).toBe(1)
		await page.waitForURL(/\?q=touch$/)
		expect(page.problems).toEqual([])
	})

	test("hides a member kind and inherited members", async () => {
		const page = await openPage("/classes/Part/")
		await page.locator(`${form} label.chip`, { hasText: "Methods" }).click()
		expect(await page.locator("section[data-section=Method]").isHidden()).toBe(true)
		await page.locator(`${form} [popovertarget]`).click()
		await page
			.locator("input[name=inherited]")
			.evaluate((input: HTMLInputElement) => input.click())
		expect(await page.locator("[data-inherited-group]:not([hidden])").count()).toBe(
			0,
		)
	})

	test("applies required tags from the URL", async () => {
		const page = await openPage("/classes/Instance/?tags=deprecated")
		const shown = await page
			.locator("[data-member]:visible")
			.evaluateAll((rows) =>
				rows.map((row) => row.getAttribute("data-tags") ?? ""),
			)
		expect(shown.length).toBeGreaterThan(0)
		expect(shown.every((tags) => tags.split(" ").includes("deprecated"))).toBe(true)
	})
})

describe("list filters", () => {
	const form = "main form[data-list-filter]"

	test("filters a list from the query in the URL", async () => {
		const page = await openPage("/enums/?q=mat")
		const text = await waitForSummary(page, form, / of /)
		expect(text).toMatch(/^\d+ of 6\d\d$/)
		expect(
			await page
				.locator("#enum-list li:not([hidden])[data-name=Material]")
				.count(),
		).toBe(1)
	})

	test("filters the class tree and opens the branches that match", async () => {
		const page = await openPage("/classes/")
		await page.fill(`${form} input`, "SpawnLocation")
		await waitForSummary(page, form, / of /)
		expect(
			await page
				.locator("#class-tree a", { hasText: /^SpawnLocation$/ })
				.isVisible(),
		).toBe(true)
	})

	test("counts the changes of an update with their noun", async () => {
		const page = await openPage("/updates/")
		expect(await summary(page, form)).toMatch(/^\d+ classes and enums$/)
		await page.fill(`${form} input`, "zzqqxx")
		expect(await waitForSummary(page, form, /^0 of/)).toMatch(
			/^0 of \d+ classes and enums$/,
		)
	})
})
