export const historyKinds = ["added", "removed", "changed", "described"] as const

export type HistoryKind = (typeof historyKinds)[number]

export type MemberCounts = {
	added: number
	removed: number
	changed: number
}

export type HistoryEntry = {
	version: string
	published: string
	kinds: HistoryKind[]
	members?: MemberCounts
}

export type HistoryEvent = {
	version: string
	published: string
}

export type ItemHistory = {
	entries: HistoryEntry[]
	added?: HistoryEvent
	addedBefore?: string
	changed?: HistoryEvent
	deprecated?: HistoryEvent
}

export type RemovedOwner = {
	name: string
	removed: HistoryEvent
}

export type Removed<Value> = {
	value: Value
	removed: HistoryEvent
}
