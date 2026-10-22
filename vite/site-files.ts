import { absoluteUrl } from "../src/site/origin.ts"
import { escapeHtml } from "./escape-html.ts"
import { siteDescription } from "../src/site/brand.ts"

export function robotsTxt(): string {
	return `User-agent: *\nAllow: /\n\nSitemap: ${absoluteUrl("/sitemap.xml")}\n`
}

export function sitemapXml(routes: string[], lastModified: string): string {
	const entries = routes
		.filter((route) => route !== "/404")
		.map(
			(route) =>
				`<url><loc>${escapeHtml(absoluteUrl(route))}</loc><lastmod>${lastModified}</lastmod></url>`,
		)
	return `<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${entries.join("")}</urlset>\n`
}

export function openSearchXml(): string {
	return `<?xml version="1.0" encoding="UTF-8"?><OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/"><ShortName>Roblox API</ShortName><Description>${siteDescription}</Description><InputEncoding>UTF-8</InputEncoding><Url type="text/html" template="${escapeHtml(absoluteUrl("/"))}?search={searchTerms}"/></OpenSearchDescription>\n`
}
