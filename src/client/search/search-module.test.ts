import { beforeEach, describe, expect, test, vi } from "vite-plus/test"

const init = vi.hoisted(() => vi.fn<() => Promise<unknown>>())

vi.mock("../../generated/search.js", () => ({
	default: init,
	Matcher: "Matcher",
	Search: "Search",
}))

beforeEach(() => {
	init.mockReset()
})

describe("loadSearchModule", () => {
	test("starts the WebAssembly module again after it fails, then keeps it", async () => {
		init.mockRejectedValueOnce(new Error("Failed to fetch")).mockResolvedValue({})
		vi.resetModules()
		const { loadSearchModule } = await import("./search-module.ts")

		await expect(loadSearchModule()).rejects.toThrow("Failed to fetch")
		const module = await loadSearchModule()
		expect(module).toEqual({ Matcher: "Matcher", Search: "Search" })
		expect(await loadSearchModule()).toBe(module)
		expect(init).toHaveBeenCalledTimes(2)
	})
})
