import { compareNames } from "../../../src/api/compare-names.ts"
import type { MemberKind } from "../../../src/api/types.ts"
import {
	changeStatuses,
	type ChangeCounts,
	type ChangeDetail,
	type ChangeEntry,
	type ChangeStatus,
	type OwnerChanges,
	type OwnerKind,
	type Update,
} from "../../../src/api/update-types.ts"
import type {
	ClassChange,
	DumpChange,
	DumpDiff,
	EnumChange,
	EnumItemChange,
	FieldChange,
	MemberChange,
} from "../../diff/diff-types.ts"
import { normalizeEnumItem } from "../normalize-enum-item.ts"
import { normalizeMember } from "../normalize-member.ts"
import { changeDetails, stringTags } from "./field-details.ts"

const memberKinds: MemberKind[] = ["Property", "Method", "Event", "Callback"]
const rootSuperclass = "<<<ROOT>>>"

const statuses = {
	Add: "Added",
	Remove: "Removed",
	Change: "Changed",
} as const satisfies Record<DumpChange["type"], ChangeStatus>

function restore<T extends object>(value: T, changes: FieldChange[]): T {
	const before = { ...value } as Record<string, unknown>
	for (const change of changes) {
		if (change.before === null) {
			delete before[change.field]
		} else {
			before[change.field] = change.before
		}
	}
	return before as T
}

function entryOf<Raw extends object, Value>(
	change: { type: DumpChange["type"]; value: Raw; changes?: FieldChange[] },
	normalize: (raw: Raw) => Value,
):
	| { status: ChangeStatus; details: ChangeDetail[]; before?: Value; after?: Value }
	| undefined {
	const status = statuses[change.type]
	if (change.type === "Add") {
		return { status, details: [], after: normalize(change.value) }
	}
	if (change.type === "Remove") {
		return { status, details: [], before: normalize(change.value) }
	}
	const fields = change.changes ?? []
	const before = normalize(restore(change.value, fields))
	const after = normalize(change.value)
	const details = changeDetails(fields)
	if (details.length === 0 && JSON.stringify(before) === JSON.stringify(after)) {
		return undefined
	}
	return { status, details, before, after }
}

function memberEntry(change: MemberChange): ChangeEntry | undefined {
	const entry = entryOf(change, normalizeMember)
	return (
		entry && { kind: "Member", owner: change.class, name: change.member, ...entry }
	)
}

function enumItemEntry(change: EnumItemChange): ChangeEntry | undefined {
	const entry = entryOf(change, normalizeEnumItem)
	return (
		entry && { kind: "EnumItem", owner: change.enum, name: change.item, ...entry }
	)
}

function addedDetails(change: ClassChange | EnumChange): ChangeDetail[] {
	const superclass =
		change.element === "Class" ? change.value.Superclass : rootSuperclass
	const details: ChangeDetail[] = []
	if (superclass !== rootSuperclass) {
		details.push({
			kind: "value",
			label: "Superclass",
			after: { text: superclass, tone: "type" },
		})
	}
	const tags = stringTags(change.value.Tags).sort()
	if (tags.length > 0) {
		details.push({ kind: "tags", label: "Tags", tags })
	}
	return details
}

function ownerDetails(change: ClassChange | EnumChange): ChangeDetail[] {
	switch (change.type) {
		case "Add":
			return addedDetails(change)
		case "Remove":
			return []
		case "Change":
			return changeDetails(change.changes)
	}
}

class OwnerTable {
	readonly owners = new Map<string, OwnerChanges>()

	get(kind: OwnerKind, name: string): OwnerChanges {
		const key = `${kind}:${name}`
		let owner = this.owners.get(key)
		if (owner === undefined) {
			owner = { kind, name, details: [], entries: [] }
			this.owners.set(key, owner)
		}
		return owner
	}

	add(change: DumpChange): void {
		switch (change.element) {
			case "Class":
			case "Enum": {
				const name = change.element === "Class" ? change.class : change.enum
				const owner = this.get(change.element, name)
				owner.status = statuses[change.type]
				owner.details = ownerDetails(change)
				return
			}
			case "Member":
				this.addEntry("Class", memberEntry(change))
				return
			case "EnumItem":
				this.addEntry("Enum", enumItemEntry(change))
		}
	}

	private addEntry(kind: OwnerKind, entry: ChangeEntry | undefined): void {
		if (entry !== undefined) {
			this.get(kind, entry.owner).entries.push(entry)
		}
	}
}

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

export function buildUpdate(diff: DumpDiff, published: string): Update {
	const table = new OwnerTable()
	for (const change of diff.changes) {
		table.add(change)
	}
	const owners = [...table.owners.values()]
		.filter((owner) => owner.status !== undefined || owner.entries.length > 0)
		.sort(compareOwners)
	for (const owner of owners) {
		owner.entries.sort(compareEntries)
	}
	return {
		...diff.to,
		published,
		previous: diff.from,
		counts: countChanges(owners),
		owners,
	}
}
