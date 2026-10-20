import type { RenderedPage } from "../src/server/page-module.ts"
import { escapeHtml } from "./escape-html.ts"
import { headTags } from "./head-tags.ts"
import type { Site } from "./site.ts"

export function fillTemplate(
	template: string,
	page: RenderedPage,
	route: string,
	site: Site,
): string {
	return template
		.replace("<!--app-title-->", () => escapeHtml(page.title))
		.replace("<!--app-description-->", () => escapeHtml(page.description))
		.replace("<!--app-head-->", () => headTags(page, route, site))
		.replace("<!--app-body-->", () => page.body)
}
