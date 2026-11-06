import { describe, expect, test } from "vite-plus/test"
import {
	ghostText,
	helpHtml,
	highlight,
	optionHrefs,
	optionId,
	optionIndex,
	resultsHtml,
	type SearchView,
} from "./search-view.ts"

const view: SearchView = {
	u: [
		[
			"0.745.0.7451137",
			"version-7062aec9dbe74b14",
			"Oct 26, 2026",
			"/updates/0.745.0.7451137/",
		],
	],
	g: [
		[
			"Events",
			[
				[
					4,
					-1,
					"Touched",
					[0, 5],
					"BasePart",
					"",
					0,
					"/classes/BasePart/#Event-Touched",
				],
			],
		],
		["Classes", [[0, 7, "Part<b>", [], "", "", 1, "/classes/Part/"]]],
	],
	x: [
		[
			0,
			"BulkImportService",
			"",
			"0.745.0.7451137",
			"/index/removed/#Class-BulkImportService",
		],
	],
	c: "Part.Touched",
}

describe("search view", () => {
	test("numbers the options: updates, then API items, then removed items", () => {
		expect(optionHrefs(view)).toEqual([
			"/updates/0.745.0.7451137/",
			"/classes/BasePart/#Event-Touched",
			"/classes/Part/",
			"/index/removed/#Class-BulkImportService",
		])
		const html = resultsHtml(view, "/icons.svg")
		for (const [index, href] of optionHrefs(view).entries()) {
			expect(html).toContain(
				`id="${optionId(index)}" role="option" aria-selected="false" href="${href}"`,
			)
		}
		expect(optionIndex(optionId(12))).toBe(12)
		expect(optionIndex("other")).toBe(-1)
	})

	test("highlights the matched characters and escapes names", () => {
		const marked = (html: string) =>
			[...html.matchAll(/<span[^>]*>([^<]*)<\/span>/g)].map((match) => match[1])
		const text = (html: string) => html.replaceAll(/<[^>]*>/g, "")
		expect(marked(highlight("Touched", [0, 5]))).toEqual(["Touch"])
		const parts = highlight("GetTouchingParts", [0, 1, 3, 4, 11, 12])
		expect(marked(parts)).toEqual(["G", "T", "P"])
		expect(text(parts)).toBe("GetTouchingParts")
		const html = resultsHtml(view, "/icons.svg")
		expect(html).toContain("Part&lt;b&gt;")
		expect(html).not.toContain("<b>")
		expect(html).toContain('style="--icon: 7"')
		expect(html).toMatch(/>Deprecated<\/span>/)
		expect(html).toContain('aria-label="Removed"')
		expect(html).toContain('data-status="Removed"')
	})

	test("completes only what follows the typed text", () => {
		expect(ghostText(view, "part.touc")).toBe("hed")
		expect(ghostText(view, "Humanoid")).toBe("")
		expect(ghostText({ ...view, c: null }, "Part")).toBe("")
	})

	test("adds the newest update to the syntax help", () => {
		expect(helpHtml(undefined)).not.toContain("version-")
		expect(helpHtml("version-7062aec9dbe74b14")).toContain(
			'data-search-pick="version-7062aec9dbe74b14"',
		)
		expect(helpHtml(undefined)).toContain('data-search-pick="&quot;Name&quot;"')
	})
})
