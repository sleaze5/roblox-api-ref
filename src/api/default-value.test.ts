import { describe, expect, test } from "vite-plus/test"
import { defaultValueTokens } from "./default-value.ts"
import type { ApiType } from "./types.ts"

function text(apiType: ApiType, raw: string): string {
	return defaultValueTokens(apiType, raw)
		.map((part) => part.text)
		.join("")
}

function type(category: ApiType["category"], name: string): ApiType {
	return { category, name, optional: false }
}

describe("defaultValueTokens", () => {
	test("writes enum defaults as Luau enum items", () => {
		expect(text(type("Enum", "PartType"), "Block")).toBe("Enum.PartType.Block")
	})

	test("quotes strings and lowercases booleans", () => {
		expect(text(type("Primitive", "string"), "Part")).toBe('"Part"')
		expect(text(type("Primitive", "bool"), "False")).toBe("false")
	})

	test("uses Luau shorthands for common data type values", () => {
		expect(text(type("DataType", "Vector3"), "0, 0, 0")).toBe("Vector3.zero")
		expect(
			text(type("DataType", "CFrame"), "0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1"),
		).toBe("CFrame.identity")
	})

	test("writes other data type values as constructors", () => {
		expect(text(type("DataType", "Color3"), "0.5, 0.25, 1")).toBe(
			"Color3.new(0.5, 0.25, 1)",
		)
	})

	test("adds a swatch to Color3 values", () => {
		const [first] = defaultValueTokens(type("DataType", "Color3"), "1, 0, 0")
		expect(first?.swatch).toBe("rgb(255 0 0)")
	})
})
