const origin = import.meta.env["VITE_SITE_ORIGIN"] as string | undefined

export function siteUrl(href: string): string {
	return origin === undefined || origin === "" ? href : new URL(href, origin).href
}
