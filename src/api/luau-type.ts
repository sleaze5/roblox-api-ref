import { linked, token, type CodeToken } from "./code-token.ts"
import { classHref, enumHref, typeHref } from "./routes.ts"
import type { ApiType } from "./types.ts"

const primitiveNames: Record<string, string> = {
	int: "number",
	int64: "number",
	float: "number",
	double: "number",
	bool: "boolean",
	Variant: "any",
	ContentId: "string",
	ProtectedString: "string",
	BinaryString: "string",
}

const aliasNames: Record<string, string> = {
	CoordinateFrame: "CFrame",
	OptionalCoordinateFrame: "CFrame",
}

const numberNames = new Set(["int", "int64", "float", "double", "number"])
const stringNames = new Set(["string", "ContentId", "ProtectedString", "BinaryString"])

export function isNumberType(type: ApiType): boolean {
	return numberNames.has(type.name)
}

export function isStringType(type: ApiType): boolean {
	return stringNames.has(type.name)
}

export function isBooleanType(type: ApiType): boolean {
	return type.name === "bool"
}

export function luauName(name: string): string {
	return primitiveNames[name] ?? aliasNames[name] ?? name
}

function groupTokens(name: string): CodeToken[] | undefined {
	switch (name) {
		case "null":
		case "void":
			return [token("()", "punctuation")]
		case "Array":
			return [
				token("{ ", "punctuation"),
				token("any", "type"),
				token(" }", "punctuation"),
			]
		case "Dictionary":
		case "Map":
			return [
				token("{ [", "punctuation"),
				token(name === "Dictionary" ? "string" : "any", "type"),
				token("]: ", "punctuation"),
				token("any", "type"),
				token(" }", "punctuation"),
			]
		case "Objects":
		case "Instances":
			return [
				token("{ ", "punctuation"),
				token("Instance", "type"),
				token(" }", "punctuation"),
			]
		case "Function":
			return [
				token("(...", "punctuation"),
				token("any", "type"),
				token(") -> ...", "punctuation"),
				token("any", "type"),
			]
	}
	return undefined
}

function tupleTokens(type: ApiType): CodeToken[] {
	const members = type.members ?? []
	if (members.length === 0) {
		return linked(
			[token("...", "punctuation"), token("any", "type")],
			typeHref("Tuple"),
		)
	}
	const tokens: CodeToken[] = [token("(", "punctuation")]
	members.forEach((member, index) => {
		if (index > 0) {
			tokens.push(token(", ", "punctuation"))
		}
		tokens.push(...typeTokens(member))
	})
	tokens.push(token(")", "punctuation"))
	return tokens
}

function bodyTokens(type: ApiType): CodeToken[] {
	if (type.name === "Tuple") {
		return tupleTokens(type)
	}
	if (type.category === "Class") {
		return [token(type.name, "type", classHref(type.name))]
	}
	if (type.category === "Enum") {
		return linked(
			[token("Enum.", "punctuation"), token(type.name, "type")],
			enumHref(type.name),
		)
	}
	const title = luauName(type.name) === type.name ? undefined : type.name
	const group = groupTokens(type.name)
	if (group !== undefined) {
		return linked(group, typeHref(type.name), type.name)
	}
	return linked([token(luauName(type.name), "type")], typeHref(type.name), title)
}

export function typeTokens(type: ApiType): CodeToken[] {
	const optional = type.optional || type.name === "OptionalCoordinateFrame"
	const body = bodyTokens(type)
	if (!optional) {
		return body
	}
	if (type.name === "Function") {
		return [token("(", "punctuation"), ...body, token(")?", "punctuation")]
	}
	return [...body, token("?", "punctuation")]
}

export function typeText(type: ApiType): string {
	return typeTokens(type)
		.map((part) => part.text)
		.join("")
}
