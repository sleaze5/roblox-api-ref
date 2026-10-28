import { deflateSync } from "node:zlib"
import { describe, expect, test } from "vite-plus/test"
import { decodePng } from "./decode-png.ts"
import { encodePng } from "./encode-png.ts"
import type { Image } from "./image.ts"
import { neighbors, predict } from "./unfilter.ts"

function gradient(width: number, height: number): Image {
	const pixels = new Uint8Array(width * height * 4)
	pixels.forEach((_, index) => {
		pixels[index] = (index * 37 + (index >> 3) * 11) & 0xff
	})
	return { width, height, pixels }
}

function withFilters(png: Uint8Array, image: Image): Uint8Array {
	const stride = image.width * 4
	const scanlines = new Uint8Array((stride + 1) * image.height)
	for (let row = 0; row < image.height; row++) {
		const filter = row % 5
		scanlines[row * (stride + 1)] = filter
		for (let column = 0; column < stride; column++) {
			const value = image.pixels[row * stride + column] ?? 0
			const predicted = predict(
				filter,
				neighbors(image.pixels, row, column, stride),
			)
			scanlines[row * (stride + 1) + 1 + column] = (value - predicted) & 0xff
		}
	}
	const idat = deflateSync(scanlines)
	const view = new DataView(png.buffer, png.byteOffset, png.byteLength)
	const headerEnd = 8 + 12 + view.getUint32(8)
	const body = new Uint8Array(idat.length + 12)
	const bodyView = new DataView(body.buffer)
	bodyView.setUint32(0, idat.length)
	body.set(new TextEncoder().encode("IDAT"), 4)
	body.set(idat, 8)
	return Buffer.concat([png.subarray(0, headerEnd), body, png.subarray(-12)])
}

describe("PNG", () => {
	const image = gradient(5, 6)

	test("decodes what it encodes", () => {
		expect(decodePng(encodePng(image))).toEqual(image)
	})

	test("decodes every row filter", () => {
		expect(decodePng(withFilters(encodePng(image), image))).toEqual(image)
	})

	test("rejects images that are not 8-bit RGBA", () => {
		const png = encodePng(image)
		png[8 + 8 + 9] = 2
		expect(() => decodePng(png)).toThrow("8-bit RGBA")
	})
})
