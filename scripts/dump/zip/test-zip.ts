import { deflateRawSync } from "node:zlib"

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

export function buildZip(files: FileSpec[], comment = ""): Uint8Array {
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
