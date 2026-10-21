import { token, unlinked } from "../../api/code-token.ts"
import { enumHref, enumItemHref, memberHref } from "../../api/routes.ts"
import {
	inlineTokens,
	signatureParts,
	type SignatureParts,
} from "../../api/signature.ts"
import type { Api, EnumItem, Member } from "../../api/types.ts"

function unlinkedParts(parts: SignatureParts): SignatureParts {
	const copy: SignatureParts = {
		owner: unlinked(parts.owner),
		head: unlinked(parts.head),
		tail: unlinked(parts.tail),
	}
	if (parts.parameters !== undefined) {
		copy.parameters = parts.parameters.map(unlinked)
	}
	return copy
}

function memberParts(api: Api, owner: string, member: Member): SignatureParts {
	const exists = api.classes[owner]?.members.some(
		(current) => current.name === member.name && current.kind === member.kind,
	)
	return signatureParts(
		member,
		owner,
		exists ? memberHref(owner, member.kind, member.name) : undefined,
	)
}

function enumItemParts(api: Api, owner: string, item: EnumItem): SignatureParts {
	const current = api.enums[owner]
	const exists = current?.items.some((entry) => entry.name === item.name)
	return {
		owner: [
			token("Enum.", "punctuation"),
			token(owner, "type", current === undefined ? undefined : enumHref(owner)),
			token(".", "punctuation"),
		],
		head: [
			token(
				item.name,
				"constant",
				exists ? enumItemHref(owner, item.name) : undefined,
			),
		],
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
			? memberParts(api, owner, value)
			: enumItemParts(api, owner, value)
	return linked ? parts : unlinkedParts(parts)
}

export function partsText(parts: SignatureParts): string {
	return inlineTokens(parts)
		.map((part) => part.text)
		.join("")
}
