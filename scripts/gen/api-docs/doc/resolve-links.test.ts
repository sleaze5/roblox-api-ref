import { describe, expect, test } from "vite-plus/test"
import { testNames } from "../test-names.ts"
import { resolveCode } from "./resolve-code.ts"
import { resolveHref } from "./resolve-href.ts"

const names = testNames()

describe("resolveCode", () => {
	test("links classes, data types, enums, enum items, and inherited members", () => {
		expect(resolveCode(names, "Part")).toEqual({ kind: "class", name: "Part" })
		expect(resolveCode(names, "Vector3")).toEqual({ kind: "type", name: "Vector3" })
		expect(resolveCode(names, "Enum.Material")).toEqual({
			kind: "enum",
			name: "Material",
		})
		expect(resolveCode(names, "Enum.Material.Plastic")).toEqual({
			kind: "enumItem",
			owner: "Material",
			name: "Plastic",
		})
		expect(resolveCode(names, "Part.Anchored")).toEqual({
			kind: "member",
			owner: "BasePart",
			member: "Property",
			name: "Anchored",
		})
		expect(resolveCode(names, "Part:FindFirstChild()")).toEqual({
			kind: "member",
			owner: "Instance",
			member: "Method",
			name: "FindFirstChild",
		})
	})

	test("links bare names to members of the documented class or items of the documented enum", () => {
		const part = { kind: "class", name: "Part" } as const
		const material = { kind: "enum", name: "Material" } as const
		expect(resolveCode(names, "Anchored", part)).toEqual({
			kind: "member",
			owner: "BasePart",
			member: "Property",
			name: "Anchored",
		})
		expect(resolveCode(names, "Vector3", part)).toEqual({
			kind: "type",
			name: "Vector3",
		})
		expect(resolveCode(names, "Plastic", material)).toEqual({
			kind: "enumItem",
			owner: "Material",
			name: "Plastic",
		})
		expect(resolveCode(names, "FindFirstChild()", part)).toEqual({
			kind: "member",
			owner: "Instance",
			member: "Method",
			name: "FindFirstChild",
		})
		expect(resolveCode(names, "Anchored()", part)).toBeUndefined()
		expect(resolveCode(names, "Plastic()", material)).toBeUndefined()
		expect(resolveCode(names, "Anchored")).toBeUndefined()
		expect(resolveCode(names, "Plastic", part)).toBeUndefined()
	})

	test("leaves anything that does not name the API exactly", () => {
		const texts = [
			"true",
			"Material",
			"Enum.Material.Missing",
			"Part.Missing",
			`Instance.new("Part")`,
			"a b",
		]
		for (const text of texts) {
			expect(resolveCode(names, text)).toBeUndefined()
		}
	})
})

describe("resolveHref", () => {
	test("points Creator Hub reference links at this site", () => {
		expect(
			resolveHref(
				names,
				"https://create.roblox.com/docs/reference/engine/enums/Material#Plastic",
			),
		).toEqual({ kind: "enumItem", owner: "Material", name: "Plastic" })
		expect(
			resolveHref(
				names,
				"https://create.roblox.com/docs/en-us/reference/engine/classes/Part#Anchored",
			),
		).toEqual({
			kind: "member",
			owner: "BasePart",
			member: "Property",
			name: "Anchored",
		})
		expect(resolveHref(names, "../datatypes/Vector3.md")).toEqual({
			kind: "type",
			name: "Vector3",
		})
	})

	test("resolves guide links against Creator Hub", () => {
		expect(
			resolveHref(names, "../../../input/input-action-system.md#setup"),
		).toEqual({
			kind: "external",
			url: "https://create.roblox.com/docs/input/input-action-system#setup",
		})
		expect(resolveHref(names, "/luau/strings.md")).toEqual({
			kind: "external",
			url: "https://create.roblox.com/docs/luau/strings",
		})
		expect(resolveHref(names, "../../../projects/assets/index.md")).toEqual({
			kind: "external",
			url: "https://create.roblox.com/docs/projects/assets",
		})
		expect(resolveHref(names, "https://devforum.roblox.com/t/x/1")).toEqual({
			kind: "external",
			url: "https://devforum.roblox.com/t/x/1",
		})
	})

	test("keeps unknown reference pages on Creator Hub and drops unsafe links", () => {
		expect(
			resolveHref(
				names,
				"https://create.roblox.com/docs/reference/engine/classes/Gone",
			),
		).toEqual({
			kind: "external",
			url: "https://create.roblox.com/docs/reference/engine/classes/Gone",
		})
		expect(resolveHref(names, "javascript:alert(1)")).toBeUndefined()
	})
})
