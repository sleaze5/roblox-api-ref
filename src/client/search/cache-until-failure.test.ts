import { describe, expect, test, vi } from "vite-plus/test"
import { cacheUntilFailure } from "./cache-until-failure.ts"

describe("cacheUntilFailure", () => {
	test("loads again after a failure, then keeps the result", async () => {
		const load = vi
			.fn<() => Promise<string>>()
			.mockRejectedValueOnce(new Error("offline"))
			.mockResolvedValue("engine")
		const cached = cacheUntilFailure(load)

		await expect(cached()).rejects.toThrow("offline")
		expect(await cached()).toBe("engine")
		expect(await cached()).toBe("engine")
		expect(load).toHaveBeenCalledTimes(2)
	})

	test("shares one load between calls made while it runs", async () => {
		const load = vi.fn<() => Promise<object>>(() => Promise.resolve({}))
		const cached = cacheUntilFailure(load)

		const [first, second] = await Promise.all([cached(), cached()])
		expect(first).toBe(second)
		expect(load).toHaveBeenCalledOnce()
	})
})
