import type { ItemHistory } from "../../../src/api/item-history.ts"
import type { Api } from "../../../src/api/types.ts"
import {
	classKey,
	enumItemKey,
	enumKey,
	memberKey,
	type HistoryTable,
} from "./history-table.ts"

const recentDays = 30
const dayMs = 24 * 60 * 60 * 1000

function isRecent(published: string | undefined, now: number): boolean {
	return published !== undefined && now - Date.parse(published) <= recentDays * dayMs
}

function withRecent(history: ItemHistory, now: number): ItemHistory {
	if (isRecent(history.added?.published, now)) {
		return Object.assign({}, history, { recent: "added" as const })
	}
	return isRecent(history.changed?.published, now)
		? Object.assign({}, history, { recent: "changed" as const })
		: history
}

export function applyHistory(api: Api, table: HistoryTable, start: string): void {
	api.historyStart = start
	const now = Date.parse(api.published)
	const attach = (item: { history?: ItemHistory }, key: string) => {
		const history = table.items.get(key)
		if (history !== undefined && Object.keys(history).length > 0) {
			item.history = withRecent(history, now)
		}
	}
	for (const item of Object.values(api.classes)) {
		attach(item, classKey(item.name))
		for (const member of item.members) {
			attach(member, memberKey(item.name, member.name))
		}
	}
	for (const item of Object.values(api.enums)) {
		attach(item, enumKey(item.name))
		for (const enumItem of item.items) {
			attach(enumItem, enumItemKey(item.name, enumItem.name))
		}
	}
}
