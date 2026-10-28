import type {
	ComebackBatch,
	RecordGroup,
	StatRef,
	UpdateEvent,
} from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { itemKey, ownerKey } from "../history/history-table.ts"
import { highest } from "./extremes.ts"
import {
	currentEntryRef,
	currentOwnerRef,
	daysBetween,
	eventOf,
	oldestFirst,
	removedEntryRef,
	removedOwnerRef,
} from "./update-refs.ts"

type Comeback = {
	ref: StatRef
	removed: UpdateEvent
	returned: UpdateEvent
}

type Tracker = {
	removed: Map<string, UpdateEvent>
	comebacks: Comeback[]
}

function track(
	tracker: Tracker,
	key: string,
	status: string | undefined,
	update: Update,
	ref: () => StatRef,
): void {
	const removed = tracker.removed.get(key)
	if (
		status === "Added" &&
		removed !== undefined &&
		removed.version !== update.version
	) {
		tracker.comebacks.push({ ref: ref(), removed, returned: eventOf(update) })
		tracker.removed.delete(key)
	} else if (status === "Removed") {
		tracker.removed.set(key, eventOf(update))
	}
}

function comebacksOf(api: Api, updates: Update[]): Comeback[] {
	const tracker: Tracker = { removed: new Map(), comebacks: [] }
	for (const update of oldestFirst(updates)) {
		for (const owner of update.owners) {
			track(
				tracker,
				ownerKey(owner.kind, owner.name),
				owner.status,
				update,
				() =>
					currentOwnerRef(api, owner) ??
					removedOwnerRef(owner, update.version),
			)
			for (const entry of owner.entries) {
				track(
					tracker,
					itemKey(entry.kind, owner.name, entry.name),
					entry.status,
					update,
					() =>
						currentEntryRef(api, owner, entry) ??
						removedEntryRef(owner, entry, update.version),
				)
			}
		}
	}
	return tracker.comebacks
}

function gone(comeback: Comeback): number {
	return Math.round(
		daysBetween(comeback.removed.published, comeback.returned.published),
	)
}

function batches(comebacks: Comeback[]): ComebackBatch[] {
	const groups = new Map<string, ComebackBatch>()
	for (const comeback of comebacks) {
		const key = `${comeback.removed.version} ${comeback.returned.version}`
		const batch = groups.get(key) ?? {
			removed: comeback.removed,
			returned: comeback.returned,
			count: 0,
			days: gone(comeback),
		}
		batch.count++
		groups.set(key, batch)
	}
	return [...groups.values()].sort(
		(a, b) =>
			b.count - a.count ||
			b.returned.published.localeCompare(a.returned.published),
	)
}

export function comebackStats(
	api: Api,
	updates: Update[],
): { records: RecordGroup; batches: ComebackBatch[] } {
	const comebacks = comebacksOf(api, updates)
	const owners = comebacks.filter(
		(comeback) => comeback.ref.kind === "Class" || comeback.ref.kind === "Enum",
	)
	return {
		records: {
			id: "comeback-records",
			title: "Comebacks",
			records: [
				{
					id: "items-added-back",
					label: "Items added back",
					hint: "Classes, members, enums, and enum items that an update removed and a later update added back.",
					value: comebacks.length,
					items: [],
				},
				{
					id: "owners-added-back",
					label: "Classes and enums added back",
					hint: "Classes and enums that an update removed and a later update added back.",
					value: owners.length,
					items: owners.map((comeback) => comeback.ref),
				},
				{
					...highest(
						{
							id: "longest-gone",
							label: "Longest gone",
							hint: "The items that stayed removed the longest before an update added them back.",
						},
						comebacks.map((comeback) => ({
							ref: comeback.ref,
							value: gone(comeback),
						})),
					),
					unit: "days",
				},
			],
		},
		batches: batches(comebacks),
	}
}
