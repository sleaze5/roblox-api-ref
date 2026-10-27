import { describe, expect, test } from "vite-plus/test"
import { matchUpdates } from "./update-lookup.ts"

const updates = [
	{ version: "0.745.0.7451137", hash: "7062aec9dbe74b14", published: "2026-10-26" },
	{ version: "0.744.0.7441077", hash: "7aaa6838d36c4448", published: "2026-10-19" },
	{ version: "0.74.0.740001", hash: "0123456789abcdef", published: "2020-01-01" },
]

function versions(query: string): string[] {
	return matchUpdates(updates, query, 8).map((update) => update.version)
}

describe("update lookup", () => {
	test("finds an update by its hash, with or without the version- prefix", () => {
		expect(versions("7062aec9dbe74b14")).toEqual(["0.745.0.7451137"])
		expect(versions(" version-7AAA6838D36C4448 ")).toEqual(["0.744.0.7441077"])
		expect(versions("7aaa68")).toEqual(["0.744.0.7441077"])
	})

	test("finds updates by the start of their version", () => {
		expect(versions("0.745")).toEqual(["0.745.0.7451137"])
		expect(versions("0.74")).toEqual(updates.map((update) => update.version))
	})

	test("ignores short hashes and ordinary names", () => {
		expect(versions("7062")).toEqual([])
		expect(versions("Part")).toEqual([])
	})
})
