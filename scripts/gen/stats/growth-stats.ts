import { compareNames } from "../../../src/api/compare-names.ts"
import type { GrowthPeriod } from "../../../src/api/stats-types.ts"
import type { ChangeStatus, Update } from "../../../src/api/update-types.ts"

const shown = 5

const netChange: Record<ChangeStatus, number> = { Added: 1, Removed: -1, Changed: 0 }

export function growthStats(updates: Update[]): GrowthPeriod[] {
	const years = new Map<string, Map<string, number>>()
	for (const update of updates.toReversed()) {
		const year = update.published.slice(0, 4)
		const classes = years.get(year) ?? new Map<string, number>()
		for (const owner of update.owners) {
			if (owner.kind !== "Class") {
				continue
			}
			const net = owner.entries.reduce(
				(sum, entry) => sum + netChange[entry.status],
				0,
			)
			classes.set(owner.name, (classes.get(owner.name) ?? 0) + net)
		}
		years.set(year, classes)
	}
	return [...years.entries()].map(([period, classes]) => ({
		period,
		classes: [...classes.entries()]
			.filter(([, added]) => added > 0)
			.map(([name, added]) => ({ name, added }))
			.sort((a, b) => b.added - a.added || compareNames(a.name, b.name))
			.slice(0, shown),
	}))
}
