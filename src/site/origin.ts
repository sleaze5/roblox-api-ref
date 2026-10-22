export const siteOrigin = "https://rapi.sleaze.dev"

export function absoluteUrl(path: string): string {
	return new URL(path, siteOrigin).href
}
