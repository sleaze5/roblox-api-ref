export const siteName = "Roblox API Reference"

export type Site = {
	origin: string | undefined
	base: string
}

export function absoluteUrl(site: Site, path: string): string | undefined {
	if (site.origin === undefined) {
		return undefined
	}
	return new URL(`${site.base}${path.replace(/^\//, "")}`, site.origin).href
}
