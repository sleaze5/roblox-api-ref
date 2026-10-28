import { inflateRawSync } from "node:zlib"
import { readEntries, type ZipEntry } from "./central-directory.ts"
import type { RangeReader } from "./range-reader.ts"

const localSignature = 0x04034b50
const localHeaderSize = 30
const storedMethod = 0
const deflateMethod = 8

async function readCompressed(
	reader: RangeReader,
	entry: ZipEntry,
): Promise<Uint8Array> {
	const headerEnd = entry.localHeaderOffset + localHeaderSize
	const bytes = await reader.read(entry.localHeaderOffset, headerEnd)
	const header = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)
	if (header.getUint32(0, true) !== localSignature) {
		throw new Error(`The zip local header of ${entry.name} is missing.`)
	}
	const dataStart =
		headerEnd + header.getUint16(26, true) + header.getUint16(28, true)
	return reader.read(dataStart, dataStart + entry.compressedSize)
}

function decompress(entry: ZipEntry, compressed: Uint8Array): Uint8Array {
	switch (entry.method) {
		case storedMethod:
			return compressed
		case deflateMethod:
			return new Uint8Array(inflateRawSync(compressed))
		default:
			throw new Error(
				`${entry.name} uses unsupported zip compression method ${entry.method}.`,
			)
	}
}

export async function readEntryData(
	reader: RangeReader,
	entry: ZipEntry,
): Promise<Uint8Array> {
	if (entry.encrypted) {
		throw new Error(`${entry.name} is encrypted.`)
	}
	const bytes = decompress(entry, await readCompressed(reader, entry))
	if (bytes.length !== entry.size || Bun.hash.crc32(bytes) !== entry.crc32) {
		throw new Error(`${entry.name} failed its zip integrity check.`)
	}
	return bytes
}

export async function extractEntry(
	reader: RangeReader,
	name: string,
): Promise<Uint8Array> {
	const entry = (await readEntries(reader)).find((item) => item.name === name)
	if (entry === undefined) {
		throw new Error(`The zip archive does not contain ${name}.`)
	}
	return readEntryData(reader, entry)
}
