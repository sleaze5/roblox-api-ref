import type { DocLink } from "../../../../src/api/doc-types.ts"
import { enumLink, memberLink, typeLink, type ApiNames } from "./api-names.ts"

export type CodeContext = { kind: "class" | "enum"; name: string }

const enumPattern = /^Enum\.(\w+)(?:\.(\w+))?$/
const memberPattern = /^(\w+)[.:](\w+)$/
const namePattern = /^\w+$/

const callPattern = /\(\)$/

function contextLink(
	names: ApiNames,
	name: string,
	call: boolean,
	context: CodeContext | undefined,
): DocLink | undefined {
	if (context?.kind === "enum") {
		return call ? undefined : enumLink(names, context.name, name)
	}
	const link =
		context === undefined ? undefined : memberLink(names, context.name, name)
	return call && link?.kind === "member" && link.member !== "Method"
		? undefined
		: link
}

export function resolveCode(
	names: ApiNames,
	text: string,
	context?: CodeContext,
): DocLink | undefined {
	const trimmed = text.trim()
	const name = trimmed.replace(callPattern, "")
	const enumMatch = enumPattern.exec(name)
	if (enumMatch !== null) {
		return enumLink(names, enumMatch[1] ?? "", enumMatch[2])
	}
	const memberMatch = memberPattern.exec(name)
	if (memberMatch !== null) {
		return memberLink(names, memberMatch[1] ?? "", memberMatch[2] ?? "")
	}
	if (!namePattern.test(name)) {
		return undefined
	}
	return (
		typeLink(names, name) ??
		contextLink(names, name, callPattern.test(trimmed), context)
	)
}
