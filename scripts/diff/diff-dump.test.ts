import { describe, expect, test } from "vite-plus/test"
import type { Dump, DumpClass, DumpMember } from "../gen/dump-types.ts"
import { diffDump } from "./diff-dump.ts"

function property(name: string, overrides: Partial<DumpMember> = {}): DumpMember {
	return {
		MemberType: "Property",
		Name: name,
		Category: "Data",
		ValueType: { Category: "Primitive", Name: "bool" },
		Security: { Read: "None", Write: "None" },
		Serialization: { CanLoad: true, CanSave: true },
		ThreadSafety: "ReadSafe",
		...overrides,
	} as DumpMember
}

function method(name: string): DumpMember {
	return {
		MemberType: "Function",
		Name: name,
		Parameters: [],
		ReturnType: { Category: "Primitive", Name: "null" },
		Security: "None",
		ThreadSafety: "Unsafe",
	}
}

function dumpClass(name: string, members: DumpMember[], tags?: string[]): DumpClass {
	return {
		Name: name,
		Superclass: "Instance",
		MemoryCategory: "Instances",
		Members: members,
		...(tags === undefined ? {} : { Tags: tags }),
	}
}

function dump(classes: DumpClass[], enums: Dump["Enums"] = []): Dump {
	return { Version: 1, Classes: classes, Enums: enums }
}

describe("dump diffs", () => {
	test("finds nothing between identical dumps", () => {
		const same = dump([dumpClass("Part", [property("Anchored")])])
		expect(diffDump(same, structuredClone(same))).toEqual([])
	})

	test("adds a class with its members and removes a class without them", () => {
		const changes = diffDump(
			dump([dumpClass("Old", [property("Gone")])]),
			dump([dumpClass("New", [property("Fresh")])]),
		)
		expect(changes.map((change) => [change.type, change.element])).toEqual([
			["Remove", "Class"],
			["Add", "Class"],
			["Add", "Member"],
		])
		expect(changes[2]).toMatchObject({ class: "New", member: "Fresh" })
		expect(changes[1]?.value).not.toHaveProperty("Members")
	})

	test("records the fields that changed with their previous and new values", () => {
		const [change] = diffDump(
			dump([dumpClass("Part", [property("Size")])]),
			dump([dumpClass("Part", [property("Size", { Tags: ["Deprecated"] })])]),
		)
		expect(change).toEqual({
			element: "Member",
			class: "Part",
			member: "Size",
			type: "Change",
			changes: [{ field: "Tags", before: null, after: ["Deprecated"] }],
			value: property("Size", { Tags: ["Deprecated"] }),
		})
	})

	test("ignores the order of tags", () => {
		expect(
			diffDump(
				dump([dumpClass("Part", [], ["NotCreatable", "Service"])]),
				dump([dumpClass("Part", [], ["Service", "NotCreatable"])]),
			),
		).toEqual([])
	})

	test("replaces a member whose kind changed", () => {
		const changes = diffDump(
			dump([dumpClass("Part", [property("Touch")])]),
			dump([dumpClass("Part", [method("Touch")])]),
		)
		expect(changes.map((change) => change.type)).toEqual(["Remove", "Add"])
	})

	test("diffs enum items", () => {
		const changes = diffDump(
			dump([], [{ Name: "Material", Items: [{ Name: "Grass", Value: 1 }] }]),
			dump(
				[],
				[
					{
						Name: "Material",
						Items: [
							{ Name: "Grass", Value: 2 },
							{ Name: "Sand", Value: 3 },
						],
					},
				],
			),
		)
		expect(changes).toEqual([
			{
				element: "EnumItem",
				enum: "Material",
				item: "Grass",
				type: "Change",
				changes: [{ field: "Value", before: 1, after: 2 }],
				value: { Name: "Grass", Value: 2 },
			},
			{
				element: "EnumItem",
				enum: "Material",
				item: "Sand",
				type: "Add",
				value: { Name: "Sand", Value: 3 },
			},
		])
	})
})
