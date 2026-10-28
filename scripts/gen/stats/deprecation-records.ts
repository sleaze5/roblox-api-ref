import type { HistoryEvent, ItemHistory } from "../../../src/api/item-history.ts"
import type { RecordGroup, StatRef } from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { entryDeprecation, itemKey } from "../history/history-table.ts"
import { highest } from "./extremes.ts"
import { daysBetween, oldestFirst, removedEntryRef } from "./update-refs.ts"

type Removal = {
	ref: StatRef
	deprecated: boolean
	scriptable: boolean
	days?: number
}

const deprecatedTag = "Deprecated"

function removals(updates: Update[]): Removal[] {
	const deprecatedAt = new Map<string, HistoryEvent>()
	const removed = new Map<string, Removal>()
	for (const update of oldestFirst(updates)) {
		for (const owner of update.owners) {
			for (const entry of owner.entries) {
				const key = itemKey(entry.kind, owner.name, entry.name)
				if (entry.status === "Added") {
					removed.delete(key)
				}
				if (entry.status === "Removed") {
					const since = deprecatedAt.get(key)
					removed.set(key, {
						ref: removedEntryRef(owner, entry, update.version),
						deprecated: entry.before?.tags.includes(deprecatedTag) ?? false,
						scriptable:
							entry.kind === "EnumItem" ||
							(entry.before !== undefined &&
								entry.before.security === "None"),
						...(since === undefined
							? {}
							: {
									days: Math.round(
										daysBetween(since.published, update.published),
									),
								}),
					})
				}
				const change = entryDeprecation(entry)
				if (change === "set") {
					deprecatedAt.set(key, {
						version: update.version,
						published: update.published,
					})
				} else if (change === "clear") {
					deprecatedAt.delete(key)
				}
			}
		}
	}
	return [...removed.values()]
}

type Deprecated = { ref: StatRef; history?: ItemHistory; tags: string[] }

function deprecatedItem(
	ref: StatRef,
	item: { history?: ItemHistory; tags: string[] },
): Deprecated[] {
	return item.tags.includes(deprecatedTag)
		? [
				item.history === undefined
					? { ref, tags: item.tags }
					: { ref, history: item.history, tags: item.tags },
			]
		: []
}

function currentItems(api: Api): Deprecated[] {
	const items: Deprecated[] = []
	for (const item of Object.values(api.classes)) {
		items.push(...deprecatedItem({ kind: "Class", name: item.name }, item))
		for (const member of item.members) {
			items.push(
				...deprecatedItem(
					{ kind: member.kind, owner: item.name, name: member.name },
					member,
				),
			)
		}
	}
	for (const item of Object.values(api.enums)) {
		items.push(...deprecatedItem({ kind: "Enum", name: item.name }, item))
		for (const enumItem of item.items) {
			items.push(
				...deprecatedItem(
					{ kind: "EnumItem", owner: item.name, name: enumItem.name },
					enumItem,
				),
			)
		}
	}
	return items
}

function median(values: number[]): number {
	const sorted = values.toSorted((a, b) => a - b)
	return sorted[Math.floor((sorted.length - 1) / 2)] ?? 0
}

export function deprecationRecords(api: Api, updates: Update[]): RecordGroup {
	const all = removals(updates)
	const deprecated = all.filter((removal) => removal.deprecated)
	const timed = deprecated.flatMap((removal) =>
		removal.days === undefined ? [] : [{ ref: removal.ref, value: removal.days }],
	)
	const current = currentItems(api)
	const dated = current.flatMap((item) =>
		item.history?.deprecated === undefined
			? []
			: [
					{
						ref: item.ref,
						value: Math.round(
							daysBetween(
								item.history.deprecated.published,
								api.published,
							),
						),
					},
				],
	)
	return {
		id: "deprecation-records",
		title: "Deprecations",
		records: [
			{
				id: "removed-deprecated",
				label: "Removed after deprecation",
				hint: "Members and enum items that were deprecated when an update removed them.",
				value: deprecated.length,
				items: [],
			},
			{
				id: "removed-undeprecated",
				label: "Removed without deprecation",
				hint: "Members and enum items that an update removed without deprecating them first.",
				value: all.length - deprecated.length,
				items: [],
			},
			{
				id: "removed-undeprecated-scriptable",
				label: "Removed without deprecation, usable by scripts",
				hint: "Of the items removed without deprecation, the enum items and the members that game scripts could use.",
				value: all.filter(
					(removal) => !removal.deprecated && removal.scriptable,
				).length,
				items: [],
			},
			{
				id: "median-deprecated-time",
				label: "Median time deprecated before removal",
				hint: "For removed members and enum items whose deprecation is recorded, the middle time between deprecation and removal.",
				value: median(timed.map((entry) => entry.value)),
				unit: "days",
				items: [],
			},
			{
				...highest(
					{
						id: "longest-deprecated-removed",
						label: "Longest deprecated before removal",
						hint: "The removed members and enum items that stayed deprecated the longest.",
					},
					timed,
				),
				unit: "days",
			},
			{
				...highest(
					{
						id: "longest-deprecated-current",
						label: "Longest deprecated, still present",
						hint: "The deprecated items that still exist and were deprecated the longest ago, counting only deprecations the history records.",
					},
					dated,
				),
				unit: "days",
			},
			{
				id: "deprecated-untracked",
				label: "Deprecated before tracking, still present",
				hint: "Deprecated classes, members, enums, and enum items that were already deprecated in the oldest tracked version.",
				value: current.length - dated.length,
				items: [],
			},
		],
	}
}
