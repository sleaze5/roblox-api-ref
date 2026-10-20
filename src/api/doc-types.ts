import type { MemberKind } from "./types.ts"

export type DocLink =
	| { kind: "class"; name: string }
	| { kind: "member"; owner: string; member: MemberKind; name: string }
	| { kind: "enum"; name: string }
	| { kind: "enumItem"; owner: string; name: string }
	| { kind: "type"; name: string }
	| { kind: "external"; url: string }

export const docTags = [
	"a",
	"br",
	"code",
	"em",
	"kbd",
	"li",
	"pre",
	"strong",
	"sup",
	"ul",
] as const

export type DocTag = (typeof docTags)[number]

export type DocElement = {
	tag: DocTag
	children: DocNode[]
	link?: DocLink
}

export type DocNode = string | DocElement

export type Doc = DocNode[]
