import { releaseText } from "../../api/release.ts"
import {
	classesHref,
	enumsHref,
	indexHref,
	llmsTxtHref,
	typesHref,
	updatesHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { absoluteUrl } from "../../site/origin.ts"
import { joinBlocks } from "./document.ts"
import { link } from "./escape.ts"
import { markdownUrl } from "./markdown-href.ts"

export function notFoundMarkdown(api: Api): string {
	return joinBlocks([
		"# Page not found",
		`This page does not exist in ${releaseText(api)}. Check the spelling and case of the class, enum, or data type name in the URL, or start from one of these pages.`,
		[
			`- ${link("llms.txt", absoluteUrl(llmsTxtHref()))}: What this reference covers and its URL patterns`,
			`- ${link("Classes", markdownUrl(classesHref()))}: Every class`,
			`- ${link("Enums", markdownUrl(enumsHref()))}: Every enum`,
			`- ${link("Data types", markdownUrl(typesHref()))}: Every data type`,
			`- ${link("Index", markdownUrl(indexHref()))}: Every tag, security level, and other facet`,
			`- ${link("Updates", markdownUrl(updatesHref()))}: What changed in each Roblox Studio build`,
			`- ${link("Sitemap", absoluteUrl("/sitemap.xml"))}: Every HTML page`,
		].join("\n"),
	])
}
