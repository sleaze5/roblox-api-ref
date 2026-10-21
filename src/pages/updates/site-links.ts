import type { CodeToken } from "../../api/code-token.ts"
import {
	classHref,
	enumHref,
	enumItemHref,
	memberHref,
	typeHref,
} from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"

const cache = new WeakMap<Api, Set<string>>()

function collectLinks(api: Api): Set<string> {
	const links = new Set<string>()
	for (const item of Object.values(api.classes)) {
		links.add(classHref(item.name))
		for (const member of item.members) {
			links.add(memberHref(item.name, member.kind, member.name))
		}
	}
	for (const item of Object.values(api.enums)) {
		links.add(enumHref(item.name))
		for (const entry of item.items) {
			links.add(enumItemHref(item.name, entry.name))
		}
	}
	for (const name of Object.keys(api.dataTypes)) {
		links.add(typeHref(name))
	}
	return links
}

export function siteLinks(api: Api): Set<string> {
	let links = cache.get(api)
	if (links === undefined) {
		links = collectLinks(api)
		cache.set(api, links)
	}
	return links
}

export function liveTokens(tokens: CodeToken[], links: Set<string>): CodeToken[] {
	return tokens.map((part) => {
		if (part.href === undefined || links.has(part.href)) {
			return part
		}
		const copy = { ...part }
		delete copy.href
		return copy
	})
}
