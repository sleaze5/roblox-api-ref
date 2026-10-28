import { compareNames } from "../../../src/api/compare-names.ts"
import { bytesPerPixel, type Image } from "../png/image.ts"

export const cellSize = 48

export type Sprite = {
	image: Image
	columns: number
	rows: number
	cells: Map<string, number>
}

function draw(target: Image, icon: Image, cell: number, columns: number): void {
	if (icon.width > cellSize || icon.height > cellSize) {
		throw new Error(`Studio icons must fit in ${cellSize}x${cellSize} pixels.`)
	}
	const left = (cell % columns) * cellSize + Math.floor((cellSize - icon.width) / 2)
	const top =
		Math.floor(cell / columns) * cellSize + Math.floor((cellSize - icon.height) / 2)
	const rowBytes = icon.width * bytesPerPixel
	for (let row = 0; row < icon.height; row++) {
		const from = row * rowBytes
		target.pixels.set(
			icon.pixels.subarray(from, from + rowBytes),
			((top + row) * target.width + left) * bytesPerPixel,
		)
	}
}

export function buildSprite(icons: Map<string, Image>): Sprite {
	const names = [...icons.keys()].sort(compareNames)
	const columns = Math.max(2, Math.ceil(Math.sqrt(names.length)))
	const rows = Math.max(2, Math.ceil(names.length / columns))
	const width = columns * cellSize
	const height = rows * cellSize
	const image = {
		width,
		height,
		pixels: new Uint8Array(width * height * bytesPerPixel),
	}
	const cells = new Map<string, number>()
	names.forEach((name, cell) => {
		const icon = icons.get(name)
		if (icon !== undefined) {
			draw(image, icon, cell, columns)
			cells.set(name, cell)
		}
	})
	return { image, columns, rows, cells }
}
