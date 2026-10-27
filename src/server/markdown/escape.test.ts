import { describe, expect, test } from "vite-plus/test"
import { escapeText, tableCell } from "./escape.ts"

describe("escapeText", () => {
	test("escapes Markdown syntax so it reads as plain text", () => {
		expect(escapeText("a *b* _c_ `d` [e] <f>")).toBe(
			"a \\*b\\* \\_c\\_ \\`d\\` \\[e\\] \\<f>",
		)
	})

	test("leaves plain text alone", () => {
		expect(escapeText("Part.Anchored")).toBe("Part.Anchored")
	})
})

describe("tableCell", () => {
	test("escapes pipes so they stay in one cell", () => {
		expect(tableCell("The `|` key")).toBe("The `\\|` key")
	})

	test("joins lines into one", () => {
		expect(tableCell("First.\n\nSecond.")).toBe("First. Second.")
	})
})
