import type { RecordGroup, StatRef } from "../../../src/api/stats-types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { changeTotal } from "../../../src/api/update-text.ts"
import { highest } from "./extremes.ts"
import { daysBetween } from "./update-refs.ts"

function updateRef(update: Update): StatRef {
	return { kind: "Update", version: update.version, published: update.published }
}

function median(values: number[]): number {
	const sorted = values.toSorted((a, b) => a - b)
	const middle = Math.floor(sorted.length / 2)
	return sorted.length % 2 === 1
		? (sorted[middle] ?? 0)
		: ((sorted[middle - 1] ?? 0) + (sorted[middle] ?? 0)) / 2
}

function oneDecimal(value: number): number {
	return Math.round(value * 10) / 10
}

export function rhythmRecords(updates: Update[]): RecordGroup {
	const byDate = updates.toSorted((a, b) => a.published.localeCompare(b.published))
	const gaps = byDate.slice(1).map((update, index) => ({
		update,
		days: daysBetween(
			byDate[index]?.published ?? update.published,
			update.published,
		),
	}))
	const quiet = updates.filter((update) => changeTotal(update) === 0)
	return {
		id: "rhythm-records",
		title: "Release rhythm",
		records: [
			{
				id: "average-gap",
				label: "Average time between updates",
				hint: "The average time from one update's publish date to the next.",
				value: oneDecimal(
					gaps.reduce((sum, gap) => sum + gap.days, 0) /
						Math.max(gaps.length, 1),
				),
				unit: "days",
				items: [],
			},
			{
				id: "median-gap",
				label: "Median time between updates",
				hint: "Half of the updates came sooner than this after the one before, and half came later.",
				value: oneDecimal(median(gaps.map((gap) => gap.days))),
				unit: "days",
				items: [],
			},
			{
				...highest(
					{
						id: "longest-gap",
						label: "Longest wait for an update",
						hint: "The updates that came the longest after the one before them.",
					},
					gaps.map((gap) => ({
						ref: updateRef(gap.update),
						value: Math.round(gap.days),
					})),
				),
				unit: "days",
			},
			{
				id: "quiet-updates",
				label: "Updates with no API changes",
				hint: "Updates that added, removed, or changed nothing in the API, though they may have changed descriptions.",
				value: quiet.length,
				items: quiet.map(updateRef),
			},
		],
	}
}
