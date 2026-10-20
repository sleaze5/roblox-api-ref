import { describe, expect, test } from "vite-plus/test"
import { parseHtml } from "../html/parse-html.ts"
import { toDoc, type LinkResolver } from "./to-doc.ts"

const links: LinkResolver = {
	href: (href) =>
		href.startsWith("javascript:") ? undefined : { kind: "external", url: href },
	code: (text) => (text === "Part" ? { kind: "class", name: "Part" } : undefined),
}

const convert = (html: string) => toDoc(parseHtml(html), links)

describe("toDoc", () => {
	test("collapses whitespace, trims the edges, and removes images", () => {
		expect(
			convert(`  Shows   a <strong>part</strong>.  <img src="/x.png" /> `),
		).toEqual(["Shows a ", { tag: "strong", children: ["part"] }, "."])
	})

	test("links code spans, but not code inside a link or a preformatted block", () => {
		expect(
			convert(
				`<code>Part</code> <a href="https://x.dev"><code>Part</code></a> <pre><code>Part</code></pre>`,
			),
		).toEqual([
			{ tag: "code", children: ["Part"], link: { kind: "class", name: "Part" } },
			" ",
			{
				tag: "a",
				children: [{ tag: "code", children: ["Part"] }],
				link: { kind: "external", url: "https://x.dev" },
			},
			" ",
			{ tag: "pre", children: [{ tag: "code", children: ["Part"] }] },
		])
	})

	test("unwraps unknown elements and links without a target", () => {
		expect(convert(`<span>a</span> <a href="javascript:x">b</a>`)).toEqual(["a b"])
	})

	test("drops whitespace between list items", () => {
		expect(convert("<ul> <li> One </li> <li>Two</li> </ul>")).toEqual([
			{
				tag: "ul",
				children: [
					{ tag: "li", children: ["One"] },
					{ tag: "li", children: ["Two"] },
				],
			},
		])
	})

	test("normalizes quotes and line breaks across merged text", () => {
		expect(
			convert(`Says &ldquo;hi&rdquo; <pre>a\r\n\r\n<span>\r\n</span>\r\nb</pre>`),
		).toEqual([`Says "hi" `, { tag: "pre", children: ["a\n\nb"] }])
	})

	test("trims whitespace, line breaks, and empty elements at block edges", () => {
		expect(
			convert(
				`<br /> <strong> </strong>Hi <code>x </code><br /> <pre><code> y </code></pre><br />`,
			),
		).toEqual([
			"Hi ",
			{ tag: "code", children: ["x "] },
			{ tag: "br", children: [] },
			" ",
			{ tag: "pre", children: [{ tag: "code", children: ["y"] }] },
		])
	})

	test("returns nothing for empty documentation", () => {
		expect(convert(`  <img src="/x.png" /> `)).toBeUndefined()
	})
})
