import { compareNames } from "../../../src/api/compare-names.ts"
import { statRefLabel } from "../../../src/api/stat-ref-label.ts"
import type { RecordGroup, Stats } from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { accessStats } from "./access-stats.ts"
import { apiRecords } from "./api-records.ts"
import { comebackStats } from "./comeback-stats.ts"
import { deprecationRecords } from "./deprecation-records.ts"
import { docStats } from "./doc-stats.ts"
import { growthStats } from "./growth-stats.ts"
import { rhythmRecords } from "./rhythm-records.ts"
import { signatureRecords } from "./signature-records.ts"
import { capabilityStats, tagStats } from "./tag-stats.ts"
import { countStats } from "./count-stats.ts"
import { historyRecords } from "./history-records.ts"
import { nameRecords } from "./name-records.ts"
import { monthTrends, updateTrends, yearTrends } from "./trend-stats.ts"

function sortItems(group: RecordGroup): RecordGroup {
	return {
		...group,
		records: group.records.map((record) => ({
			...record,
			items: record.items.toSorted((a, b) =>
				compareNames(statRefLabel(a), statRefLabel(b)),
			),
		})),
	}
}

export function buildStats(api: Api, updates: Update[], historyStart: string): Stats {
	const trends = updateTrends(api, updates)
	const comebacks = comebackStats(api, updates)
	return {
		historyStart,
		counts: countStats(api, updates),
		records: [
			...apiRecords(api),
			nameRecords(api),
			historyRecords(api, updates),
			rhythmRecords(updates),
			deprecationRecords(api, updates),
			comebacks.records,
			signatureRecords(api, updates),
		].map(sortItems),
		updates: trends,
		years: yearTrends(trends),
		months: monthTrends(trends),
		comebacks: comebacks.batches,
		access: accessStats(api, updates),
		tags: tagStats(api, updates),
		capabilities: capabilityStats(api, updates),
		docs: docStats(api, updates),
		growth: growthStats(updates),
	}
}
