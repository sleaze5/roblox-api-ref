import { isApiChange } from "../../../src/api/change-kind.ts"
import type {
	HistoryEntry,
	HistoryKind,
	ItemHistory,
	MemberCounts,
} from "../../../src/api/item-history.ts"
import type {
	ChangeEntry,
	ChangeStatus,
	OwnerChanges,
	Update,
} from "../../../src/api/update-types.ts"

const statusKinds: Record<ChangeStatus, HistoryKind> = {
	Added: "added",
	Removed: "removed",
	Changed: "changed",
}

export function ownerKey(kind: OwnerChanges["kind"], name: string): string {
	return `${kind}:${name}`
}

export function itemKey(
	kind: ChangeEntry["kind"],
	owner: string,
	name: string,
): string {
	return `${kind}:${owner}.${name}`
}

function entryKey(entry: ChangeEntry): string {
	return itemKey(entry.kind, entry.owner, entry.name)
}

function entryKinds(entry: ChangeEntry): HistoryKind[] {
	return [
		...(isApiChange(entry) ? [statusKinds[entry.status]] : []),
		...(entry.descriptions.length > 0 ? (["described"] as const) : []),
	]
}

function ownerKinds(owner: OwnerChanges): HistoryKind[] {
	return [
		...(owner.status === undefined ? [] : [statusKinds[owner.status]]),
		...(owner.descriptions.length > 0 ? (["described"] as const) : []),
	]
}

function memberCounts(owner: OwnerChanges): MemberCounts | undefined {
	if (owner.status === "Added") {
		return undefined
	}
	const counts: MemberCounts = { added: 0, removed: 0, changed: 0 }
	for (const entry of owner.entries) {
		if (isApiChange(entry)) {
			counts[statusKinds[entry.status] as keyof MemberCounts]++
		}
	}
	return counts.added + counts.removed + counts.changed === 0 ? undefined : counts
}

export class HistoryTable {
	readonly entries = new Map<string, HistoryEntry[]>()

	private push(key: string, entry: HistoryEntry): void {
		if (entry.kinds.length === 0 && entry.members === undefined) {
			return
		}
		const list = this.entries.get(key) ?? []
		list.push(entry)
		this.entries.set(key, list)
	}

	apply(update: Update): void {
		const event = { version: update.version, published: update.published }
		for (const owner of update.owners) {
			const members = memberCounts(owner)
			this.push(ownerKey(owner.kind, owner.name), {
				...event,
				kinds: ownerKinds(owner),
				...(members === undefined ? {} : { members }),
			})
			for (const entry of owner.entries) {
				this.push(entryKey(entry), { ...event, kinds: entryKinds(entry) })
			}
		}
	}

	history(key: string): ItemHistory | undefined {
		const entries = this.entries.get(key)
		if (entries === undefined) {
			return undefined
		}
		const addedIndex = entries.findLastIndex((entry) =>
			entry.kinds.includes("added"),
		)
		const changedIndex = entries.findLastIndex(
			(entry) => entry.kinds.includes("changed") || entry.members !== undefined,
		)
		const history: ItemHistory = { entries: entries.toReversed() }
		const added = entries[addedIndex]
		const changed = changedIndex > addedIndex ? entries[changedIndex] : undefined
		if (added !== undefined) {
			history.added = { version: added.version, published: added.published }
		}
		if (changed !== undefined) {
			history.changed = { version: changed.version, published: changed.published }
		}
		return history
	}
}

export function buildHistory(updates: Update[]): HistoryTable {
	const table = new HistoryTable()
	for (const update of updates.toReversed()) {
		table.apply(update)
	}
	return table
}
