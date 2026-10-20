import type { RenderedPage } from "../src/server/page-module.ts"
import { escapeHtml } from "./escape-html.ts"
import { absoluteUrl, siteName, type Site } from "./site.ts"
import { structuredData } from "./structured-data.ts"

const image = { path: "/og-image.png", width: "1200", height: "630" }

function meta(attribute: "name" | "property", key: string, content: string): string {
	return `<meta ${attribute}="${key}" content="${escapeHtml(content)}">`
}

function link(attributes: Record<string, string>): string {
	const pairs = Object.entries(attributes).map(
		([key, value]) => `${key}="${escapeHtml(value)}"`,
	)
	return `<link ${pairs.join(" ")}>`
}

function sharedTags(page: RenderedPage): string[] {
	const article = page.schemaType === "TechArticle"
	return [
		meta("name", "is-agentic-site-type", "content"),
		meta("property", "og:type", article ? "article" : "website"),
		meta("property", "og:site_name", siteName),
		meta("property", "og:title", page.title),
		meta("property", "og:description", page.description),
		meta("property", "og:locale", "en_US"),
		meta("name", "twitter:title", page.title),
		meta("name", "twitter:description", page.description),
		...(article ? [meta("property", "article:modified_time", page.modified)] : []),
	]
}

function imageTags(site: Site): string[] {
	const url = absoluteUrl(site, image.path)
	if (url === undefined) {
		return [meta("name", "twitter:card", "summary")]
	}
	return [
		meta("name", "twitter:card", "summary_large_image"),
		meta("property", "og:image", url),
		meta("property", "og:image:type", "image/png"),
		meta("property", "og:image:width", image.width),
		meta("property", "og:image:height", image.height),
		meta("property", "og:image:alt", siteName),
		meta("name", "twitter:image", url),
	]
}

function discoveryTags(page: RenderedPage, site: Site): string[] {
	const tags: string[] = []
	if (page.markdown !== undefined) {
		tags.push(
			link({ rel: "alternate", type: "text/markdown", href: page.markdown.href }),
		)
	}
	if (site.origin !== undefined) {
		tags.push(
			link({
				rel: "search",
				type: "application/opensearchdescription+xml",
				title: siteName,
				href: `${site.base}opensearch.xml`,
			}),
		)
	}
	return tags
}

function canonicalTags(page: RenderedPage, url: string | undefined): string[] {
	if (page.status === 404) {
		return [meta("name", "robots", "noindex")]
	}
	if (url === undefined) {
		return []
	}
	return [link({ rel: "canonical", href: url }), meta("property", "og:url", url)]
}

export function headTags(page: RenderedPage, route: string, site: Site): string {
	const url = absoluteUrl(site, route === "/404" ? "/404.html" : route)
	return [
		...sharedTags(page),
		...imageTags(site),
		...canonicalTags(page, url),
		...discoveryTags(page, site),
		url === undefined ? "" : structuredData(page, url, site),
	].join("")
}
