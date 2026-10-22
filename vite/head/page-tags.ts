import type { RenderedPage } from "../../src/server/page-module.ts"
import { siteName } from "../../src/site/brand.ts"
import { meta } from "./html-tags.ts"

export function pageTags(page: RenderedPage): string[] {
	return [
		meta("name", "application-name", siteName),
		meta("name", "apple-mobile-web-app-title", siteName),
		meta(
			"name",
			"robots",
			page.status === 404 ? "noindex" : "index, follow, max-image-preview:large",
		),
		meta("name", "is-agentic-site-type", "content"),
	]
}
