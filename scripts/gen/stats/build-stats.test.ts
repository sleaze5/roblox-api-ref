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

	test("sum the updates of each year and of each month", () => {
		const added = stats.updates.reduce(
			(sum, trend) => sum + trend.tallies.Member.added,
			0,
		)
		for (const periods of [stats.years, stats.months]) {
			expect(periods.reduce((sum, period) => sum + period.updates, 0)).toBe(
				updates.length,
			)
			expect(
				periods.reduce((sum, period) => sum + period.tallies.Member.added, 0),
			).toBe(added)
		}
	})

	test("group months across years, from January to December", () => {
		expect(stats.months.map((month) => month.period)).toEqual(
			stats.months.map((month) => month.period).toSorted(),
		)
		expect(stats.months.length).toBeLessThanOrEqual(12)
	})

	test("count every comeback once, in a batch of updates", () => {
		expect(stats.comebacks.reduce((sum, batch) => sum + batch.count, 0)).toBe(
			record("items-added-back").value,
		)
	})

	test("split each year's added members by whether scripts can use them", () => {
		for (const year of stats.access.years) {
			const trend = stats.years.find((item) => item.period === year.period)
			expect(year.addedOpen + year.addedRestricted).toBe(
				trend?.tallies.Member.added,
			)
		}
	})

	test("count each tag as many times as the Index lists its uses", () => {
		for (const [name, facet] of Object.entries(api.facets.tag)) {
			expect(stats.tags.find((row) => row.name === name)?.current).toBe(
				facet.uses.length,
			)
		}
	})

	test("measure description coverage against every current class and enum", () => {
		const coverage = Object.fromEntries(
			stats.docs.coverage.map((row) => [row.kind, row.total]),
		)
		expect(coverage["Class"]).toBe(Object.keys(api.classes).length)
		expect(coverage["Enum"]).toBe(Object.keys(api.enums).length)
	})

	test("list only classes that grew, largest first", () => {
		for (const year of stats.growth) {
			const added = year.classes.map((item) => item.added)
			expect(added.every((value) => value > 0)).toBe(true)
			expect(added).toEqual(added.toSorted((a, b) => b - a))
		}
	})

	test("put every current member in exactly one access group", () => {
		const members = Object.values(api.classes).reduce(
			(sum, item) => sum + item.members.length,
			0,
		)
		const grouped = stats.access.tiers.reduce(
			(sum, row) =>
				sum +
				row.kinds.Property +
				row.kinds.Method +
				row.kinds.Event +
				row.kinds.Callback,
			0,
		)
		expect(grouped).toBe(members)
		expect(stats.access.tiers[0]?.tier).toBe("game")
		expect(
			Object.values(stats.access.tiers[0]?.kinds ?? {}).reduce(
				(a, b) => a + b,
				0,
			),
		).toBe(stats.access.now.scriptable)
	})
})
