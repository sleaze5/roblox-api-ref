import {
	type ChangeDetail,
	type ChangeEntry,
	type ChangeStatus,
} from "../../../src/api/update-types.ts"
import type {
	ClassChange,
	DumpChange,
	EnumChange,
	EnumItemChange,
	FieldChange,
	MemberChange,
} from "../../diff/diff-types.ts"
import { normalizeEnumItem } from "../normalize-enum-item.ts"
import { normalizeMember } from "../normalize-member.ts"
import { changeDetails, stringTags } from "./field-details.ts"

const rootSuperclass = "<<<ROOT>>>"

export const statuses = {
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

export function memberEntry(change: MemberChange): ChangeEntry | undefined {
	const entry = entryOf(change, normalizeMember)
	return (
		entry && {
			kind: "Member",
			owner: change.class,
			name: change.member,
			descriptions: [],
			...entry,
		}
	)
}

export function enumItemEntry(change: EnumItemChange): ChangeEntry | undefined {
	const entry = entryOf(change, normalizeEnumItem)
	return (
		entry && {
			kind: "EnumItem",
			owner: change.enum,
			name: change.item,
			descriptions: [],
			...entry,
		}
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

export function ownerDetails(change: ClassChange | EnumChange): ChangeDetail[] {
	switch (change.type) {
		case "Add":
			return addedDetails(change)
		case "Remove":
			return []
		case "Change":
			return changeDetails(change.changes)
	}
}
