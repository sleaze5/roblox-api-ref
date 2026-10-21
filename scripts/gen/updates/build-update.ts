import { compareNames } from "../../../src/api/compare-names.ts"
import type { MemberKind } from "../../../src/api/types.ts"
import {
	changeStatuses,
	type ChangeCounts,
	type ChangeEntry,
	type OwnerChanges,
	type Update,
} from "../../../src/api/update-types.ts"
import type { DumpDiff } from "../../diff/diff-types.ts"
import type { ApiNames } from "../api-docs/doc/api-names.ts"
import { descriptionDetail } from "./description-details.ts"
import { OwnerTable } from "./owner-table.ts"

const memberKinds: MemberKind[] = ["Property", "Method", "Event", "Callback"]

function entryRank(entry: ChangeEntry): number {
	if (entry.kind === "EnumItem") {
		return (entry.after ?? entry.before)?.value ?? 0
	}
	const member = entry.after ?? entry.before
	return member === undefined ? 0 : memberKinds.indexOf(member.kind)
}

function compareEntries(a: ChangeEntry, b: ChangeEntry): number {
	return entryRank(a) - entryRank(b) || compareNames(a.name, b.name)
}

function compareOwners(a: OwnerChanges, b: OwnerChanges): number {
	if (a.kind !== b.kind) {
		return a.kind === "Class" ? -1 : 1
	}
	return compareNames(a.name, b.name)
}

function countChanges(owners: OwnerChanges[]): ChangeCounts {
	const counts = Object.fromEntries(
		changeStatuses.map((status) => [status, 0]),
	) as ChangeCounts
	for (const owner of owners) {
		if (owner.status !== undefined) {
			counts[owner.status]++
		}
		for (const entry of owner.entries) {
			counts[entry.status]++
		}
	}
	return counts
}

function hasChanges(owner: OwnerChanges): boolean {
	return (
		owner.status !== undefined ||
		owner.entries.length > 0 ||
		owner.descriptions.length > 0
	)
}

export function buildUpdate(
	diff: DumpDiff,
	published: string,
	names: ApiNames,
): Update {
	const table = new OwnerTable()
	for (const change of diff.changes) {
		table.add(change)
	}
	const counts = countChanges([...table.owners.values()])
	let descriptionCount = 0
	for (const change of diff.descriptions) {
		const detail = descriptionDetail(names, change)
		if (detail !== undefined) {
			table.addDescription(change, detail)
			descriptionCount++
		}
	}
	const owners = [...table.owners.values()].filter(hasChanges).sort(compareOwners)
	for (const owner of owners) {
		owner.entries.sort(compareEntries)
	}
	return {
		...diff.to,
		published,
		previous: diff.from,
		counts,
		descriptionCount,
		owners,
	}
}
