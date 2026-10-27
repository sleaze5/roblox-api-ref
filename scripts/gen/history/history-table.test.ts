import { describe, expect, test } from "vite-plus/test"
import type { ItemHistory } from "../../../src/api/item-history.ts"
import type { Property } from "../../../src/api/types.ts"
import type {
	ChangeEntry,
	OwnerChanges,
	Update,
} from "../../../src/api/update-types.ts"
import { buildHistory, itemKey, ownerKey } from "./history-table.ts"

function property(value: string): Property {
	return {
		kind: "Property",
		name: "Size",
		type: { category: "Primitive", name: "int", optional: false },
		category: "Data",
		default: value,
		tags: [],
		security: "None",
		writeSecurity: "None",
		threadSafety: "ReadSafe",
		capabilities: { read: [], write: [] },
		canLoad: true,
		canSave: true,
		simulationAccess: false,
	}
}

function size(fields: Partial<ChangeEntry>): ChangeEntry {
	return {
		kind: "Member",
		status: "Added",
		owner: "Part",
		name: "Size",
		details: [],
		descriptions: [],
		after: property("1"),
		...fields,
	} as ChangeEntry
}

function part(status: OwnerChanges["status"], entries: ChangeEntry[]): OwnerChanges {
	return {
		kind: "Class",
		name: "Part",
		...(status === undefined ? {} : { status }),
		details: [],
		descriptions: [],
		entries,
	}
}

function update(version: string, owners: OwnerChanges[]): Update {
	return { version, published: `${version}-date`, owners } as unknown as Update
}

function summary(history: ItemHistory | undefined) {
	return {
		added: history?.added?.version,
		changed: history?.changed?.version,
		entries: history?.entries.map((entry) => [
			entry.version,
			entry.kinds,
			entry.members,
		]),
	}
}

const resized = size({ status: "Changed", before: property("1"), after: property("2") })
const described = size({
	status: "Changed",
	before: property("2"),
	after: property("2"),
	descriptions: [{ part: "description", status: "Added" }],
})

describe("item history", () => {
	const table = buildHistory(
		[
			update("1", [part("Added", [size({})])]),
			update("2", [part(undefined, [resized])]),
			update("3", [part(undefined, [described])]),
			update("4", [
				part(undefined, [size({ status: "Removed", before: property("2") })]),
			]),
			update("5", [part(undefined, [size({})])]),
		].reverse(),
	)

	test("lists every change to a member, newest first", () => {
		expect(summary(table.history(itemKey("Member", "Part", "Size")))).toEqual({
			added: "5",
			changed: undefined,
			entries: [
				["5", ["added"], undefined],
				["4", ["removed"], undefined],
				["3", ["described"], undefined],
				["2", ["changed"], undefined],
				["1", ["added"], undefined],
			],
		})
	})

	test("counts member changes on the class, but not members of a new class", () => {
		expect(summary(table.history(ownerKey("Class", "Part")))).toEqual({
			added: "1",
			changed: "5",
			entries: [
				["5", [], { added: 1, removed: 0, changed: 0 }],
				["4", [], { added: 0, removed: 1, changed: 0 }],
				["2", [], { added: 0, removed: 0, changed: 1 }],
				["1", ["added"], undefined],
			],
		})
	})
})
