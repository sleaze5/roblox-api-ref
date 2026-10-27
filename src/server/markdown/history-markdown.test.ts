import { describe, expect, test } from "vite-plus/test"
import type { EnumItem } from "../../api/types.ts"
import { siteOrigin } from "../../site/origin.ts"
import { removedMarkdown } from "./history-markdown.ts"

const old: EnumItem = { name: "Old", value: 4, tags: [], legacyNames: [] }

describe("removed Markdown", () => {
	test("lists each removed item with a link to its removal", () => {
		expect(
			removedMarkdown("Removed items", "Enum", "Material", [
				{
					value: old,
					removed: {
						version: "0.2.0.1",
						published: "2026-10-05T00:00:00.000Z",
					},
				},
			]),
		).toBe(
			`## Removed items\n\n- \`Enum.Material.Old = 4\`: removed in [0.2.0.1](${siteOrigin}/updates/0.2.0.1/#Enum-Material.Old) (Oct 05, 2026)`,
		)
	})

	test("adds nothing when nothing was removed", () => {
		expect(
			removedMarkdown("Removed items", "Enum", "Material", undefined),
		).toBeUndefined()
	})
})
