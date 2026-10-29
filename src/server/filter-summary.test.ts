import { describe, expect, test } from "vite-plus/test"
import { fillFilterSummaries } from "./filter-summary.ts"

const field = '<output data-filter-summary aria-live="polite"></output>'

describe("filter summaries", () => {
	test("count the rows of a list", () => {
		const html = `<form data-list-filter="list" data-param="q">${field}</form><ul id="list"><li data-name="A"></li><li data-name="B"><ul><li>x</li></ul></li></ul><ul><li data-name="C"></li></ul>`
		expect(fillFilterSummaries(html)).toContain(">2 total</output>")
	})

	test("count every node of a tree, and every line of a change list with its noun", () => {
		const tree = `<form data-list-filter="tree">${field}</form><div id="tree" data-tree><ul><li><ul><li></li></ul></li></ul></div>`
		expect(fillFilterSummaries(tree)).toContain(">2 total</output>")
		const changes = `<form data-list-filter="changes" data-noun="changes">${field}</form><div id="changes" data-changes><div data-change>a</div><div data-change data-name="b"></div></div>`
		expect(fillFilterSummaries(changes)).toContain(">2 changes</output>")
	})

	test("count the members on a class page", () => {
		const html = `<form data-member-filter role="search">${field}</form><div data-member data-name="A"></div><div data-member data-name="B"></div>`
		expect(fillFilterSummaries(html)).toContain(">2 total</output>")
	})

	test("leave a filter alone when its list is missing", () => {
		const html = `<form data-list-filter="missing">${field}</form>`
		expect(fillFilterSummaries(html)).toBe(html)
	})
})
