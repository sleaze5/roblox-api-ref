import { describe, expect, test } from "vite-plus/test"
import { extractEntries } from "./extract-entries.ts"
import { memoryRangeReader, type RangeReader } from "./range-reader.ts"
import { buildZip } from "./test-zip.ts"

function countingReader(bytes: Uint8Array): RangeReader & { reads: number } {
	const reader = memoryRangeReader(bytes)
	const counting = {
		size: reader.size,
		reads: 0,
		read: async (start: number, end: number) => {
			counting.reads++
			return reader.read(start, end)
		},
	}
	return counting
}

const decode = (bytes: Uint8Array) => new TextDecoder().decode(bytes)

describe("extractEntries", () => {
	const zip = buildZip([
		{ name: "Icons\\Part.png", text: "part", deflate: false },
		{ name: "Icons\\Part@2x.png", text: "part 2x", deflate: false },
		{ name: "Icons\\Model.png", text: "model".repeat(20), deflate: true },
		{ name: "Other\\Readme.txt", text: "readme", deflate: false },
	])

	test("extracts every selected entry", async () => {
		const entries = await extractEntries(
			memoryRangeReader(zip),
			(name) => name.startsWith("Icons\\") && !name.includes("@"),
		)
		expect(
			Object.fromEntries(
				[...entries].map(([name, bytes]) => [name, decode(bytes)]),
			),
		).toEqual({ "Icons\\Part.png": "part", "Icons\\Model.png": "model".repeat(20) })
	})

	test("reads nearby entries with one range request", async () => {
		const reader = countingReader(zip)
		await extractEntries(reader, (name) => name.startsWith("Icons\\"))
		const directoryReads = reader.reads
		const plain = countingReader(zip)
		await extractEntries(plain, () => false)
		expect(directoryReads - plain.reads).toBe(1)
	})
})
