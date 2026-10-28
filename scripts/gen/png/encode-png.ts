import { deflateSync } from "node:zlib"
import { bytesPerPixel, pngSignature, type Image } from "./image.ts"

function chunk(type: string, body: Uint8Array): Uint8Array {
	const bytes = new Uint8Array(body.length + 12)
	const view = new DataView(bytes.buffer)
	view.setUint32(0, body.length)
	bytes.set(new TextEncoder().encode(type), 4)
	bytes.set(body, 8)
	view.setUint32(body.length + 8, Bun.hash.crc32(bytes.subarray(4, body.length + 8)))
	return bytes
}

function header(image: Image): Uint8Array {
	const body = new Uint8Array(13)
	const view = new DataView(body.buffer)
	view.setUint32(0, image.width)
	view.setUint32(4, image.height)
	body.set([8, 6, 0, 0, 0], 8)
	return body
}

export function encodePng(image: Image): Uint8Array {
	const stride = image.width * bytesPerPixel
	const scanlines = new Uint8Array((stride + 1) * image.height)
	for (let row = 0; row < image.height; row++) {
		scanlines.set(
			image.pixels.subarray(row * stride, (row + 1) * stride),
			row * (stride + 1) + 1,
		)
	}
	const compressed = deflateSync(scanlines, { level: 9, memLevel: 9 })
	return Buffer.concat([
		pngSignature,
		chunk("IHDR", header(image)),
		chunk("IDAT", compressed),
		chunk("IEND", new Uint8Array()),
	])
}
