const localOrigin = "http://localhost:5173"

function resolveOrigin(): string {
	const configured =
		process.env["SITE_ORIGIN"] || process.env["CF_PAGES_URL"] || localOrigin
	return new URL(configured).origin
}

export const siteOrigin = resolveOrigin()

export function absoluteUrl(path: string): string {
	return new URL(path, siteOrigin).href
}
