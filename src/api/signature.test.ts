import { describe, expect, test } from "vite-plus/test"
import { inlineTokens, signatureParts } from "./signature.ts"
import type { Member } from "./types.ts"

const base = {
	tags: [],
	security: "None",
	threadSafety: "Safe",
	capabilities: { read: [], write: [] },
} as const

function signature(member: Member): string {
	return inlineTokens(signatureParts(member, "Part", "#anchor"))
		.map((part) => part.text)
		.join("")
}

describe("signatureParts", () => {
	test("writes a property with its default value", () => {
		const member: Member = {
			...base,
			tags: [],
			capabilities: { read: [], write: [] },
			kind: "Property",
			name: "Anchored",
			type: { category: "Primitive", name: "bool", optional: false },
			category: "Behavior",
			default: "false",
			writeSecurity: "None",
			canLoad: true,
			canSave: true,
			simulationAccess: false,
		}
		expect(signature(member)).toBe("Part.Anchored: boolean = false")
	})

	test("omits the default when the dump has none", () => {
		const member: Member = {
			...base,
			tags: [],
			capabilities: { read: [], write: [] },
			kind: "Property",
			name: "Size",
			type: { category: "DataType", name: "Vector3", optional: false },
			category: "Data",
			writeSecurity: "None",
			canLoad: true,
			canSave: true,
			simulationAccess: false,
		}
		expect(signature(member)).toBe("Part.Size: Vector3")
	})

	test("writes a method with a colon and an optional parameter", () => {
		const member: Member = {
			...base,
			tags: [],
			capabilities: { read: [], write: [] },
			kind: "Method",
			name: "FindFirstChild",
			parameters: [
				{
					name: "name",
					type: { category: "Primitive", name: "string", optional: false },
				},
				{
					name: "recursive",
					type: { category: "Primitive", name: "bool", optional: false },
					default: "false",
				},
			],
			returns: { category: "Class", name: "Instance", optional: true },
			simulationAccess: false,
		}
		expect(signature(member)).toBe(
			"Part:FindFirstChild(name: string, recursive: boolean? = false): Instance?",
		)
	})

	test("writes an event as an RBXScriptSignal", () => {
		const member: Member = {
			...base,
			tags: [],
			capabilities: { read: [], write: [] },
			kind: "Event",
			name: "Touched",
			parameters: [
				{
					name: "otherPart",
					type: { category: "Class", name: "BasePart", optional: false },
				},
			],
		}
		expect(signature(member)).toBe(
			"Part.Touched: RBXScriptSignal<(otherPart: BasePart)>",
		)
	})
})
