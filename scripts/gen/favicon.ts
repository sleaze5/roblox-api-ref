import { readIcon } from "./icon-sprite.ts"

const bodyPattern = /<svg[^>]*>([\s\S]*)<\/svg>/

export async function buildFavicon(): Promise<string> {
	const body = bodyPattern.exec(await readIcon("cube"))?.[1] ?? ""
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" rx="56" fill="#111111"/><g fill="#ffffff" transform="translate(32 32) scale(.75)">${body}</g></svg>\n`
}
