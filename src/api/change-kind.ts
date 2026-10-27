import { inlineTokens, signatureParts } from "./signature.ts"
import type { EnumItem, Member } from "./types.ts"
import type { ChangeEntry } from "./update-types.ts"

export function signatureText(owner: string, value: Member | EnumItem): string {
	if (!("kind" in value)) {
		return `Enum.${owner}.${value.name} = ${value.value}`
	}
	return inlineTokens(signatureParts(value, owner))
		.map((part) => part.text)
		.join("")
}

export function signatureChanged(entry: ChangeEntry): boolean {
	return (
		entry.before !== undefined &&
		entry.after !== undefined &&
		signatureText(entry.owner, entry.before) !==
			signatureText(entry.owner, entry.after)
	)
}

export function isApiChange(entry: ChangeEntry): boolean {
	return (
		entry.status !== "Changed" ||
		entry.details.length > 0 ||
		signatureChanged(entry)
	)
}
