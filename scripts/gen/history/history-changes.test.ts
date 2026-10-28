import { describe, expect, test } from "vite-plus/test"
import type { Property } from "../../../src/api/types.ts"
import type { ChangeEntry, OwnerChanges } from "../../../src/api/update-types.ts"
import { entryChanges, ownerChanges } from "./history-changes.ts"

function property(fields: Partial<Property>): Property {
	return {
		kind: "Property",
		name: "Size",
		type: { category: "Primitive", name: "int", optional: false },
		category: "Data",
		default: "1",
		tags: [],
		security: "None",
		writeSecurity: "None",
		threadSafety: "ReadSafe",
		capabilities: { read: [], write: [] },
		canLoad: true,
		canSave: true,
		simulationAccess: false,
		...fields,
	}
}

function changed(entry: Partial<ChangeEntry>): ChangeEntry {
	return {
		kind: "Member",
		status: "Changed",
		owner: "Part",
		name: "Size",
		details: [],
		descriptions: [],
		before: property({}),
		after: property({}),
		...entry,
	} as ChangeEntry
}

describe("history changes", () => {
	test("name the signature parts that changed", () => {
		expect(
			entryChanges(
				changed({
					after: property({
						default: "2",
						type: { category: "Primitive", name: "float", optional: false },
					}),
				}),
			),
		).toEqual({ fields: ["type", "default value"], tagsAdded: 0, tagsRemoved: 0 })
	})

	test("group read and write security, and count tags apart from Deprecated", () => {
		expect(
			entryChanges(
				changed({
					details: [
						{ kind: "value", label: "Read security" },
						{ kind: "value", label: "Write security" },
						{
							kind: "tags",
							label: "Added tags",
							tags: ["Deprecated", "Hidden"],
						},
					],
				}),
			),
		).toEqual({
			fields: ["security"],
			tagsAdded: 1,
			tagsRemoved: 0,
			deprecation: "set",
		})
	})

	test("leave added members and description-only changes without a summary", () => {
		expect(entryChanges(changed({ status: "Added" }))).toBeUndefined()
		expect(entryChanges(changed({}))).toBeUndefined()
	})

	test("summarize the class's own details, but not when it is added", () => {
		const owner: OwnerChanges = {
			kind: "Class",
			name: "Part",
			status: "Changed",
			details: [{ kind: "value", label: "Memory category" }],
			descriptions: [],
			entries: [],
		}
		expect(ownerChanges(owner)).toEqual({
			fields: ["memory category"],
			tagsAdded: 0,
			tagsRemoved: 0,
		})
		expect(ownerChanges({ ...owner, status: "Added" })).toBeUndefined()
	})

	test("roll up what changed on the class's members", () => {
		const owner: OwnerChanges = {
			kind: "Class",
			name: "Part",
			details: [],
			descriptions: [],
			entries: [
				changed({ details: [{ kind: "value", label: "Security" }] }),
				changed({
					details: [
						{ kind: "value", label: "Read security" },
						{
							kind: "tags",
							label: "Added tags",
							tags: ["Deprecated", "Yields"],
						},
					],
				}),
			],
		}
		expect(ownerChanges(owner)).toEqual({
			fields: ["security"],
			tagsAdded: 1,
			tagsRemoved: 0,
			deprecated: 1,
		})
	})
})
