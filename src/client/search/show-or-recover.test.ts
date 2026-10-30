import { describe, expect, test, vi } from "vite-plus/test"
import {
	showOrRecover,
	type Recovery,
	type SearchDialogModule,
} from "./show-or-recover.ts"

function recovery(load: Recovery["load"], canReload: boolean) {
	return {
		load,
		canReload,
		reload: vi.fn<Recovery["reload"]>(),
		report: vi.fn<Recovery["report"]>(),
	}
}

const failed = () =>
	Promise.reject(new Error("Failed to fetch dynamically imported module"))

describe("showOrRecover", () => {
	test("shows search when its code loads", async () => {
		const showSearch = vi.fn<SearchDialogModule["showSearch"]>()
		const options = recovery(() => Promise.resolve({ showSearch }), true)
		await showOrRecover("Part", options)
		expect(showSearch).toHaveBeenCalledWith("Part")
		expect(options.reload).not.toHaveBeenCalled()
		expect(options.report).not.toHaveBeenCalled()
	})

	test("reloads the page with the query when search code fails to load", async () => {
		const options = recovery(failed, true)
		await showOrRecover("Part", options)
		expect(options.reload).toHaveBeenCalledWith("Part")
		expect(options.report).not.toHaveBeenCalled()
	})

	test("reports the failure when the page cannot reload", async () => {
		const options = recovery(failed, false)
		await showOrRecover(undefined, options)
		expect(options.reload).not.toHaveBeenCalled()
		expect(options.report).toHaveBeenCalledOnce()
	})
})
