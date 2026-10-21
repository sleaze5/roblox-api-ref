import { token, unlinked, type CodeToken } from "../../api/code-token.ts"
import { enumHref, enumItemHref, memberHref } from "../../api/routes.ts"
import {
	inlineTokens,
	signatureParts,
	type SignatureParts,
} from "../../api/signature.ts"
import type { Api, EnumItem, Member } from "../../api/types.ts"
import { liveTokens, siteLinks } from "./site-links.ts"

function mapParts(
	parts: SignatureParts,
	map: (tokens: CodeToken[]) => CodeToken[],
): SignatureParts {
	const copy: SignatureParts = {
		owner: map(parts.owner),
		head: map(parts.head),
		tail: map(parts.tail),
	}
	if (parts.parameters !== undefined) {
		copy.parameters = parts.parameters.map(map)
	}
	return copy
}

function enumItemParts(owner: string, item: EnumItem): SignatureParts {
	return {
		owner: [
			token("Enum.", "punctuation"),
			token(owner, "type", enumHref(owner)),
			token(".", "punctuation"),
		],
		head: [token(item.name, "constant", enumItemHref(owner, item.name))],
		tail: [token(" = ", "punctuation"), token(String(item.value), "constant")],
	}
}

export function changeParts(
	api: Api,
	owner: string,
	value: Member | EnumItem,
	linked: boolean,
): SignatureParts {
	const parts =
		"kind" in value
			? signatureParts(value, owner, memberHref(owner, value.kind, value.name))
			: enumItemParts(owner, value)
	if (!linked) {
		return mapParts(parts, unlinked)
	}
	const links = siteLinks(api)
	return mapParts(parts, (tokens) => liveTokens(tokens, links))
}

export function partsText(parts: SignatureParts): string {
	return inlineTokens(parts)
		.map((part) => part.text)
		.join("")
}
