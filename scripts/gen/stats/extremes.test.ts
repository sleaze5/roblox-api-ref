import { describe, expect, test } from "vite-plus/test"
import { highest, lowest } from "./extremes.ts"

const info = { id: "test", label: "Test", hint: "A test record." }

function measured(values: Record<string, number>) {
	return Object.entries(values).map(([name, value]) => ({
		ref: { kind: "Class" as const, name },
		value,
	}))
}

describe("records", () => {
	test("keep every item that ties for the highest value", () => {
		expect(highest(info, measured({ A: 2, B: 5, C: 5, D: 1 }))).toEqual({
			...info,
			value: 5,
			items: [
				{ kind: "Class", name: "B" },
				{ kind: "Class", name: "C" },
			],
		})
	})

	test("keep every item that ties for the lowest value", () => {
		const record = lowest(info, measured({ A: 2, B: 1, C: 3, D: 1 }))
		expect(record.value).toBe(1)
		expect(
			record.items.map((item) => (item.kind === "Class" ? item.name : "")),
		).toEqual(["B", "D"])
	})

	test("are empty when nothing is measured", () => {
		expect(highest(info, [])).toEqual({ ...info, value: 0, items: [] })
	})
})
