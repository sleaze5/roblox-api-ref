import type { Release } from "./release.ts"
import type { MemberKind } from "./types.ts"

export const countKinds = [
	"Class",
	"Property",
	"Method",
	"Event",
	"Callback",
	"Enum",
	"EnumItem",
	"DataType",
] as const

export type CountKind = (typeof countKinds)[number]

export type CountRow = {
	kind: CountKind
	current: number
	removed: number
}

export type StatRef =
	| { kind: "Class" | "Enum" | "DataType"; name: string }
	| { kind: MemberKind | "EnumItem"; owner: string; name: string }
	| {
			kind: "Parameter"
			owner: string
			member: string
			memberKind: MemberKind
			name: string
	  }
	| { kind: "Update"; version: string; published: string }

export type StatRecord = {
	id: string
	label: string
	hint: string
	value: number
	items: StatRef[]
}

export type RecordGroup = {
	id: string
	title: string
	records: StatRecord[]
}

export const trendKinds = ["Class", "Member", "Enum", "EnumItem"] as const

export type TrendKind = (typeof trendKinds)[number]

export type ChangeTally = {
	added: number
	removed: number
	changed: number
	deprecated: number
}

export type Tallies = Record<TrendKind, ChangeTally>

export type UpdateTrend = Release & {
	published: string
	tallies: Tallies
	classes: number
	enums: number
}

export type PeriodTrend = {
	period: string
	updates: number
	tallies: Tallies
}

export type Stats = {
	historyStart: string
	counts: CountRow[]
	records: RecordGroup[]
	updates: UpdateTrend[]
	years: PeriodTrend[]
	months: PeriodTrend[]
}
