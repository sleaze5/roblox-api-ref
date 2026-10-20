import type { RenderedPage } from "../src/server/page-module.ts"
import { absoluteUrl, originUrl, siteName, type Site } from "./site.ts"

type Node = Record<string, unknown>

function website(home: string, page: RenderedPage): Node {
	return {
		"@type": "WebSite",
		"name": siteName,
		"description": page.description,
		"url": home,
		"inLanguage": "en",
		"potentialAction": {
			"@type": "SearchAction",
			"target": {
				"@type": "EntryPoint",
				"urlTemplate": `${home}?search={search_term_string}`,
			},
			"query-input": "required name=search_term_string",
		},
	}
}

function breadcrumbList(page: RenderedPage, site: Site): Node {
	return {
		"@type": "BreadcrumbList",
		"itemListElement": page.breadcrumbs.map((crumb, index) => ({
			"@type": "ListItem",
			"position": index + 1,
			"name": crumb.name,
			"item": originUrl(site, crumb.href),
		})),
	}
}

function webPage(page: RenderedPage, url: string, home: string, site: Site): Node {
	return {
		"@type": page.schemaType,
		"headline": page.breadcrumbs.at(-1)?.name ?? page.title,
		"description": page.description,
		"url": url,
		"inLanguage": "en",
		"dateModified": page.modified,
		"isPartOf": { "@type": "WebSite", "name": siteName, "url": home },
		"breadcrumb": breadcrumbList(page, site),
	}
}

export function structuredData(page: RenderedPage, url: string, site: Site): string {
	const home = absoluteUrl(site, "/")
	if (page.schemaType === undefined || home === undefined) {
		return ""
	}
	const node =
		page.schemaType === "WebSite"
			? website(home, page)
			: webPage(page, url, home, site)
	const json = JSON.stringify({ "@context": "https://schema.org", ...node })
	return `<script type="application/ld+json">${json.replaceAll("<", "\u003c")}</script>`
}
