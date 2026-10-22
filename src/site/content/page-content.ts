export type InlineLink = { text: string; href: string }

export type InlineCode = { code: string }

export type Inline = string | InlineLink | InlineCode

export type Block =
	| { kind: "paragraph"; content: Inline[] }
	| { kind: "list"; items: Inline[][] }

export type ContentSection = {
	id: string
	heading: string
	blocks: Block[]
}

export type PageContent = {
	lead: string
	sections: ContentSection[]
}

export function paragraph(...content: Inline[]): Block {
	return { kind: "paragraph", content }
}

export function list(...items: Inline[][]): Block {
	return { kind: "list", items }
}

export function isExternal(href: string): boolean {
	return /^https?:/.test(href)
}
