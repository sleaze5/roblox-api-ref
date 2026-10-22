import type { RenderedPage, SchemaType } from "../src/server/page-module.ts"
import { absoluteUrl } from "../src/site/origin.ts"
import { organization, organizationId } from "./organization.ts"
import { siteDescription, siteName } from "../src/site/brand.ts"

type Node = Record<string, unknown>

const home = absoluteUrl("/")

const organizationPages = new Set<SchemaType>(["WebSite", "AboutPage", "ContactPage"])

function website(): Node {
	return {
		"@type": "WebSite",
		"@id": `${home}#website`,
		"name": siteName,
		"alternateName": "Roblox Engine API Reference",
		"description": siteDescription,
		"url": home,
		"inLanguage": "en",
		"publisher": { "@id": organizationId },
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

function breadcrumbList(page: RenderedPage): Node {
	return {
		"@type": "BreadcrumbList",
		"itemListElement": page.breadcrumbs.map((crumb, index) => ({
			"@type": "ListItem",
			"position": index + 1,
			"name": crumb.name,
			"item": absoluteUrl(crumb.href),
		})),
	}
}

function webPage(page: RenderedPage, url: string): Node {
	return {
		"@type": page.schemaType,
		"headline": page.breadcrumbs.at(-1)?.name ?? page.title,
		"description": siteDescription,
		"url": url,
		"inLanguage": "en",
		"dateModified": page.modified,
		"isPartOf": { "@type": "WebSite", "name": siteName, "url": home },
		"breadcrumb": breadcrumbList(page),
		...(page.schemaType === "AboutPage" || page.schemaType === "ContactPage"
			? { mainEntity: { "@id": organizationId } }
			: {}),
	}
}

function graph(page: RenderedPage, url: string): Node {
	const main = page.schemaType === "WebSite" ? website() : webPage(page, url)
	if (page.schemaType === undefined || !organizationPages.has(page.schemaType)) {
		return { "@context": "https://schema.org", ...main }
	}
	return { "@context": "https://schema.org", "@graph": [main, organization()] }
}

export function structuredData(page: RenderedPage, url: string): string {
	if (page.schemaType === undefined) {
		return ""
	}
	const json = JSON.stringify(graph(page, url))
	return `<script type="application/ld+json">${json.replaceAll("<", "\\u003c")}</script>`
}
