import { describe, expect, test } from "vite-plus/test"
import type { DumpChange } from "../../diff/diff-types.ts"
import type { DumpMember } from "../dump-types.ts"
import { buildHistory, classKey, memberKey, type DatedDiff } from "./history-table.ts"

const member = { Name: "Size" } as DumpMember

function diff(
	version: string,
	changes: DumpChange[],
	described: string[] = [],
): DatedDiff {
	return {
		published: `${version}-date`,
		diff: {
			from: { version: "", hash: "" },
			to: { version, hash: "" },
			changes,
			descriptions: described.map((name) => ({
				element: "Member",
				class: "Part",
				member: name,
				value: member,
				part: "description",
				before: null,
				after: "Text.",
			})),
		},
	}
}

function versions(history: object | undefined): Record<string, string> {
	return Object.fromEntries(
		Object.entries(history ?? {}).map(([field, event]) => [field, event.version]),
	)
}

const addPart: DumpChange = {
	element: "Class",
	class: "Part",
	type: "Add",
	value: {} as never,
}
const addSize: DumpChange = {
	element: "Member",
	class: "Part",
	member: "Size",
	type: "Add",
	value: member,
}
const changeSize: DumpChange = { ...addSize, type: "Change", changes: [] }
const removeSize: DumpChange = { ...addSize, type: "Remove" }

describe("item history", () => {
	test("does not mark a new class as changed by its own new members", () => {
		const table = buildHistory([diff("1", [addPart, addSize])])
		expect(versions(table.items.get(classKey("Part")))).toEqual({ added: "1" })
		expect(versions(table.items.get(memberKey("Part", "Size")))).toEqual({
			added: "1",
		})
	})

	test("marks a class as changed when its members change later", () => {
		const table = buildHistory([
			diff("1", [addPart, addSize]),
			diff("2", [changeSize], ["Size"]),
		])
		expect(versions(table.items.get(classKey("Part")))).toEqual({
			added: "1",
			changed: "2",
		})
		expect(versions(table.items.get(memberKey("Part", "Size")))).toEqual({
			added: "1",
			changed: "2",
			described: "2",
		})
	})

	test("starts over when an item is removed and added again", () => {
		const table = buildHistory([
			diff("1", [addSize]),
			diff("2", [changeSize]),
			diff("3", [removeSize]),
			diff("4", [addSize]),
		])
		expect(versions(table.items.get(memberKey("Part", "Size")))).toEqual({
			added: "4",
		})
		expect(versions(table.items.get(classKey("Part")))).toEqual({ changed: "4" })
	})
})
