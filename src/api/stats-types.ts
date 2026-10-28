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
	| { kind: "Removed"; label: string; version: string; anchor: string }

export type StatRecord = {
	id: string
	label: string
	hint: string
	value: number
	unit?: string
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

export type UpdateEvent = {
	version: string
	published: string
}

export type ComebackBatch = {
	removed: UpdateEvent
	returned: UpdateEvent
	count: number
	days: number
}

export type AccessPeriod = {
	period: string
	addedOpen: number
	addedRestricted: number
	opened: number
	restricted: number
	madeSafe: number
	madeUnsafe: number
}

export type AccessNow = {
	members: number
	scriptable: number
	threadSafe: number
}

export type TagTrend = {
	name: string
	current: number
	gained: number
	lost: number
	first?: UpdateEvent
}

export const coverageKinds = [
	"Class",
	"Property",
	"Method",
	"Event",
	"Callback",
	"Parameter",
	"Return",
	"Enum",
	"EnumItem",
] as const

export type CoverageKind = (typeof coverageKinds)[number]

export type CoverageRow = {
	kind: CoverageKind
	described: number
	total: number
}

export type DocPeriod = {
	period: string
	added: number
	changed: number
	removed: number
}

export type GrowthPeriod = {
	period: string
	classes: { name: string; added: number }[]
}

export type Stats = {
	historyStart: string
	counts: CountRow[]
	records: RecordGroup[]
	updates: UpdateTrend[]
	years: PeriodTrend[]
	months: PeriodTrend[]
	comebacks: ComebackBatch[]
	access: { now: AccessNow; years: AccessPeriod[] }
	tags: TagTrend[]
	capabilities: TagTrend[]
	docs: { coverage: CoverageRow[]; years: DocPeriod[] }
	growth: GrowthPeriod[]
}
