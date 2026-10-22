import { releaseText } from "../../api/release.ts"
import {
	aboutHref,
	contactHref,
	privacyHref,
	classesHref,
	classHref,
	enumsHref,
	homeHref,
	typesHref,
	updatesHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { joinBlocks } from "./document.ts"
import { inlineCode, link } from "./escape.ts"
import { classMarkdownUrl, markdownHref } from "./markdown-href.ts"
import { siteDescription } from "../../site/brand.ts"
import { absoluteUrl } from "../../site/origin.ts"

export function llmsTxt(api: Api): string {
	const home = absoluteUrl(homeHref())
	const counts = `${Object.keys(api.classes).length} classes, ${Object.keys(api.enums).length} enums, and ${Object.keys(api.dataTypes).length} data types`
	return joinBlocks([
		"# Roblox API Reference",
		`> ${siteDescription}`,
		`This build is generated from the API dump and the English API docs for ${releaseText(api)} on the ${api.channel} channel, which Roblox published on ${api.published.slice(0, 10)}. It lists ${counts}.`,
		`Each class, enum, and data type page has a Markdown version. To get the Markdown version of a page, remove the trailing slash from the page URL and add ${inlineCode(".md")}. For example, the Markdown version of ${absoluteUrl(classHref("Part"))} is ${classMarkdownUrl("Part")}.`,
		"## When to use this reference",
		[
			"Use this reference to:",
			"",
			"- Look up the properties, methods, events, and callbacks of a Roblox class.",
			"- Find the members that a class inherits from its superclasses.",
			"- Check the Luau signature of a member, including parameter types, default values, and return types.",
			"- Check the items and numeric values of a Roblox enum.",
			"- Check the security level, thread safety, capabilities, and tags of a member, such as Deprecated or Yields.",
			"- Find the class members that use a data type or an enum.",
			"- Check what changed in the API from one Roblox Studio version to the next.",
		].join("\n"),
		"Do not use this reference for tutorials, guides, or the Luau language.",
		"## URL patterns",
		[
			`- Class: ${home}classes/{ClassName}.md`,
			`- Enum: ${home}enums/{EnumName}.md`,
			`- Data type: ${home}index/types/{TypeName}.md`,
			`- Update: ${home}updates/{Version}.md`,
		].join("\n"),
		`Replace the placeholder with the exact name, such as ${inlineCode("Part")}, ${inlineCode("KeyCode")}, or ${inlineCode("CFrame")}, or with a version, such as ${inlineCode(api.version)}. Names are case-sensitive. Enum URLs do not include the ${inlineCode("Enum.")} prefix.`,
		"## Indexes",
		[
			`- ${link("Classes", absoluteUrl(markdownHref(classesHref())))}: Every class, with its superclass and tags, such as Service and Deprecated`,
			`- ${link("Enums", absoluteUrl(markdownHref(enumsHref())))}: Every enum, with its tags`,
			`- ${link("Data types", absoluteUrl(markdownHref(typesHref())))}: Every data type, with its category`,
			`- ${link("Updates", absoluteUrl(markdownHref(updatesHref())))}: Every update, with the number of added, removed, and changed API items`,
		].join("\n"),
		"## Optional",
		[
			`- ${link("About", absoluteUrl(markdownHref(aboutHref())))}: What this site is, where its data comes from, and how it is updated`,
			`- ${link("Contact", absoluteUrl(markdownHref(contactHref())))}: How to report a problem or suggest a feature`,
			`- ${link("Privacy policy", absoluteUrl(markdownHref(privacyHref())))}: What the site collects`,
			`- ${link("Sitemap", `${home}sitemap.xml`)}: Every HTML page`,
		].join("\n"),
	])
}
