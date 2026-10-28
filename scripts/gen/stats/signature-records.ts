import type { RecordGroup, StatRef } from "../../../src/api/stats-types.ts"
import type { Api, Member } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { itemKey } from "../history/history-table.ts"
import { highest } from "./extremes.ts"
import { currentEntryRef, oldestFirst, removedEntryRef } from "./update-refs.ts"

function signature(member: Member): string {
	switch (member.kind) {
		case "Property":
			return JSON.stringify([member.type])
		case "Event":
			return JSON.stringify([
				member.parameters.map(({ name, type, default: value }) => [
					name,
					type,
					value,
				]),
			])
		case "Method":
		case "Callback":
			return JSON.stringify([
				member.parameters.map(({ name, type, default: value }) => [
					name,
					type,
					value,
				]),
				member.returns,
			])
	}
}

function defaultValue(member: Member): string | undefined {
	return member.kind === "Property" ? member.default : undefined
}

type Tally = { ref: StatRef; signature: number; defaults: number }

export function signatureRecords(api: Api, updates: Update[]): RecordGroup {
	const tallies = new Map<string, Tally>()
	let total = 0
	for (const update of oldestFirst(updates)) {
		for (const owner of update.owners) {
			for (const entry of owner.entries) {
				if (entry.kind !== "Member" || entry.status !== "Changed") {
					continue
				}
				const { before, after } = entry
				if (before === undefined || after === undefined) {
					continue
				}
				const changedSignature = signature(before) !== signature(after)
				const changedDefault = defaultValue(before) !== defaultValue(after)
				if (!changedSignature && !changedDefault) {
					continue
				}
				const key = itemKey(entry.kind, owner.name, entry.name)
				const tally = tallies.get(key) ?? {
					ref:
						currentEntryRef(api, owner, entry) ??
						removedEntryRef(owner, entry, update.version),
					signature: 0,
					defaults: 0,
				}
				tally.signature += Number(changedSignature)
				tally.defaults += Number(changedDefault)
				total += Number(changedSignature)
				tallies.set(key, tally)
			}
		}
	}
	const all = [...tallies.values()]
	return {
		id: "signature-records",
		title: "Signature changes",
		records: [
			{
				id: "signature-changes",
				label: "Signature changes",
				hint: "Times an update changed the type, parameters, or return type of a member.",
				value: total,
				items: [],
			},
			{
				id: "members-with-signature-changes",
				label: "Members with a changed signature",
				hint: "Members whose type, parameters, or return type changed at least once.",
				value: all.filter((tally) => tally.signature > 0).length,
				items: [],
			},
			highest(
				{
					id: "most-signature-changes",
					label: "Most signature changes",
					hint: "The members whose type, parameters, or return type changed in the most updates.",
				},
				all.map((tally) => ({ ref: tally.ref, value: tally.signature })),
			),
			highest(
				{
					id: "most-default-changes",
					label: "Most default value changes",
					hint: "The properties whose default value changed in the most updates.",
				},
				all.map((tally) => ({ ref: tally.ref, value: tally.defaults })),
			),
		],
	}
}
