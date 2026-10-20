export const siteName = "Roblox API Reference"

export type Site = {
	origin: string | undefined
	base: string
}

export function originUrl(site: Site, href: string): string | undefined {
	return site.origin === undefined ? undefined : new URL(href, site.origin).href
}

export function absoluteUrl(site: Site, path: string): string | undefined {
	return originUrl(site, `${site.base}${path.replace(/^\//, "")}`)
}
