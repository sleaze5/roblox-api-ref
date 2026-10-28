import type { HistoryEntry, ItemHistory } from "../../../src/api/item-history.ts"
import type { RecordGroup, StatRef } from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { changeTotal } from "../../../src/api/update-text.ts"
import { highest, type Measured } from "./extremes.ts"

function apiChanges(history: ItemHistory | undefined): number {
	return (history?.entries ?? []).filter(
		(entry: HistoryEntry) =>
			entry.members !== undefined ||
			entry.kinds.some((kind) => kind === "changed" || kind === "removed"),
	).length
}

function measured(
	items: { ref: StatRef; history: ItemHistory | undefined }[],
	count: (history: ItemHistory | undefined) => number,
): Measured[] {
	return items.map((item) => ({ ref: item.ref, value: count(item.history) }))
}

export function historyRecords(api: Api, updates: Update[]): RecordGroup {
	const classes = Object.values(api.classes).map((item) => ({
		ref: { kind: "Class", name: item.name } as StatRef,
		history: item.history,
	}))
	const members = Object.values(api.classes).flatMap((item) =>
		item.members.map((member) => ({
			ref: { kind: member.kind, owner: item.name, name: member.name } as StatRef,
			history: member.history,
		})),
	)
	const enums = Object.values(api.enums).map((item) => ({
		ref: { kind: "Enum", name: item.name } as StatRef,
		history: item.history,
	}))
	return {
		id: "history-records",
		title: "History",
		records: [
			highest(
				{
					id: "largest-update",
					label: "Largest update",
					hint: "The updates that added, removed, or changed the most classes, members, enums, and enum items.",
				},
				updates.map((update) => ({
					ref: {
						kind: "Update",
						version: update.version,
						published: update.published,
					},
					value: changeTotal(update),
				})),
			),
			highest(
				{
					id: "most-changed-class",
					label: "Most-changed class",
					hint: "The classes that the most updates changed, counting changes to their members.",
				},
				measured(classes, apiChanges),
			),
			highest(
				{
					id: "most-changed-member",
					label: "Most-changed member",
					hint: "The members that the most updates changed.",
				},
				measured(members, apiChanges),
			),
			highest(
				{
					id: "most-changed-enum",
					label: "Most-changed enum",
					hint: "The enums that the most updates changed, counting changes to their items.",
				},
				measured(enums, apiChanges),
			),
		],
	}
}
