export type Image = {
	width: number
	height: number
	pixels: Uint8Array
}

export const bytesPerPixel = 4

export const pngSignature = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10])
