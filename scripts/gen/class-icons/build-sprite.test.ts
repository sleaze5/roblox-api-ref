import { describe, expect, test } from "vite-plus/test"
import type { Image } from "../png/image.ts"
import { buildSprite, cellSize } from "./build-sprite.ts"

function solid(width: number, height: number, value: number): Image {
	return { width, height, pixels: new Uint8Array(width * height * 4).fill(value) }
}

function pixel(image: Image, x: number, y: number): number | undefined {
	return image.pixels[(y * image.width + x) * 4]
}

describe("class icon sprite", () => {
	const sprite = buildSprite(
		new Map([
			["Part", solid(cellSize, cellSize, 1)],
			["Model", solid(cellSize, cellSize, 2)],
			["Folder", solid(cellSize, cellSize / 2, 3)],
		]),
	)

	test("places icons in name order on a square grid", () => {
		expect([...sprite.cells]).toEqual([
			["Folder", 0],
			["Model", 1],
			["Part", 2],
		])
		expect([sprite.columns, sprite.rows]).toEqual([2, 2])
		expect([sprite.image.width, sprite.image.height]).toEqual([
			cellSize * 2,
			cellSize * 2,
		])
		expect(pixel(sprite.image, cellSize, 0)).toBe(2)
		expect(pixel(sprite.image, 0, cellSize)).toBe(1)
	})

	test("centers icons that are shorter than a cell", () => {
		expect(pixel(sprite.image, 0, 0)).toBe(0)
		expect(pixel(sprite.image, 0, cellSize / 4)).toBe(3)
		expect(pixel(sprite.image, 0, (cellSize * 3) / 4)).toBe(0)
	})
})
