import { isApiChange } from "../../../src/api/api-change.ts"
import {
	trendKinds,
	type ChangeTally,
	type Tallies,
	type UpdateTrend,
	type PeriodTrend,
} from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type {
	ChangeStatus,
	OwnerChanges,
	Update,
} from "../../../src/api/update-types.ts"
import { entryDeprecation, ownerDeprecation } from "../history/history-table.ts"

const statusKeys: Record<ChangeStatus, keyof ChangeTally> = {
	Added: "added",
	Removed: "removed",
	Changed: "changed",
}

function emptyTallies(): Tallies {
	return Object.fromEntries(
		trendKinds.map((kind) => [
			kind,
			{ added: 0, removed: 0, changed: 0, deprecated: 0 },
		]),
	) as Tallies
}

function tallyOwner(tallies: Tallies, owner: OwnerChanges): void {
	const tally = tallies[owner.kind]
	if (owner.status !== undefined) {
		tally[statusKeys[owner.status]]++
	}
	if (ownerDeprecation(owner) === "set") {
		tally.deprecated++
	}
	for (const entry of owner.entries) {
		const entryTally = tallies[entry.kind]
		if (isApiChange(entry)) {
			entryTally[statusKeys[entry.status]]++
		}
		if (entryDeprecation(entry) === "set") {
			entryTally.deprecated++
		}
	}
}

function updateTallies(update: Update): Tallies {
	const tallies = emptyTallies()
	for (const owner of update.owners) {
		tallyOwner(tallies, owner)
	}
	return tallies
}

export function updateTrends(api: Api, updates: Update[]): UpdateTrend[] {
	let classes = Object.keys(api.classes).length
	let enums = Object.keys(api.enums).length
	const trends: UpdateTrend[] = []
	for (const update of updates) {
		const tallies = updateTallies(update)
		trends.push({
			version: update.version,
			hash: update.hash,
			published: update.published,
			tallies,
			classes,
			enums,
		})
		classes += tallies.Class.removed - tallies.Class.added
		enums += tallies.Enum.removed - tallies.Enum.added
	}
	return trends.reverse()
}

function groupTrends(
	trends: UpdateTrend[],
	periodOf: (trend: UpdateTrend) => string,
): PeriodTrend[] {
	const periods = new Map<string, PeriodTrend>()
	for (const trend of trends) {
		const period = periodOf(trend)
		const total = periods.get(period) ?? {
			period,
			updates: 0,
			tallies: emptyTallies(),
		}
		total.updates++
		for (const kind of trendKinds) {
			for (const key of Object.keys(
				total.tallies[kind],
			) as (keyof ChangeTally)[]) {
				total.tallies[kind][key] += trend.tallies[kind][key]
			}
		}
		periods.set(period, total)
	}
	return [...periods.values()].sort((a, b) => a.period.localeCompare(b.period))
}

export function yearTrends(trends: UpdateTrend[]): PeriodTrend[] {
	return groupTrends(trends, (trend) => trend.published.slice(0, 4))
}

export function monthTrends(trends: UpdateTrend[]): PeriodTrend[] {
	return groupTrends(trends, (trend) => trend.published.slice(5, 7))
}
