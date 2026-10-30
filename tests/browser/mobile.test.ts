import { afterAll, beforeAll, describe, expect, test } from "bun:test"
import { openPage, startPreview, stopPreview, type CheckedPage } from "./preview.ts"

beforeAll(startPreview)
afterAll(stopPreview)

const phone = { viewport: { width: 360, height: 800 }, hasTouch: true, isMobile: true }

function tooltipOpen(page: CheckedPage): Promise<boolean> {
	return page.locator("#tooltip").evaluate((tip) => tip.matches(":popover-open"))
}

describe("phones", () => {
	test.each([360, 402, 768])("the statistics page fits %ipx", async (width) => {
		const page = await openPage("/stats/", {
			...phone,
			viewport: { width, height: 800 },
		})
		const overflow = await page.evaluate(
			() =>
				document.documentElement.scrollWidth -
				document.documentElement.clientWidth,
		)
		expect(overflow).toBe(0)
	})

	test("chart labels stay apart and inside their chart", async () => {
		const page = await openPage("/stats/", phone)
		const problems = await page.evaluate(() =>
			[...document.querySelectorAll("figure.chart")].flatMap((figure) => {
				const bounds = figure.getBoundingClientRect()
				return [
					...figure.querySelectorAll<HTMLElement>("[data-fit], .border-l"),
				]
					.filter((label) => label.offsetParent !== null)
					.map((label) => label.getBoundingClientRect())
					.flatMap((box, index, boxes) => {
						const next = boxes[index + 1]
						const crowded =
							next !== undefined &&
							next.top === box.top &&
							next.left < box.right
						const outside =
							box.left < bounds.left || box.right > bounds.right
						return crowded || outside
							? [`${figure.querySelector("figcaption")?.textContent}`]
							: []
					})
			}),
		)
		expect(problems).toEqual([])
	})

	test("a tap shows the tooltip of a label", async () => {
		const page = await openPage("/stats/", phone)
		const label = page.locator("#counts span[data-tooltip]").first()
		await label.scrollIntoViewIfNeeded()
		await label.tap()
		const tooltip = page.locator("#tooltip")
		expect(await tooltipOpen(page)).toBe(true)
		expect(await tooltip.textContent()).toBe(
			await label.getAttribute("data-tooltip"),
		)
		await label.tap()
		expect(await tooltipOpen(page)).toBe(false)
	})

	test("a tap on a chart column shows its values", async () => {
		const page = await openPage("/stats/", phone)
		const column = page.locator("#release-days g[data-tooltip]").first()
		await column.scrollIntoViewIfNeeded()
		await column.tap()
		const tooltip = page.locator("#tooltip")
		expect(await tooltipOpen(page)).toBe(true)
		expect(await tooltip.textContent()).toBe(
			await column.getAttribute("data-tooltip"),
		)
	})
})
