import { describe, expect, test } from "vite-plus/test"
import type { Method, Property } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
import { updateMarkdown } from "./update-markdown.ts"

const yielding: Method = {
	kind: "Method",
	name: "AddReadOnlyReasonsAsync",
	parameters: [],
	returns: { category: "Primitive", name: "void", optional: false },
	tags: ["Yields"],
	security: "PluginSecurity",
	threadSafety: "Unsafe",
	capabilities: { read: [], write: [] },
	simulationAccess: false,
}

const restricted: Property = {
	kind: "Property",
	name: "IncomingCall",
	type: { category: "Primitive", name: "bool", optional: false },
	category: "Data",
	tags: [],
	security: "RobloxScriptSecurity",
	writeSecurity: "RobloxScriptSecurity",
	threadSafety: "ReadSafe",
	capabilities: { read: [], write: [] },
	canLoad: false,
	canSave: false,
	simulationAccess: false,
}

const update: Update = {
	version: "0.2.0.1",
	hash: "bbbbbbbbbbbbbbbb",
	published: "2026-10-19T23:04:06.000Z",
	previous: { version: "0.1.0.1", hash: "aaaaaaaaaaaaaaaa" },
	counts: { Added: 3, Removed: 0, Changed: 0 },
	owners: [
		{
			kind: "Class",
			name: "ScriptDocument",
			details: [],
			entries: [
				{
					kind: "Member",
					status: "Added",
					owner: "ScriptDocument",
					name: yielding.name,
					details: [],
					after: yielding,
				},
				{
					kind: "Member",
					status: "Added",
					owner: "ScriptDocument",
					name: restricted.name,
					details: [],
					after: restricted,
				},
			],
		},
		{
			kind: "Enum",
			name: "Material",
			details: [],
			entries: [
				{
					kind: "EnumItem",
					status: "Added",
					owner: "Material",
					name: "Old",
					details: [],
					after: {
						name: "Old",
						value: 4,
						tags: ["Deprecated"],
						legacyNames: [],
					},
				},
			],
		},
	],
}

describe("update Markdown", () => {
	test("lists the tags of added members and enum items, as the HTML page does", () => {
		const lines = updateMarkdown(update).split("\n")
		expect(lines).toContain(
			"- Added `ScriptDocument:AddReadOnlyReasonsAsync()`. Tags: Yields, Plugin security",
		)
		expect(lines).toContain(
			"- Added `ScriptDocument.IncomingCall: boolean`. Tags: Roblox script security",
		)
		expect(lines).toContain("- Added `Enum.Material.Old = 4`. Tags: Deprecated")
	})
})
