import { bytesPerPixel } from "./image.ts"

function paeth(left: number, up: number, upLeft: number): number {
	const estimate = left + up - upLeft
	const toLeft = Math.abs(estimate - left)
	const toUp = Math.abs(estimate - up)
	const toUpLeft = Math.abs(estimate - upLeft)
	if (toLeft <= toUp && toLeft <= toUpLeft) {
		return left
	}
	return toUp <= toUpLeft ? up : upLeft
}

export type Neighbors = {
	left: number
	up: number
	upLeft: number
}

export function neighbors(
	pixels: Uint8Array,
	row: number,
	column: number,
	stride: number,
): Neighbors {
	const at = row * stride + column
	const hasLeft = column >= bytesPerPixel
	return {
		left: hasLeft ? (pixels[at - bytesPerPixel] ?? 0) : 0,
		up: row > 0 ? (pixels[at - stride] ?? 0) : 0,
		upLeft: row > 0 && hasLeft ? (pixels[at - stride - bytesPerPixel] ?? 0) : 0,
	}
}

export function predict(filter: number, { left, up, upLeft }: Neighbors): number {
	switch (filter) {
		case 1:
			return left
		case 2:
			return up
		case 3:
			return (left + up) >> 1
		case 4:
			return paeth(left, up, upLeft)
		default:
			return 0
	}
}
