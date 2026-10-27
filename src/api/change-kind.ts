import { inlineTokens, signatureParts } from "./signature.ts"
import type { EnumItem, Member } from "./types.ts"

export function signatureText(owner: string, value: Member | EnumItem): string {
	if (!("kind" in value)) {
		return `Enum.${owner}.${value.name} = ${value.value}`
	}
	return inlineTokens(signatureParts(value, owner))
		.map((part) => part.text)
		.join("")
}
