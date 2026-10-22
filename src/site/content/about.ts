import { releaseText, type Release } from "../../api/release.ts"
import {
	classesHref,
	contactHref,
	enumsHref,
	llmsTxtHref,
	updatesHref,
} from "../../api/routes.ts"
import { siteDescription } from "../brand.ts"
import { repositoryUrl } from "../repository.ts"
import { list, paragraph, type PageContent } from "./page-content.ts"

export function aboutContent(release: Release): PageContent {
	return {
		lead: siteDescription,
		sections: [
			{
				id: "what-it-covers",
				heading: "What it covers",
				blocks: [
					paragraph(
						"The site lists every ",
						{ text: "class", href: classesHref() },
						", member, ",
						{ text: "enum", href: enumsHref() },
						", enum item, and data type in the Roblox Engine API. Each member shows its Luau signature, its default value, and the security level, thread safety, capabilities, and tags that control where scripts can use it. The site also shows:",
					),
					list(
						["The members that each class inherits from its superclasses."],
						[
							"The properties, parameters, and return values that use each data type and enum.",
						],
						[
							"The API changes between Roblox Studio builds, on the ",
							{ text: "Updates", href: updatesHref() },
							" page.",
						],
					),
				],
			},
			{
				id: "data-sources",
				heading: "Where the data comes from",
				blocks: [
					paragraph(
						"Every page is generated from two files that Roblox publishes with each Roblox Studio build:",
					),
					list(
						["The API dump, which lists the classes, members, and enums."],
						["The English API docs, which hold the descriptions."],
					),
					paragraph(
						`Descriptions are copied from the API docs without changes. This build covers ${releaseText(release)}. Every hour, the site checks the LIVE channel for a new Roblox Studio build, and rebuilds when Roblox publishes one.`,
					),
				],
			},
			{
				id: "agents",
				heading: "For AI agents and tools",
				blocks: [
					paragraph(
						"Each class, enum, data type, update, and site page has a Markdown version. To get it, remove the trailing slash from the page URL and add ",
						{ code: ".md" },
						". The ",
						{ text: "llms.txt", href: llmsTxtHref() },
						" file lists the URL patterns and indexes.",
					),
				],
			},
			{
				id: "affiliation",
				heading: "Not affiliated with Roblox",
				blocks: [
					paragraph(
						"This site is not an official Roblox product. It is not affiliated with or endorsed by Roblox Corporation. Roblox is a trademark of Roblox Corporation.",
					),
				],
			},
			{
				id: "source-code",
				heading: "Source code",
				blocks: [
					paragraph(
						"The site is open source. The code, the build history, and the change list for each update are ",
						{ text: "on GitHub", href: repositoryUrl },
						". To report a problem, see ",
						{ text: "Contact", href: contactHref() },
						".",
					),
				],
			},
		],
	}
}
