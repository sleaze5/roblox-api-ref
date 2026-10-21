import type { Doc } from "./doc-types.ts"
import type { Release } from "./release.ts"
import type { EnumItem, Member } from "./types.ts"

export const changeStatuses = ["Added", "Removed", "Changed"] as const

export type ChangeStatus = (typeof changeStatuses)[number]

export type DetailTone = "text" | "code" | "type"

export type DetailValue = {
	text: string
	tone: DetailTone
}

export type ChangeDetail =
	| { kind: "value"; label: string; before?: DetailValue; after?: DetailValue }
	| { kind: "tags"; label: string; tags: string[] }

export type DescriptionPart = "description" | "parameter" | "return"

export type DescriptionDetail = {
	part: DescriptionPart
	parameter?: string
	status: ChangeStatus
	before?: Doc
	after?: Doc
}

type EntryBase = {
	status: ChangeStatus
	owner: string
	name: string
	details: ChangeDetail[]
	descriptions: DescriptionDetail[]
}

export type MemberEntry = EntryBase & {
	kind: "Member"
	before?: Member
	after?: Member
}

export type EnumItemEntry = EntryBase & {
	kind: "EnumItem"
	before?: EnumItem
	after?: EnumItem
}

export type ChangeEntry = MemberEntry | EnumItemEntry

export type OwnerKind = "Class" | "Enum"

export type OwnerChanges = {
	kind: OwnerKind
	name: string
	status?: ChangeStatus
	details: ChangeDetail[]
	descriptions: DescriptionDetail[]
	entries: ChangeEntry[]
}

export type ChangeCounts = Record<ChangeStatus, number>

export type Update = Release & {
	published: string
	previous: Release
	counts: ChangeCounts
	descriptionCount: number
	owners: OwnerChanges[]
}
