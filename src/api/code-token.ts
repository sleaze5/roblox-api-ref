export type CodeTone =
	| "self"
	| "type"
	| "constant"
	| "string"
	| "function"
	| "name"
	| "parameter"
	| "punctuation"
	| "plain"

export type CodeToken = {
	text: string
	tone: CodeTone
	href?: string
	title?: string
	swatch?: string
}

export function token(text: string, tone: CodeTone, href?: string): CodeToken {
	return href === undefined ? { text, tone } : { text, tone, href }
}

export function linked(tokens: CodeToken[], href: string, title?: string): CodeToken[] {
	return tokens.map((part) =>
		title === undefined ? { ...part, href } : { ...part, href, title },
	)
}

export function textLength(tokens: CodeToken[]): number {
	return tokens.reduce((length, part) => length + part.text.length, 0)
}

export function unlinked(tokens: CodeToken[]): CodeToken[] {
	return tokens.map((part) => {
		const copy = { ...part }
		delete copy.href
		return copy
	})
}
