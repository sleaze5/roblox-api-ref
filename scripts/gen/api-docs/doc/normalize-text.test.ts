import { describe, expect, test } from "vite-plus/test"
import { normalizeText } from "./normalize-text.ts"

describe("normalizeText", () => {
	test("replaces typographic quotes with plain quotes", () => {
		expect(normalizeText("“It’s ‘on’” „low‟")).toBe(`"It's 'on'" "low"`)
	})

	test("uses LF line endings and keeps at most two line breaks in a row", () => {
		expect(normalizeText("a\r\nb\rc\n\n\n\nd\r\n\r\n\r\ne")).toBe(
			"a\nb\nc\n\nd\n\ne",
		)
	})

	test("leaves other characters alone", () => {
		expect(normalizeText("x — y → z")).toBe("x — y → z")
	})
})
