import { linked, token, type CodeToken } from "./code-token.ts"
import {
	classHref,
	enumHref,
	enumItemHref,
	memberHref,
	typeHref,
	updateHref,
} from "./routes.ts"
import { memberSeparator } from "./stat-ref-label.ts"
import type { StatRef } from "./stats-types.ts"

export function statRefHref(ref: StatRef): string {
	switch (ref.kind) {
		case "Class":
			return classHref(ref.name)
		case "DataType":
			return typeHref(ref.name)
		case "Enum":
			return enumHref(ref.name)
		case "EnumItem":
			return enumItemHref(ref.owner, ref.name)
		case "Parameter":
			return memberHref(ref.owner, ref.memberKind, ref.member)
		case "Update":
			return updateHref(ref.version)
		case "Removed":
			return `${updateHref(ref.version)}#${ref.anchor}`
		default:
			return memberHref(ref.owner, ref.kind, ref.name)
	}
}

function memberTokens(owner: string, kind: string, name: string): CodeToken[] {
	return [
		token(owner, "self"),
		token(memberSeparator(kind), "punctuation"),
		token(name, kind === "Method" ? "function" : "name"),
	]
}

function unlinkedTokens(ref: StatRef): CodeToken[] {
	switch (ref.kind) {
		case "Class":
		case "DataType":
			return [token(ref.name, "type")]
		case "Enum":
			return [token("Enum.", "punctuation"), token(ref.name, "type")]
		case "EnumItem":
			return [
				token("Enum.", "punctuation"),
				token(ref.owner, "type"),
				token(".", "punctuation"),
				token(ref.name, "constant"),
			]
		case "Parameter":
			return [
				...memberTokens(ref.owner, ref.memberKind, ref.member),
				token("(", "punctuation"),
				token(ref.name, "parameter"),
				token(")", "punctuation"),
			]
		case "Update":
			return [token(ref.version, "plain")]
		case "Removed":
			return [token(ref.label, "self")]
		default:
			return memberTokens(ref.owner, ref.kind, ref.name)
	}
}

export function statRefTokens(ref: StatRef): CodeToken[] {
	return linked(unlinkedTokens(ref), statRefHref(ref))
}
