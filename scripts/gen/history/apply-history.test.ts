import { describe, expect, test } from "vite-plus/test"
import type { Api, Class } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { applyHistory } from "./apply-history.ts"
import { buildHistory } from "./history-table.ts"

describe("apply history", () => {
	test("marks the addition as not recorded when the class predates the history", () => {
		const part = { name: "Part", members: [] } as unknown as Class
		const api = { classes: { Part: part }, enums: {} } as unknown as Api
		const update = {
			version: "2",
			published: "2-date",
			owners: [
				{
					kind: "Class",
					name: "Part",
					status: "Changed",
					details: [],
					descriptions: [],
					entries: [],
				},
			],
		} as unknown as Update
		applyHistory(api, buildHistory([update]), "1")
		expect(part.history?.added).toBeUndefined()
		expect(part.history?.addedBefore).toBe("1")
		expect(api.historyStart).toBe("1")
	})
})
