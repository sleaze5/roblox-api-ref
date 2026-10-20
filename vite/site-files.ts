import { escapeHtml } from "./escape-html.ts"
import { absoluteUrl, siteName, type Site } from "./site.ts"

export function robotsTxt(site: Site, routes: string[]): string {
	const sitemap = absoluteUrl(site, "/sitemap.xml")
	const lines = ["User-agent: *", "Allow: /"]
	if (sitemap !== undefined && routes.length > 0) {
		lines.push(`Sitemap: ${sitemap}`)
	}
	return `${lines.join("\n")}\n`
}

export function sitemapXml(site: Site, routes: string[]): string | undefined {
	const urls = routes
		.filter((route) => route !== "/404")
		.map((route) => absoluteUrl(site, route))
	if (urls.some((url) => url === undefined)) {
		return undefined
	}
	const entries = urls.map((url) => `<url><loc>${escapeHtml(url ?? "")}</loc></url>`)
	return `<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${entries.join("")}</urlset>\n`
}

export function openSearchXml(site: Site): string | undefined {
	const template = absoluteUrl(site, "/")
	if (template === undefined) {
		return undefined
	}
	return `<?xml version="1.0" encoding="UTF-8"?><OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/"><ShortName>Roblox API</ShortName><Description>Search the ${siteName}</Description><InputEncoding>UTF-8</InputEncoding><Url type="text/html" template="${escapeHtml(template)}?search={searchTerms}"/></OpenSearchDescription>\n`
}
