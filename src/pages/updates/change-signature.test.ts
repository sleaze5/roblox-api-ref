import { describe, expect, test } from "vite-plus/test"
import { inlineTokens, type SignatureParts } from "../../api/signature.ts"
import type { Api, Class, Enum, Property } from "../../api/types.ts"
import { changeParts } from "./change-signature.ts"

function property(name: string, type: Property["type"], value?: string): Property {
	return {
		kind: "Property",
		name,
		type,
		category: "Data",
		...(value === undefined ? {} : { default: value }),
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

const surviving = property("Target", {
	category: "Class",
	name: "Gone",
	optional: false,
})
const partClass: Class = {
	name: "Part",
	memoryCategory: "Instances",
	tags: [],
	members: [
		surviving,
		property("Anchored", { category: "Primitive", name: "bool", optional: false }),
	],
	subclasses: [],
}
const material: Enum = {
	name: "Material",
	tags: [],
	items: [{ name: "Grass", value: 1, tags: [], legacyNames: [] }],
	usedBy: [],
}
const api = {
	version: "0.2.0.1",
	hash: "bbbbbbbbbbbbbbbb",
	channel: "LIVE",
	published: "2026-10-19T23:04:06.000Z",
	classes: { Part: partClass },
	enums: { Material: material },
	dataTypes: { bool: { name: "bool", category: "Primitive", usedBy: [] } },
} as unknown as Api

function links(parts: SignatureParts): [string, string | undefined][] {
	return inlineTokens(parts).map((part) => [part.text, part.href])
}

function linkedTexts(parts: SignatureParts): string[] {
	return links(parts)
		.filter(([, href]) => href !== undefined)
		.map(([text]) => text)
}

describe("historical signatures", () => {
	test("keeps the text but drops links to a removed owner", () => {
		const anchored = property("Anchored", {
			category: "Primitive",
			name: "bool",
			optional: false,
		})
		const parts = changeParts(api, "OldPart", anchored, true)
		expect(
			links(parts)
				.map(([text]) => text)
				.join(""),
		).toBe("OldPart.Anchored: boolean")
		expect(linkedTexts(parts)).toEqual(["boolean"])
	})

	test("drops the link to a removed type from a surviving member", () => {
		const parts = changeParts(api, "Part", surviving, true)
		expect(links(parts)).toEqual([
			["Part", "/classes/Part/"],
			[".", undefined],
			["Target", "/classes/Part/#Property-Target"],
			[": ", undefined],
			["Gone", undefined],
		])
	})

	test("drops the link to a removed enum item in a default value", () => {
		const parts = changeParts(
			api,
			"Part",
			property(
				"Surface",
				{ category: "Enum", name: "Material", optional: false },
				"Sand",
			),
			true,
		)
		expect(links(parts).slice(-4)).toEqual([
			["Enum.", "/enums/Material/"],
			["Material", "/enums/Material/"],
			[".", undefined],
			["Sand", undefined],
		])
	})

	test("keeps live enum item links and unlinks every old signature", () => {
		const grass = { name: "Grass", value: 1, tags: [], legacyNames: [] }
		expect(linkedTexts(changeParts(api, "Material", grass, true))).toEqual([
			"Material",
			"Grass",
		])
		expect(linkedTexts(changeParts(api, "Part", surviving, false))).toEqual([])
	})
})
