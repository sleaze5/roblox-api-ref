import type { RenderedPage } from "../src/server/page-module.ts"
import { siteDescription } from "../src/site/brand.ts"
import { escapeHtml } from "./escape-html.ts"
import { headTags } from "./head-tags.ts"

export function fillTemplate(
	template: string,
	page: RenderedPage,
	route: string,
): string {
	return template
		.replace("<!--app-title-->", () => escapeHtml(page.title))
		.replace("<!--app-description-->", () => escapeHtml(siteDescription))
		.replace("<!--app-head-->", () => headTags(page, route))
		.replace("<!--app-body-->", () => page.body)
}
