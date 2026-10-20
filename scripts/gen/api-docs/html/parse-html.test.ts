import { describe, expect, test } from "vite-plus/test"
import { parseHtml } from "./parse-html.ts"

describe("parseHtml", () => {
	test("builds elements, decodes entities, and keeps void elements empty", () => {
		expect(
			parseHtml(
				`A&mdash;<code class="language-lua">x &lt; 2</code><br />B<img src='/a.png'>`,
			),
		).toEqual([
			"A—",
			{
				name: "code",
				attributes: { class: "language-lua" },
				children: ["x < 2"],
			},
			{ name: "br", attributes: {}, children: [] },
			"B",
			{ name: "img", attributes: { src: "/a.png" }, children: [] },
		])
	})

	test("tolerates stray closing tags, unclosed elements, and a lone angle bracket", () => {
		expect(parseHtml("a < b</em><strong>c")).toEqual([
			"a ",
			"<",
			" b",
			{ name: "strong", attributes: {}, children: ["c"] },
		])
	})
})
