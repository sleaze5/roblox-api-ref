import { describe, expect, test } from "vite-plus/test"
import type { HistoryEntry } from "../../api/item-history.ts"
import { entryText } from "./history-text.ts"

function entry(fields: Partial<HistoryEntry>): HistoryEntry {
	return { version: "1", published: "2026-01-01", kinds: [], ...fields }
}

describe("history text", () => {
	test("counts members, then names what changed on the class", () => {
		expect(
			entryText(
				entry({
					kinds: ["changed"],
					members: { added: 2, removed: 0, changed: 1 },
					changes: { fields: ["superclass"], tagsAdded: 2, tagsRemoved: 0 },
				}),
			),
		).toBe("2 members added, 1 changed, superclass updated, 2 tags added")
	})

	test("names what changed on the members, and counts deprecated members", () => {
		expect(
			entryText(
				entry({
					kinds: [],
					members: { added: 2, removed: 0, changed: 3 },
					changes: {
						fields: ["security"],
						tagsAdded: 2,
						tagsRemoved: 0,
						deprecated: 1,
					},
				}),
			),
		).toBe(
			"2 members added, 3 changed, 1 deprecated, security updated, 2 tags added",
		)
	})

	test("calls the members of an enum items", () => {
		expect(
			entryText(entry({ members: { added: 1, removed: 3, changed: 0 } }), "item"),
		).toBe("1 item added, 3 removed")
	})

	test("puts a deprecation first and the description last", () => {
		expect(
			entryText(
				entry({
					kinds: ["changed", "described"],
					changes: {
						fields: ["replacement", "security"],
						tagsAdded: 0,
						tagsRemoved: 1,
						deprecation: "set",
					},
				}),
			),
		).toBe(
			"Deprecated, replacement updated, security updated, 1 tag removed, description updated",
		)
	})

	test("says added, removed, or changed when nothing more is known", () => {
		expect(entryText(entry({ kinds: ["added"] }))).toBe("Added")
		expect(entryText(entry({ kinds: ["changed"] }))).toBe("Changed")
		expect(entryText(entry({ kinds: ["described"] }))).toBe("Description updated")
	})
})
