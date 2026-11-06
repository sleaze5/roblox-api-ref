import { afterEach, beforeEach, describe, expect, test, vi } from "vite-plus/test"

const loadSearchModule = vi.hoisted(() => vi.fn<() => Promise<unknown>>())

vi.mock("./search-module.ts", () => ({ loadSearchModule }))

class FakeSearch {
	readonly index: Uint8Array

	constructor(index: Uint8Array) {
		this.index = index
	}
}

const searchModule = { Search: FakeSearch }

function indexResponse(): Response {
	return new Response(new Uint8Array([1, 2, 3]))
}

async function freshLoadEngine() {
	vi.resetModules()
	return (await import("./load-engine.ts")).loadEngine
}

beforeEach(() => {
	loadSearchModule.mockReset()
})

afterEach(() => {
	vi.unstubAllGlobals()
})

describe("loadEngine", () => {
	test("fetches the index again after a failed fetch, then keeps the engine", async () => {
		loadSearchModule.mockResolvedValue(searchModule)
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValueOnce(new Response(null, { status: 503 }))
			.mockResolvedValueOnce(indexResponse())
		vi.stubGlobal("fetch", fetch)
		const loadEngine = await freshLoadEngine()

		await expect(loadEngine()).rejects.toThrow("HTTP 503")
		const engine = await loadEngine()
		expect(engine).toBeInstanceOf(FakeSearch)
		expect((engine as unknown as FakeSearch).index).toEqual(
			new Uint8Array([1, 2, 3]),
		)
		expect(await loadEngine()).toBe(engine)
		expect(fetch).toHaveBeenCalledTimes(2)
	})
})
