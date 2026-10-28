import { describe, expect, test } from "vite-plus/test"
import { loadApi } from "../../../src/server/load-api.ts"
import { loadUpdates } from "../../../src/server/load-updates.ts"
import { buildStats } from "./build-stats.ts"

const api = loadApi()
const updates = loadUpdates()
const stats = buildStats(api, updates, api.historyStart ?? api.version)

function record(id: string) {
	const found = stats.records
		.flatMap((group) => group.records)
		.find((item) => item.id === id)
	if (found === undefined) {
		throw new Error(`No record ${id}`)
	}
	return found
}

describe("statistics", () => {
	test("count the current classes, enums, and data types", () => {
		const counts = Object.fromEntries(stats.counts.map((row) => [row.kind, row]))
		expect(counts["Class"]?.current).toBe(Object.keys(api.classes).length)
		expect(counts["Enum"]?.current).toBe(Object.keys(api.enums).length)
		expect(counts["DataType"]?.current).toBe(Object.keys(api.dataTypes).length)
		expect(counts["Class"]?.removed).toBe(api.removed?.classes.length)
	})

	test("count every current member once, by kind", () => {
		const members = Object.values(api.classes).reduce(
			(sum, item) => sum + item.members.length,
			0,
		)
		const counted = stats.counts
			.filter((row) =>
				["Property", "Method", "Event", "Callback"].includes(row.kind),
			)
			.reduce((sum, row) => sum + row.current, 0)
		expect(counted).toBe(members)
	})

	test("name Instance as the class with the most subclasses", () => {
		expect(record("most-subclasses").items).toEqual([
			{ kind: "Class", name: "Instance" },
		])
		expect(record("most-subclasses").value).toBe(
			api.classes["Instance"]?.subclasses.length,
		)
	})

	test("list every class that ties for the longest name", () => {
		const longest = Math.max(...Object.keys(api.classes).map((name) => name.length))
		const names = Object.keys(api.classes).filter((name) => name.length === longest)
		const found = record("longest-class-name")
		expect(found.value).toBe(longest)
		expect(found.items).toHaveLength(names.length)
	})

	test("follow the class count back through every update, oldest first", () => {
		expect(stats.updates).toHaveLength(updates.length)
		expect(stats.updates.at(-1)?.version).toBe(updates[0]?.version)
		expect(stats.updates.at(-1)?.classes).toBe(Object.keys(api.classes).length)
		const expected = stats.updates.slice(1).map((trend, index) => {
			const previous = stats.updates[index]?.classes ?? 0
			return previous + trend.tallies.Class.added - trend.tallies.Class.removed
		})
		expect(stats.updates.slice(1).map((trend) => trend.classes)).toEqual(expected)
	})

	test("sum the updates of each year", () => {
		expect(stats.years.reduce((sum, year) => sum + year.updates, 0)).toBe(
			updates.length,
		)
		const added = stats.updates.reduce(
			(sum, trend) => sum + trend.tallies.Member.added,
			0,
		)
		expect(
			stats.years.reduce((sum, year) => sum + year.tallies.Member.added, 0),
		).toBe(added)
	})
})
