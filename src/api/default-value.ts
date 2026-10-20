import { linked, token, type CodeToken } from "./code-token.ts"
import { isBooleanType, isNumberType, isStringType, luauName } from "./luau-type.ts"
import { enumHref, enumItemHref, typeHref } from "./routes.ts"
import type { ApiType } from "./types.ts"

const constructorTypes = new Set([
	"Vector2",
	"Vector3",
	"Vector2int16",
	"Vector3int16",
	"Color3",
	"Rect",
	"UDim",
	"UDim2",
	"CFrame",
	"CoordinateFrame",
	"NumberRange",
])

const identityCFrame = "0,0,0,1,0,0,0,1,0,0,0,1"

function unquote(value: string): string {
	return value.length >= 2 && value.startsWith('"') && value.endsWith('"')
		? value.slice(1, -1)
		: value
}

function numbersIn(value: string): string[] {
	return value
		.replaceAll(/[{}]/g, " ")
		.split(/[\s,]+/)
		.filter((part) => part.length > 0)
}

function typeNameToken(typeName: string): CodeToken {
	return { ...token(luauName(typeName), "type"), href: typeHref(typeName) }
}

function constructorTokens(typeName: string, args: string[]): CodeToken[] {
	const tokens: CodeToken[] = [
		typeNameToken(typeName),
		token(".", "punctuation"),
		token("new", "function"),
		token("(", "punctuation"),
	]
	args.forEach((arg, index) => {
		if (index > 0) {
			tokens.push(token(", ", "punctuation"))
		}
		tokens.push(token(arg, "constant"))
	})
	tokens.push(token(")", "punctuation"))
	return tokens
}

function staticTokens(typeName: string, member: string): CodeToken[] {
	return [typeNameToken(typeName), token(".", "punctuation"), token(member, "name")]
}

function colorSwatch(args: string[]): string | undefined {
	if (args.length !== 3) {
		return undefined
	}
	const channels = args.map((arg) => Math.round(Number(arg) * 255))
	if (channels.some((channel) => Number.isNaN(channel))) {
		return undefined
	}
	return `rgb(${channels.join(" ")})`
}

function isVectorType(typeName: string): boolean {
	return typeName === "Vector3" || typeName === "Vector2"
}

function shorthandTokens(typeName: string, args: string[]): CodeToken[] | undefined {
	if (luauName(typeName) === "CFrame" && args.join(",") === identityCFrame) {
		return staticTokens(typeName, "identity")
	}
	if (isVectorType(typeName) && args.every((arg) => arg === "0")) {
		return staticTokens(typeName, "zero")
	}
	if (isVectorType(typeName) && args.every((arg) => arg === "1")) {
		return staticTokens(typeName, "one")
	}
	if (typeName === "NumberRange" && args.length === 2 && args[0] === args[1]) {
		return constructorTokens(typeName, args.slice(0, 1))
	}
	return undefined
}

function dataTypeTokens(typeName: string, raw: string): CodeToken[] {
	const args = numbersIn(raw)
	if (args.some((arg) => Number.isNaN(Number(arg)))) {
		return [token(raw, "plain")]
	}
	const shorthand = shorthandTokens(typeName, args)
	if (shorthand !== undefined) {
		return shorthand
	}
	const tokens = constructorTokens(typeName, args)
	const swatch = typeName === "Color3" ? colorSwatch(args) : undefined
	const first = tokens[0]
	if (swatch !== undefined && first !== undefined) {
		tokens[0] = { ...first, swatch }
	}
	return tokens
}

function enumTokens(enumName: string, item: string): CodeToken[] {
	return [
		...linked(
			[token("Enum.", "punctuation"), token(enumName, "type")],
			enumHref(enumName),
		),
		...linked(
			[token(".", "punctuation"), token(item, "constant")],
			enumItemHref(enumName, item),
		),
	]
}

export function defaultValueTokens(type: ApiType, raw: string): CodeToken[] {
	const value = raw.trim()
	if (value === "nil") {
		return [token("nil", "constant")]
	}
	if (type.category === "Enum") {
		return enumTokens(type.name, unquote(value))
	}
	if (isStringType(type)) {
		return [token(JSON.stringify(unquote(raw)), "string")]
	}
	if (isBooleanType(type)) {
		return [token(value.toLowerCase(), "constant")]
	}
	if (isNumberType(type)) {
		return [token(value, "constant")]
	}
	if (type.category === "DataType" && constructorTypes.has(type.name)) {
		return dataTypeTokens(type.name, value)
	}
	return [token(value, "plain")]
}
