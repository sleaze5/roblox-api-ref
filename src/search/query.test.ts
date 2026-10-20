import { describe, expect, test } from "vite-plus/test"
import { entryFlags } from "./index-format.ts"
import { parseQuery } from "./query.ts"

describe("parseQuery", () => {
	test("splits a class scope from the member word", () => {
		const query = parseQuery("Part.Touch")
		expect(query.scope).toBe("Part")
		expect(query.words).toEqual(["Touch"])
	})

	test("reads kind, tag, type, and access filters", () => {
		const query = parseQuery(
			"is:method tag:yields !tag:deprecated type:Color3 access:game",
		)
		expect([...query.kinds]).toEqual(["Method"])
		expect(query.include).toBe(entryFlags.yields)
		expect(query.exclude).toBe(entryFlags.deprecated)
		expect(query.valueType).toBe("Color3")
		expect(query.access).toBe("game")
	})

	test("reads an unknown filter as a Class:member search", () => {
		const query = parseQuery("tag:nothing")
		expect(query.scope).toBe("tag")
		expect(query.words).toEqual(["nothing"])
	})

	test("marks quoted words as exact", () => {
		const query = parseQuery('"Name"')
		expect(query.exact).toBe(true)
		expect(query.words).toEqual(["Name"])
	})

	test("reads a uses filter", () => {
		expect(parseQuery("uses:Enum.Material").uses).toBe("Enum.Material")
	})

	test("limits an Enum. prefix to enums", () => {
		const query = parseQuery("Enum.Mat")
		expect([...query.kinds]).toEqual(["Enum"])
		expect(query.words).toEqual(["Mat"])
	})

	test("reads Enum.Name.Item as an enum item search", () => {
		const query = parseQuery("Enum.Material.Plas")
		expect(query.scope).toBe("Material")
		expect(query.enumScope).toBe(true)
		expect(query.words).toEqual(["Plas"])
	})
})
