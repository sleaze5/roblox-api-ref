import type { ItemHistory, Removed } from "../../../src/api/item-history.ts"
import type { Api, EnumItem, Member } from "../../../src/api/types.ts"
import { itemKey, ownerKey, type HistoryTable } from "./history-table.ts"

export function applyHistory(api: Api, table: HistoryTable, start: string): void {
	api.historyStart = start
	api.removed = {
		classes: table
			.removedOwnersOf("Class")
			.filter((owner) => !Object.hasOwn(api.classes, owner.name)),
		enums: table
			.removedOwnersOf("Enum")
			.filter((owner) => !Object.hasOwn(api.enums, owner.name)),
	}
	const attach = (item: { history?: ItemHistory }, key: string) => {
		const history = table.history(key)
		if (history === undefined) {
			return
		}
		if (history.added === undefined) {
			history.addedBefore = start
		}
		item.history = history
	}
	for (const item of Object.values(api.classes)) {
		attach(item, ownerKey("Class", item.name))
		const removed = table.removedOf(
			ownerKey("Class", item.name),
		) as Removed<Member>[]
		if (removed.length > 0) {
			item.removedMembers = removed
		}
		for (const member of item.members) {
			attach(member, itemKey("Member", item.name, member.name))
		}
	}
	for (const item of Object.values(api.enums)) {
		attach(item, ownerKey("Enum", item.name))
		const removed = table.removedOf(
			ownerKey("Enum", item.name),
		) as Removed<EnumItem>[]
		if (removed.length > 0) {
			item.removedItems = removed
		}
		for (const enumItem of item.items) {
			attach(enumItem, itemKey("EnumItem", item.name, enumItem.name))
		}
	}
}
