import type { RenderedPage } from "../../src/server/page-module.ts"
import { siteName } from "../../src/site/brand.ts"
import { link } from "./html-tags.ts"

export function linkTags(page: RenderedPage, url: string): string[] {
	return [
		...(page.status === 404
			? []
			: [
					link({ rel: "canonical", href: url }),
					link({
						rel: "alternate",
						type: "text/markdown",
						href: page.markdown.href,
					}),
				]),
		link({
			rel: "search",
			type: "application/opensearchdescription+xml",
			title: siteName,
			href: "/opensearch.xml",
		}),
	]
}
