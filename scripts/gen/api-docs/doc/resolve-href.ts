import type { DocLink } from "../../../../src/api/doc-types.ts"
import { enumLink, memberLink, typeLink, type ApiNames } from "./api-names.ts"

const docsOrigin = "https://create.roblox.com"
const referenceBase = `${docsOrigin}/docs/reference/engine/classes/`
const referencePattern = /^\/docs\/reference\/engine\/(classes|enums|datatypes)\/(\w+)$/

function absoluteUrl(href: string): URL | undefined {
	const source =
		href.startsWith("/") && !href.startsWith("//") ? `/docs${href}` : href
	try {
		const url = new URL(source, referenceBase)
		return url.protocol === "https:" || url.protocol === "http:" ? url : undefined
	} catch {
		return undefined
	}
}

function docsPath(pathname: string): string {
	return pathname
		.replace(/^\/docs\/en-us\//, "/docs/")
		.replace(/\.md$/, "")
		.replace(/\/index$/, "")
}

function referenceLink(
	names: ApiNames,
	section: string,
	name: string,
	anchor: string,
): DocLink | undefined {
	switch (section) {
		case "classes":
			return (
				(anchor === "" ? undefined : memberLink(names, name, anchor)) ??
				typeLink(names, name)
			)
		case "enums":
			return (
				enumLink(names, name, anchor === "" ? undefined : anchor) ??
				enumLink(names, name)
			)
		default:
			return Object.hasOwn(names.dataTypes, name)
				? { kind: "type", name }
				: undefined
	}
}

export function resolveHref(names: ApiNames, href: string): DocLink | undefined {
	const url = absoluteUrl(href.trim())
	if (url === undefined) {
		return undefined
	}
	if (url.origin === docsOrigin) {
		url.pathname = docsPath(url.pathname)
		const match = referencePattern.exec(url.pathname)
		const internal =
			match === null
				? undefined
				: referenceLink(
						names,
						match[1] ?? "",
						match[2] ?? "",
						decodeURIComponent(url.hash.slice(1)),
					)
		if (internal !== undefined) {
			return internal
		}
	}
	return { kind: "external", url: url.href }
}
