import type { RangeReader } from "./range-reader.ts"

export type ZipEntry = {
	name: string
	method: number
	encrypted: boolean
	crc32: number
	compressedSize: number
	size: number
	localHeaderOffset: number
}

const endSignature = 0x06054b50
const entrySignature = 0x02014b50
const endRecordSize = 22
const maxCommentSize = 0xffff
const entryHeaderSize = 46
const zip64Marker = 0xffffffff

type Directory = {
	offset: number
	size: number
}

function view(bytes: Uint8Array): DataView {
	return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)
}

function findEndRecord(tail: Uint8Array): DataView {
	const data = view(tail)
	for (let at = tail.length - endRecordSize; at >= 0; at--) {
		if (data.getUint32(at, true) === endSignature) {
			return new DataView(tail.buffer, tail.byteOffset + at, endRecordSize)
		}
	}
	throw new Error("The zip end of central directory record is missing.")
}

async function readDirectoryLocation(reader: RangeReader): Promise<Directory> {
	const tailSize = Math.min(reader.size, endRecordSize + maxCommentSize)
	const end = findEndRecord(await reader.read(reader.size - tailSize, reader.size))
	const directory = {
		size: end.getUint32(12, true),
		offset: end.getUint32(16, true),
	}
	if (directory.size === zip64Marker || directory.offset === zip64Marker) {
		throw new Error("ZIP64 archives are not supported.")
	}
	return directory
}

function readEntry(data: DataView, bytes: Uint8Array, at: number): ZipEntry {
	const nameLength = data.getUint16(at + 28, true)
	const nameStart = at + entryHeaderSize
	return {
		name: new TextDecoder().decode(
			bytes.subarray(nameStart, nameStart + nameLength),
		),
		method: data.getUint16(at + 10, true),
		encrypted: (data.getUint16(at + 8, true) & 1) === 1,
		crc32: data.getUint32(at + 16, true),
		compressedSize: data.getUint32(at + 20, true),
		size: data.getUint32(at + 24, true),
		localHeaderOffset: data.getUint32(at + 42, true),
	}
}

function entryLength(data: DataView, at: number): number {
	return (
		entryHeaderSize +
		data.getUint16(at + 28, true) +
		data.getUint16(at + 30, true) +
		data.getUint16(at + 32, true)
	)
}

export async function readEntries(reader: RangeReader): Promise<ZipEntry[]> {
	const directory = await readDirectoryLocation(reader)
	const bytes = await reader.read(directory.offset, directory.offset + directory.size)
	const data = view(bytes)
	const entries: ZipEntry[] = []
	for (let at = 0; at + entryHeaderSize <= bytes.length;) {
		if (data.getUint32(at, true) !== entrySignature) {
			throw new Error(`Unexpected zip central directory record at byte ${at}.`)
		}
		entries.push(readEntry(data, bytes, at))
		at += entryLength(data, at)
	}
	return entries
}
