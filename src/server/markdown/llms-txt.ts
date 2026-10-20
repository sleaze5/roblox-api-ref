import {
	classesHref,
	classHref,
	enumsHref,
	homeHref,
	typesHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { joinBlocks } from "./document.ts"
import { inlineCode, link } from "./escape.ts"
import { classMarkdownUrl, markdownHref } from "./markdown-href.ts"
import { siteUrl } from "./site-url.ts"

export function llmsTxt(api: Api): string {
	const home = siteUrl(homeHref())
	const counts = `${Object.keys(api.classes).length} classes, ${Object.keys(api.enums).length} enums, and ${Object.keys(api.dataTypes).length} data types`
	return joinBlocks([
		"# Roblox API Reference",
		`> A reference for the Roblox Engine API. It is generated from the API dump and the English API docs of Roblox Studio ${api.version} (${api.channel} channel), which Roblox published on ${api.published.slice(0, 10)}. It lists ${counts}.`,
		`This reference is not an official Roblox product. Each class, enum, and data type page has a Markdown version. To get the Markdown version of a page, remove the trailing slash from the page URL and add ${inlineCode(".md")}. For example, the Markdown version of ${siteUrl(classHref("Part"))} is ${classMarkdownUrl("Part")}.`,
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
		].join("\n"),
		"Do not use this reference for tutorials, guides, or the Luau language. For those, use the Roblox Creator Hub at https://create.roblox.com/docs.",
		"## URL patterns",
		[
			`- Class: ${home}classes/{ClassName}.md`,
			`- Enum: ${home}enums/{EnumName}.md`,
			`- Data type: ${home}index/types/{TypeName}.md`,
		].join("\n"),
		`Replace the placeholder with the exact name, such as ${inlineCode("Part")}, ${inlineCode("KeyCode")}, or ${inlineCode("CFrame")}. Names are case-sensitive. Enum URLs do not include the ${inlineCode("Enum.")} prefix.`,
		"## Indexes",
		[
			`- ${link("Classes", siteUrl(markdownHref(classesHref())))}: Every class, with its superclass and tags, such as Service and Deprecated`,
			`- ${link("Enums", siteUrl(markdownHref(enumsHref())))}: Every enum, with its tags`,
			`- ${link("Data types", siteUrl(markdownHref(typesHref())))}: Every data type, with its category`,
		].join("\n"),
		"## Optional",
		`- ${link("Sitemap", `${home}sitemap.xml`)}: Every HTML page`,
	])
}
