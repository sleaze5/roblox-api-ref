import { inflateSync } from "node:zlib"
import { bytesPerPixel, pngSignature, type Image } from "./image.ts"
import { neighbors, predict } from "./unfilter.ts"

type Header = {
	width: number
	height: number
}

function readHeader(data: DataView): Header {
	const bitDepth = data.getUint8(8)
	const colorType = data.getUint8(9)
	const interlace = data.getUint8(12)
	if (bitDepth !== 8 || colorType !== 6 || interlace !== 0) {
		throw new Error("Only non-interlaced 8-bit RGBA PNG images are supported.")
	}
	return { width: data.getUint32(0), height: data.getUint32(4) }
}

function readChunks(bytes: Uint8Array): { header: Header; compressed: Uint8Array } {
	if (!pngSignature.every((byte, index) => bytes[index] === byte)) {
		throw new Error("The PNG signature is missing.")
	}
	const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)
	const parts: Uint8Array[] = []
	let header: Header | undefined
	for (let at = pngSignature.length; at < bytes.length;) {
		const length = view.getUint32(at)
		const type = new TextDecoder().decode(bytes.subarray(at + 4, at + 8))
		const body = bytes.subarray(at + 8, at + 8 + length)
		if (type === "IHDR") {
			header = readHeader(
				new DataView(body.buffer, body.byteOffset, body.byteLength),
			)
		} else if (type === "IDAT") {
			parts.push(body)
		} else if (type === "IEND") {
			break
		}
		at += length + 12
	}
	if (header === undefined) {
		throw new Error("The PNG header is missing.")
	}
	return { header, compressed: Buffer.concat(parts) }
}

export function decodePng(bytes: Uint8Array): Image {
	const { header, compressed } = readChunks(bytes)
	const { width, height } = header
	const stride = width * bytesPerPixel
	const filtered = inflateSync(compressed)
	const pixels = new Uint8Array(stride * height)
	for (let row = 0; row < height; row++) {
		const filter = filtered[row * (stride + 1)] ?? 0
		if (filter > 4) {
			throw new Error(`Unknown PNG filter type ${filter}.`)
		}
		const source = row * (stride + 1) + 1
		for (let column = 0; column < stride; column++) {
			pixels[row * stride + column] =
				(filtered[source + column] ?? 0) +
				predict(filter, neighbors(pixels, row, column, stride))
		}
	}
	return { width, height, pixels }
}
