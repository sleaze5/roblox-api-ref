import { describe, expect, test } from "vite-plus/test"
import { extractEntry } from "./extract-entry.ts"
import { memoryRangeReader } from "./range-reader.ts"
import { buildZip } from "./test-zip.ts"

const decode = (bytes: Uint8Array) => new TextDecoder().decode(bytes)

describe("extractEntry", () => {
	const zip = buildZip(
		[
			{ name: "RobloxStudioBeta.exe", text: "binary", deflate: false },
			{
				name: "en-us.json",
				text: `{"documentation": ""}`.repeat(50),
				deflate: true,
			},
		],
		"archive comment",
	)

	test("extracts a deflated entry", async () => {
		const bytes = await extractEntry(memoryRangeReader(zip), "en-us.json")
		expect(decode(bytes)).toBe(`{"documentation": ""}`.repeat(50))
	})

	test("extracts a stored entry", async () => {
		const bytes = await extractEntry(memoryRangeReader(zip), "RobloxStudioBeta.exe")
		expect(decode(bytes)).toBe("binary")
	})

	test("rejects a missing entry", async () => {
		await expect(
			extractEntry(memoryRangeReader(zip), "Missing.xml"),
		).rejects.toThrow("does not contain Missing.xml")
	})

	test("rejects a corrupted entry", async () => {
		const corrupted = buildZip([{ name: "a.txt", text: "hello", deflate: false }])
		corrupted[30 + "a.txt".length] = "j".charCodeAt(0)
		await expect(
			extractEntry(memoryRangeReader(corrupted), "a.txt"),
		).rejects.toThrow("integrity check")
	})
})
