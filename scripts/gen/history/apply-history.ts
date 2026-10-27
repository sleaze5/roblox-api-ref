import type { ItemHistory } from "../../../src/api/item-history.ts"
import type { Api } from "../../../src/api/types.ts"
import { itemKey, ownerKey, type HistoryTable } from "./history-table.ts"

export function applyHistory(api: Api, table: HistoryTable, start: string): void {
	api.historyStart = start
	const attach = (item: { history?: ItemHistory }, key: string) => {
		const history = table.history(key)
		if (history !== undefined) {
			item.history = history
		}
	}
	for (const item of Object.values(api.classes)) {
		attach(item, ownerKey("Class", item.name))
		for (const member of item.members) {
			attach(member, itemKey("Member", item.name, member.name))
		}
	}
	for (const item of Object.values(api.enums)) {
		attach(item, ownerKey("Enum", item.name))
		for (const enumItem of item.items) {
			attach(enumItem, itemKey("EnumItem", item.name, enumItem.name))
		}
	}
}
