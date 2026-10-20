import { deflateRawSync } from "node:zlib"
import { describe, expect, test } from "vite-plus/test"
import { extractEntry } from "./extract-entry.ts"
import { memoryRangeReader } from "./range-reader.ts"

type FileSpec = {
	name: string
	text: string
	deflate: boolean
}

function header(size: number, fill: (data: DataView) => void): Uint8Array {
	const bytes = new Uint8Array(size)
	fill(new DataView(bytes.buffer))
	return bytes
}

function concat(parts: Uint8Array[]): Uint8Array {
	const bytes = new Uint8Array(parts.reduce((total, part) => total + part.length, 0))
	let at = 0
	for (const part of parts) {
		bytes.set(part, at)
		at += part.length
	}
	return bytes
}

function buildZip(files: FileSpec[], comment = ""): Uint8Array {
	const encoder = new TextEncoder()
	const locals: Uint8Array[] = []
	const centrals: Uint8Array[] = []
	let offset = 0
	for (const file of files) {
		const name = encoder.encode(file.name)
		const raw = encoder.encode(file.text)
		const data = file.deflate ? new Uint8Array(deflateRawSync(raw)) : raw
		const crc = Bun.hash.crc32(raw)
		const local = header(30, (view) => {
			view.setUint32(0, 0x04034b50, true)
			view.setUint16(8, file.deflate ? 8 : 0, true)
			view.setUint16(26, name.length, true)
		})
		const central = header(46, (view) => {
			view.setUint32(0, 0x02014b50, true)
			view.setUint16(10, file.deflate ? 8 : 0, true)
			view.setUint32(16, crc, true)
			view.setUint32(20, data.length, true)
			view.setUint32(24, raw.length, true)
			view.setUint16(28, name.length, true)
			view.setUint32(42, offset, true)
		})
		locals.push(local, name, data)
		centrals.push(central, name)
		offset += local.length + name.length + data.length
	}
	const directory = concat(centrals)
	const commentBytes = encoder.encode(comment)
	const end = header(22, (view) => {
		view.setUint32(0, 0x06054b50, true)
		view.setUint16(10, files.length, true)
		view.setUint32(12, directory.length, true)
		view.setUint32(16, offset, true)
		view.setUint16(20, commentBytes.length, true)
	})
	return concat([...locals, directory, end, commentBytes])
}

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
