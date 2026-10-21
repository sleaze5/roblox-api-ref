import { describe, expect, test } from "vite-plus/test"
import type { DescriptionChange, DumpDiff } from "../../diff/diff-types.ts"
import type { ApiNames } from "../api-docs/doc/api-names.ts"
import { buildUpdate } from "./build-update.ts"

const versions = {
	from: { version: "0.1.0.1", hash: "aaaaaaaaaaaaaaaa" },
	to: { version: "0.2.0.1", hash: "bbbbbbbbbbbbbbbb" },
}

const names: ApiNames = { classes: {}, enums: {}, dataTypes: {} }

function update(changes: DumpDiff["changes"], descriptions: DescriptionChange[] = []) {
	return buildUpdate(
		{ ...versions, changes, descriptions },
		"2026-10-19T23:04:06.000Z",
		names,
	)
}

describe("updates", () => {
	test("groups changes by class and enum, classes first", () => {
		const result = update([
			{
				element: "EnumItem",
				enum: "Material",
				item: "Sand",
				type: "Add",
				value: { Name: "Sand", Value: 3 },
			},
			{
				element: "Member",
				class: "Part",
				member: "Touch",
				type: "Remove",
				value: {
					MemberType: "Event",
					Name: "Touch",
					Parameters: [],
					Security: "None",
				},
			},
		])
		expect(result.owners.map((owner) => [owner.kind, owner.name])).toEqual([
			["Class", "Part"],
			["Enum", "Material"],
		])
		expect(result.counts).toEqual({ Added: 1, Removed: 1, Changed: 0 })
		expect(result.previous).toEqual(versions.from)
	})

	test("restores the previous value of a changed member and describes its details", () => {
		const result = update([
			{
				element: "Member",
				class: "Part",
				member: "Size",
				type: "Change",
				changes: [
					{ field: "Default", before: "1", after: "2" },
					{ field: "Security", before: "None", after: "PluginSecurity" },
					{ field: "Tags", before: ["Hidden"], after: ["Deprecated"] },
				],
				value: {
					MemberType: "Function",
					Name: "Size",
					Parameters: [],
					ReturnType: { Category: "Primitive", Name: "int" },
					Security: "PluginSecurity",
					Tags: ["Deprecated"],
				},
			},
		])
		const [entry] = result.owners[0]?.entries ?? []
		expect(entry?.before).toMatchObject({ security: "None", tags: ["Hidden"] })
		expect(entry?.after).toMatchObject({
			security: "PluginSecurity",
			tags: ["Deprecated"],
		})
		expect(entry?.details).toEqual([
			{
				kind: "value",
				label: "Security",
				before: { text: "No security", tone: "text" },
				after: { text: "Plugin security", tone: "text" },
			},
			{ kind: "tags", label: "Added tags", tags: ["Deprecated"] },
			{ kind: "tags", label: "Removed tags", tags: ["Hidden"] },
		])
	})

	test("skips changes that the site does not show", () => {
		const result = update([
			{
				element: "Member",
				class: "Part",
				member: "Name",
				type: "Change",
				changes: [
					{
						field: "Default",
						before: "__api_dump_a__",
						after: "__api_dump_b__",
					},
				],
				value: {
					MemberType: "Property",
					Name: "Name",
					Category: "Data",
					Default: "__api_dump_b__",
					ValueType: { Category: "Primitive", Name: "string" },
					Security: { Read: "None", Write: "None" },
					Serialization: { CanLoad: true, CanSave: true },
				},
			},
		])
		expect(result.owners).toEqual([])
		expect(result.counts).toEqual({ Added: 0, Removed: 0, Changed: 0 })
	})

	test("sorts enum items by value", () => {
		const result = update(
			[5, 1, 3].map((value) => ({
				element: "EnumItem",
				enum: "Material",
				item: `Item${value}`,
				type: "Add",
				value: { Name: `Item${value}`, Value: value },
			})),
		)
		expect(result.owners[0]?.entries.map((entry) => entry.name)).toEqual([
			"Item1",
			"Item3",
			"Item5",
		])
	})

	test("adds description changes without counting them as API changes", () => {
		const anchored = {
			MemberType: "Property",
			Name: "Anchored",
			Category: "Behavior",
			ValueType: { Category: "Primitive", Name: "bool" },
			Security: { Read: "None", Write: "None" },
			Serialization: { CanLoad: true, CanSave: true },
		} as const
		const result = update(
			[],
			[
				{
					element: "Member",
					class: "Part",
					member: "Anchored",
					part: "description",
					before: "Old text.",
					after: "New text.",
					value: anchored,
				},
				{
					element: "Class",
					class: "Part",
					part: "description",
					before: null,
					after: "A <code>Part</code>.",
				},
				{
					element: "Member",
					class: "Part",
					member: "Anchored",
					part: "return",
					before: "Same.",
					after: "<p>Same.</p>",
					value: anchored,
				},
			],
		)
		expect(result.counts).toEqual({ Added: 0, Removed: 0, Changed: 0 })
		expect(result.descriptionCount).toBe(2)
		const [owner] = result.owners
		expect(owner?.descriptions).toEqual([
			{
				part: "description",
				status: "Added",
				after: ["A ", { tag: "code", children: ["Part"] }, "."],
			},
		])
		expect(owner?.entries).toHaveLength(1)
		expect(owner?.entries[0]).toMatchObject({
			name: "Anchored",
			status: "Changed",
			descriptions: [
				{
					part: "description",
					status: "Changed",
					before: ["Old text."],
					after: ["New text."],
				},
			],
		})
	})

	test("attaches a description change to the API change of the same member", () => {
		const result = update(
			[
				{
					element: "EnumItem",
					enum: "Material",
					item: "Sand",
					type: "Add",
					value: { Name: "Sand", Value: 3 },
				},
			],
			[
				{
					element: "EnumItem",
					enum: "Material",
					item: "Sand",
					part: "description",
					before: null,
					after: "Grainy.",
					value: { Name: "Sand", Value: 3 },
				},
			],
		)
		const entries = result.owners[0]?.entries ?? []
		expect(entries).toHaveLength(1)
		expect(entries[0]).toMatchObject({
			status: "Added",
			descriptions: [
				{ part: "description", status: "Added", after: ["Grainy."] },
			],
		})
		expect(result.counts.Added).toBe(1)
	})
})
