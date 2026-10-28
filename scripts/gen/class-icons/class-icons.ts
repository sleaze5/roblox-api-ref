import type { Api } from "../../../src/api/types.ts"
import { decodePng } from "../png/decode-png.ts"
import { encodePng } from "../png/encode-png.ts"
import type { Image } from "../png/image.ts"
import { buildSprite } from "./build-sprite.ts"
import { resolveClassIcons } from "./resolve-class-icons.ts"
import type { StudioIcons } from "./studio-icons-types.ts"

export type ClassIconFiles = {
	png: Uint8Array
	css: string
}

export function applyClassIcons(api: Api, studioIcons: StudioIcons): ClassIconFiles {
	const resolved = resolveClassIcons(api.classes, new Set(Object.keys(studioIcons)))
	const images = new Map<string, Image>()
	for (const name of new Set(resolved.values())) {
		images.set(name, decodePng(Buffer.from(studioIcons[name] ?? "", "base64")))
	}
	const sprite = buildSprite(images)
	for (const [className, icon] of resolved) {
		const item = api.classes[className]
		const cell = sprite.cells.get(icon)
		if (item !== undefined && cell !== undefined) {
			item.icon = cell
		}
	}
	return {
		png: encodePng(sprite.image),
		css: `:root{--class-icon-columns:${sprite.columns};--class-icon-rows:${sprite.rows}}\n`,
	}
}
