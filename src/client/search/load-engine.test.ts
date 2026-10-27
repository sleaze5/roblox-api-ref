import { afterEach, describe, expect, test, vi } from "vite-plus/test"
import type { SearchIndex } from "../../search/index-format.ts"
import { loadEngine } from "./load-engine.ts"

const emptyIndex: SearchIndex = {
	names: [],
	kinds: [],
	parents: [],
	flags: [],
	security: [],
	valueTypes: [],
	parameterTypes: [],
	typeNames: [],
	typeEntries: [],
}

afterEach(() => {
	vi.unstubAllGlobals()
})

describe("loadEngine", () => {
	test("tries again after a failed load, then keeps the engine", async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValueOnce(new Response(null, { status: 503 }))
			.mockResolvedValueOnce(Response.json(emptyIndex))
		vi.stubGlobal("fetch", fetch)

		await expect(loadEngine()).rejects.toThrow("HTTP 503")
		const engine = await loadEngine()
		expect(await loadEngine()).toBe(engine)
		expect(fetch).toHaveBeenCalledTimes(2)
	})
})
