import { describe, expect, test } from "vite-plus/test"
import { preferredType, type MediaType } from "./accept.ts"

const both: MediaType[] = ["text/html", "text/markdown"]

const chrome =
	"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"

describe("preferredType", () => {
	test.each([
		["text/markdown", "text/markdown"],
		["text/markdown, text/html;q=0.8", "text/markdown"],
		["text/html", "text/html"],
		["text/markdown;q=0, text/html", "text/html"],
		["*/*", "text/html"],
		["text/*", "text/html"],
		["text/markdown, text/html, */*", "text/markdown"],
		["text/html, text/markdown", "text/html"],
		["text/html;q=0.5, text/markdown;q=0.9", "text/markdown"],
		["TEXT/MARKDOWN; charset=utf-8", "text/markdown"],
		["text/html;q=0, */*", "text/markdown"],
		["text/markdown;q=0", "text/html"],
		["text/html;q=0", "text/markdown"],
		[chrome, "text/html"],
	])("serves %s as %s", (accept, expected) => {
		expect(preferredType(accept, both)).toBe(expected)
	})

	test("serves the default type when Accept is missing or empty", () => {
		expect(preferredType(null, both)).toBe("text/html")
		expect(preferredType("", both)).toBe("text/html")
	})

	test("finds no type when every type is rejected or unmatched", () => {
		expect(preferredType("application/pdf", both)).toBeUndefined()
		expect(preferredType("text/html;q=0, text/markdown;q=0", both)).toBeUndefined()
		expect(preferredType("text/markdown;q=0", ["text/markdown"])).toBeUndefined()
		expect(preferredType("*/*;q=0", both)).toBeUndefined()
	})
})
