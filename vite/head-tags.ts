import type { RenderedPage } from "../src/server/page-module.ts"
import { absoluteUrl } from "../src/site/origin.ts"
import { linkTags } from "./head/link-tags.ts"
import { pageTags } from "./head/page-tags.ts"
import { socialTags } from "./head/social-tags.ts"
import { structuredData } from "./structured-data.ts"

export function headTags(page: RenderedPage, route: string): string {
	const url = absoluteUrl(route === "/404" ? "/404.html" : route)
	return [
		...pageTags(page),
		...socialTags(page, url),
		...linkTags(page, url),
		structuredData(page, url),
	].join("")
}
