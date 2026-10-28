import {
	countKinds,
	type CountKind,
	type CountRow,
} from "../../../src/api/stats-types.ts"
import type { Api, ApiType, MemberKind } from "../../../src/api/types.ts"
import type { Update } from "../../../src/api/update-types.ts"
import { memberTypes } from "./member-types.ts"

const memberKinds: MemberKind[] = ["Property", "Method", "Event", "Callback"]
const dataTypeCategories = new Set<ApiType["category"]>([
	"Primitive",
	"DataType",
	"Group",
])

function removedDataTypes(api: Api, updates: Update[]): number {
	const names = new Set<string>()
	for (const update of updates) {
		for (const owner of update.owners) {
			for (const entry of owner.entries) {
				if (entry.kind !== "Member") {
					continue
				}
				for (const member of [entry.before, entry.after]) {
					for (const type of member === undefined
						? []
						: memberTypes(member)) {
						if (
							dataTypeCategories.has(type.category) &&
							!Object.hasOwn(api.dataTypes, type.name)
						) {
							names.add(type.name)
						}
					}
				}
			}
		}
	}
	return names.size
}

export function countStats(api: Api, updates: Update[]): CountRow[] {
	const classes = Object.values(api.classes)
	const enums = Object.values(api.enums)
	const members = classes.flatMap((item) => item.members)
	const removedMembers = classes.flatMap((item) =>
		(item.removedMembers ?? []).map((removed) => removed.value),
	)
	const rows: Record<CountKind, Omit<CountRow, "kind">> = {
		Class: { current: classes.length, removed: api.removed?.classes.length ?? 0 },
		Property: { current: 0, removed: 0 },
		Method: { current: 0, removed: 0 },
		Event: { current: 0, removed: 0 },
		Callback: { current: 0, removed: 0 },
		Enum: { current: enums.length, removed: api.removed?.enums.length ?? 0 },
		EnumItem: {
			current: enums.reduce((sum, item) => sum + item.items.length, 0),
			removed: enums.reduce(
				(sum, item) => sum + (item.removedItems?.length ?? 0),
				0,
			),
		},
		DataType: {
			current: Object.keys(api.dataTypes).length,
			removed: removedDataTypes(api, updates),
		},
	}
	for (const kind of memberKinds) {
		rows[kind].current = members.filter((member) => member.kind === kind).length
		rows[kind].removed = removedMembers.filter(
			(member) => member.kind === kind,
		).length
	}
	return countKinds.map((kind) => ({
		kind,
		current: rows[kind].current,
		removed: rows[kind].removed,
	}))
}
