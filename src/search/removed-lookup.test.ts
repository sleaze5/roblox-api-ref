import { describe, expect, test } from "vite-plus/test"
import type { RemovedEntry } from "./index-format.ts"
import { parseQuery } from "./query.ts"
import { matchRemoved } from "./removed-lookup.ts"

const removed: RemovedEntry[] = [
	{ kind: "Class", name: "BulkImportService", version: "3", published: "3" },
	{ kind: "Method", name: "isA", parent: "Instance", version: "2", published: "2" },
	{ kind: "EnumItem", name: "Old", parent: "Material", version: "1", published: "1" },
]

function names(query: string): string[] {
	return matchRemoved(removed, parseQuery(query), 8).map((entry) => entry.name)
}

describe("removed lookup", () => {
	test("finds removed classes, members, and enum items by name", () => {
		expect(names("BulkImport")).toEqual(["BulkImportService"])
		expect(names("Instance:isA")).toEqual(["isA"])
		expect(names("Enum.Material.Old")).toEqual(["Old"])
	})

	test("lists everything removed from a class for a scoped query", () => {
		expect(names("Instance.")).toEqual(["isA"])
	})

	test("respects kind filters", () => {
		expect(names("is:class isA")).toEqual([])
	})
})
