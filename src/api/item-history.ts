export type HistoryEvent = {
	version: string
	published: string
}

export type ItemHistory = {
	added?: HistoryEvent
	changed?: HistoryEvent
	described?: HistoryEvent
	recent?: "added" | "changed"
}
