import { describe, expect, test } from "vite-plus/test"
import type { ApiDocs } from "../gen/api-docs/api-docs-types.ts"
import type { Dump, DumpMember } from "../gen/dump-types.ts"
import { diffDescriptions } from "./diff-descriptions.ts"

const touch: DumpMember = {
	MemberType: "Function",
	Name: "Touch",
	Parameters: [{ Name: "other", Type: { Category: "Class", Name: "BasePart" } }],
	ReturnType: { Category: "Primitive", Name: "bool" },
	Security: "None",
}

const dump: Dump = {
	Version: 1,
	Classes: [
		{
			Name: "Part",
			Superclass: "Instance",
			MemoryCategory: "Instances",
			Members: [touch],
		},
	],
	Enums: [{ Name: "Material", Items: [{ Name: "Grass", Value: 1 }] }],
}

function docs(overrides: ApiDocs): ApiDocs {
	return {
		"@roblox/globaltype/Part": { documentation: "A part." },
		"@roblox/globaltype/Part.Touch": {
			documentation: "Touches.",
			params: [
				{
					name: "other",
					documentation: "@roblox/globaltype/Part.Touch/param/0",
				},
			],
			returns: ["@roblox/globaltype/Part.Touch/return/0"],
		},
		"@roblox/globaltype/Part.Touch/param/0": { documentation: "The other part." },
		"@roblox/globaltype/Part.Touch/return/0": {
			documentation: "Whether it worked.",
		},
		"@roblox/global/Enum.Material": { documentation: "Materials." },
		"@roblox/enum/Material.Grass": { documentation: "Grass." },
		...overrides,
	}
}

describe("description diffs", () => {
	test("finds nothing when the descriptions match", () => {
		expect(
			diffDescriptions({ dump, docs: docs({}) }, { dump, docs: docs({}) }),
		).toEqual([])
	})

	test("records added, changed, and removed descriptions with their targets", () => {
		const changes = diffDescriptions(
			{ dump, docs: docs({ "@roblox/enum/Material.Grass": undefined }) },
			{
				dump,
				docs: docs({
					"@roblox/globaltype/Part.Touch/param/0": {
						documentation: "Another part.",
					},
					"@roblox/global/Enum.Material": undefined,
				}),
			},
		)
		expect(changes).toEqual([
			{
				element: "Member",
				class: "Part",
				member: "Touch",
				part: "parameter",
				parameter: "other",
				before: "The other part.",
				after: "Another part.",
				value: touch,
			},
			{
				element: "Enum",
				enum: "Material",
				part: "description",
				before: "Materials.",
				after: null,
			},
			{
				element: "EnumItem",
				enum: "Material",
				item: "Grass",
				part: "description",
				before: null,
				after: "Grass.",
				value: { Name: "Grass", Value: 1 },
			},
		])
	})

	test("skips the descriptions of removed members", () => {
		const after: Dump = { ...dump, Classes: [{ ...dump.Classes[0]!, Members: [] }] }
		expect(
			diffDescriptions({ dump, docs: docs({}) }, { dump: after, docs: docs({}) }),
		).toEqual([])
	})
})
