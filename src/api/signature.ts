import { linked, token, type CodeToken } from "./code-token.ts"
import { defaultValueTokens } from "./default-value.ts"
import { typeTokens } from "./luau-type.ts"
import { classHref, typeHref } from "./routes.ts"
import type { ApiType, Member, Parameter, Property } from "./types.ts"

export type SignatureParts = {
	owner: CodeToken[]
	head: CodeToken[]
	parameters?: CodeToken[][]
	tail: CodeToken[]
}

export function parameterTokens(parameter: Parameter): CodeToken[] {
	if (parameter.type.name === "Tuple") {
		return linked(
			[
				token("...", "punctuation"),
				token(": ", "punctuation"),
				token("any", "type"),
			],
			typeHref("Tuple"),
		)
	}
	const type =
		parameter.default === undefined || parameter.type.optional
			? parameter.type
			: { ...parameter.type, optional: true }
	const tokens = [
		token(parameter.name, "parameter"),
		token(": ", "punctuation"),
		...typeTokens(type),
	]
	if (parameter.default !== undefined) {
		tokens.push(
			token(" = ", "punctuation"),
			...defaultValueTokens(parameter.type, parameter.default),
		)
	}
	return tokens
}

export function isNothing(type: ApiType): boolean {
	return (
		type.category === "Primitive" && (type.name === "null" || type.name === "void")
	)
}

function ownerTokens(owner: string, separator: "." | ":"): CodeToken[] {
	return [token(owner, "self", classHref(owner)), token(separator, "punctuation")]
}

function propertyTail(member: Property): CodeToken[] {
	if (member.default !== undefined) {
		return [
			token(" = ", "punctuation"),
			...defaultValueTokens(member.type, member.default),
		]
	}
	return []
}

export function signatureParts(
	member: Member,
	owner: string,
	nameHref?: string,
): SignatureParts {
	switch (member.kind) {
		case "Property":
			return {
				owner: ownerTokens(owner, "."),
				head: [
					token(member.name, "name", nameHref),
					token(": ", "punctuation"),
					...typeTokens(member.type),
				],
				tail: propertyTail(member),
			}
		case "Method":
			return {
				owner: ownerTokens(owner, ":"),
				head: [
					token(member.name, "function", nameHref),
					token("(", "punctuation"),
				],
				parameters: member.parameters.map(parameterTokens),
				tail: [
					token(")", "punctuation"),
					...(isNothing(member.returns)
						? []
						: [token(": ", "punctuation"), ...typeTokens(member.returns)]),
				],
			}
		case "Event":
			return {
				owner: ownerTokens(owner, "."),
				head: [
					token(member.name, "name", nameHref),
					token(": ", "punctuation"),
					token("RBXScriptSignal", "type", typeHref("RBXScriptSignal")),
					token("<(", "punctuation"),
				],
				parameters: member.parameters.map(parameterTokens),
				tail: [token(")>", "punctuation")],
			}
		case "Callback":
			return {
				owner: ownerTokens(owner, "."),
				head: [
					token(member.name, "name", nameHref),
					token(": (", "punctuation"),
				],
				parameters: member.parameters.map(parameterTokens),
				tail: [token(") -> ", "punctuation"), ...typeTokens(member.returns)],
			}
	}
}

export function inlineTokens(parts: SignatureParts): CodeToken[] {
	const tokens = [...parts.owner, ...parts.head]
	parts.parameters?.forEach((parameter, index) => {
		if (index > 0) {
			tokens.push(token(", ", "punctuation"))
		}
		tokens.push(...parameter)
	})
	tokens.push(...parts.tail)
	return tokens
}
