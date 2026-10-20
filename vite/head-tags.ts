import type { RenderedPage } from "../src/server/page-module.ts"
import { escapeHtml } from "./escape-html.ts"
import { absoluteUrl, siteName, type Site } from "./site.ts"

function meta(attribute: "name" | "property", key: string, content: string): string {
	return `<meta ${attribute}="${key}" content="${escapeHtml(content)}">`
}

export function headTags(page: RenderedPage, route: string, site: Site): string {
	const url = absoluteUrl(site, route === "/404" ? "/404.html" : route)
	const tags = [
		meta("property", "og:type", "website"),
		meta("property", "og:site_name", siteName),
		meta("property", "og:title", page.title),
		meta("property", "og:description", page.description),
		meta("property", "og:locale", "en_US"),
		meta("name", "twitter:card", "summary"),
		meta("name", "twitter:title", page.title),
		meta("name", "twitter:description", page.description),
	]
	if (page.status === 404) {
		tags.push(meta("name", "robots", "noindex"))
	} else if (url !== undefined) {
		tags.push(
			`<link rel="canonical" href="${escapeHtml(url)}">`,
			meta("property", "og:url", url),
		)
	}
	if (site.origin !== undefined) {
		tags.push(
			`<link rel="search" type="application/opensearchdescription+xml" title="${siteName}" href="${site.base}opensearch.xml">`,
		)
	}
	return tags.join("")
}
