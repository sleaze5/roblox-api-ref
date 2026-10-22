import type { RenderedPage } from "../../src/server/page-module.ts"
import { siteDescription, siteName } from "../../src/site/brand.ts"
import { absoluteUrl } from "../../src/site/origin.ts"
import { meta } from "./html-tags.ts"

const image = {
	url: absoluteUrl("/og-image.png"),
	type: "image/png",
	width: "1200",
	height: "630",
	alt: siteName,
}

function openGraphTags(page: RenderedPage, url: string): string[] {
	const article = page.schemaType === "TechArticle"
	return [
		meta("property", "og:type", article ? "article" : "website"),
		meta("property", "og:site_name", siteName),
		meta("property", "og:title", page.title),
		meta("property", "og:description", siteDescription),
		meta("property", "og:url", url),
		meta("property", "og:locale", "en_US"),
		meta("property", "og:image", image.url),
		meta("property", "og:image:secure_url", image.url),
		meta("property", "og:image:type", image.type),
		meta("property", "og:image:width", image.width),
		meta("property", "og:image:height", image.height),
		meta("property", "og:image:alt", image.alt),
		...(article ? [meta("property", "article:modified_time", page.modified)] : []),
	]
}

function twitterTags(page: RenderedPage): string[] {
	return [
		meta("name", "twitter:card", "summary_large_image"),
		meta("name", "twitter:title", page.title),
		meta("name", "twitter:description", siteDescription),
		meta("name", "twitter:image", image.url),
		meta("name", "twitter:image:alt", image.alt),
	]
}

export function socialTags(page: RenderedPage, url: string): string[] {
	return [...openGraphTags(page, url), ...twitterTags(page)]
}
