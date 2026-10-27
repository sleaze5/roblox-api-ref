import { describe, expect, test } from "vite-plus/test"
import type { Update } from "../api/update-types.ts"
import { siteOrigin } from "../site/origin.ts"
import { updatesFeed } from "./feed.ts"

function update(index: number): Update {
	return {
		version: `0.${index}.0.1`,
		hash: "aaaaaaaaaaaaaaaa",
		published: `2026-01-${String(index).padStart(2, "0")}T00:00:00.000Z`,
		previous: { version: "0.0.0.1", hash: "bbbbbbbbbbbbbbbb" },
		counts: { Added: 1, Removed: 2, Changed: 3 },
		descriptionCount: 0,
		owners: [],
	}
}

describe("updates feed", () => {
	const updates = Array.from({ length: 25 }, (_, index) => update(25 - index))
	const feed = updatesFeed(updates)

	test("lists the 20 newest updates as Atom entries", () => {
		expect(feed).toMatch(
			/^<\?xml version="1.0" encoding="UTF-8"\?><feed xmlns="http:\/\/www.w3.org\/2005\/Atom">/,
		)
		expect(feed.match(/<entry>/g)).toHaveLength(20)
		expect(feed).toContain(`<link href="${siteOrigin}/updates/0.25.0.1/"/>`)
		expect(feed).toContain("<updated>2026-01-25T00:00:00.000Z</updated>")
		expect(feed).toContain("1 added, 2 removed, and 3 changed.")
	})
})
