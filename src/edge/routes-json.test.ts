import { readFileSync } from "node:fs"
import { describe, expect, test } from "vite-plus/test"

type RoutesJson = {
	version: number
	include: string[]
	exclude: string[]
}

const routes = JSON.parse(
	readFileSync(new URL("../../public/_routes.json", import.meta.url), "utf8"),
) as RoutesJson

function matches(rule: string, path: string): boolean {
	const pattern = rule.replaceAll(/[.+?^${}()|[\]\\]/g, "\\$&").replaceAll("*", ".*")
	return new RegExp(`^${pattern}$`).test(path)
}

describe("_routes.json", () => {
	test("follows the Cloudflare Pages routing limits", () => {
		const rules = [...routes.include, ...routes.exclude]
		expect(routes.version).toBe(1)
		expect(routes.include.length).toBeGreaterThan(0)
		expect(rules.length).toBeLessThanOrEqual(100)
		for (const rule of rules) {
			expect(rule.length).toBeLessThanOrEqual(100)
			expect(rule).toMatch(/^\//)
		}
	})

	test("runs the negotiation Function on every page", () => {
		const pages = (
			JSON.parse(
				readFileSync(
					new URL("../generated/pages.json", import.meta.url),
					"utf8",
				),
			) as [string, string][]
		)
			.map(([path]) => path)
			.filter((path) => path !== "/404")
		for (const path of pages) {
			expect(routes.include.some((rule) => matches(rule, path))).toBe(true)
			expect(routes.exclude.filter((rule) => matches(rule, path))).toEqual([])
		}
	})
})
