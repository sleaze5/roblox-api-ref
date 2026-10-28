import { accessTier, accessTiers, type TierRow } from "../../../src/api/access-tiers.ts"
import type { AccessNow, AccessPeriod } from "../../../src/api/stats-types.ts"
import type { Api, Member } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { oldestFirst } from "./update-refs.ts"

function scriptable(member: Member): boolean {
	return accessTier(member) === "game"
}

function threadSafe(member: Member): boolean {
	return member.threadSafety !== "Unsafe"
}

function emptyPeriod(period: string): AccessPeriod {
	return {
		period,
		addedOpen: 0,
		addedRestricted: 0,
		opened: 0,
		restricted: 0,
		madeSafe: 0,
		madeUnsafe: 0,
	}
}

function tallyChange(period: AccessPeriod, before: Member, after: Member): void {
	if (scriptable(before) !== scriptable(after)) {
		period[scriptable(after) ? "opened" : "restricted"]++
	}
	if (threadSafe(before) !== threadSafe(after)) {
		period[threadSafe(after) ? "madeSafe" : "madeUnsafe"]++
	}
}

export function accessStats(
	api: Api,
	updates: Update[],
): { now: AccessNow; tiers: TierRow[]; years: AccessPeriod[] } {
	const years = new Map<string, AccessPeriod>()
	for (const update of oldestFirst(updates)) {
		const year = update.published.slice(0, 4)
		const period = years.get(year) ?? emptyPeriod(year)
		for (const owner of update.owners) {
			for (const entry of owner.entries) {
				if (entry.kind !== "Member") {
					continue
				}
				if (entry.status === "Added" && entry.after !== undefined) {
					period[scriptable(entry.after) ? "addedOpen" : "addedRestricted"]++
				} else if (
					entry.status === "Changed" &&
					entry.before !== undefined &&
					entry.after !== undefined
				) {
					tallyChange(period, entry.before, entry.after)
				}
			}
		}
		years.set(year, period)
	}
	const members = Object.values(api.classes).flatMap((item) => item.members)
	const tiers: TierRow[] = accessTiers.map((tier) => ({
		tier,
		kinds: { Property: 0, Method: 0, Event: 0, Callback: 0 },
	}))
	for (const member of members) {
		const row = tiers.find((item) => item.tier === accessTier(member))
		if (row !== undefined) {
			row.kinds[member.kind]++
		}
	}
	return {
		now: {
			members: members.length,
			scriptable: members.filter(scriptable).length,
			threadSafe: members.filter(threadSafe).length,
		},
		tiers,
		years: [...years.values()],
	}
}
