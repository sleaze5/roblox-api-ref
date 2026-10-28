import { isApiChange } from "../../../src/api/api-change.ts"
import type {
	HistoryEntry,
	HistoryEvent,
	HistoryKind,
	ItemHistory,
	MemberCounts,
	Removed,
	RemovedOwner,
} from "../../../src/api/item-history.ts"
import type { EnumItem, Member } from "../../../src/api/types.ts"
import type {
	ChangeDetail,
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

const deprecatedTag = "Deprecated"

function tagDetail(details: ChangeDetail[], labels: string[]): boolean {
	return details.some(
		(detail) =>
			detail.kind === "tags" &&
			labels.includes(detail.label) &&
			detail.tags.includes(deprecatedTag),
	)
}

export function entryDeprecation(entry: ChangeEntry): "set" | "clear" | undefined {
	if (entry.status === "Removed" || tagDetail(entry.details, ["Removed tags"])) {
		return "clear"
	}
	const born = entry.status === "Added" && entry.after?.tags.includes(deprecatedTag)
	return born || tagDetail(entry.details, ["Added tags"]) ? "set" : undefined
}

export function ownerDeprecation(owner: OwnerChanges): "set" | "clear" | undefined {
	if (owner.status === "Removed" || tagDetail(owner.details, ["Removed tags"])) {
		return "clear"
	}
	return tagDetail(owner.details, ["Tags", "Added tags"]) ? "set" : undefined
}

export class HistoryTable {
	readonly entries = new Map<string, HistoryEntry[]>()
	readonly deprecations = new Map<string, HistoryEvent>()
	readonly removed = new Map<string, Map<string, Removed<Member | EnumItem>>>()
	readonly removedOwners = new Map<
		string,
		RemovedOwner & { kind: OwnerChanges["kind"] }
	>()

	removedOwnersOf(kind: OwnerChanges["kind"]): RemovedOwner[] {
		return [...this.removedOwners.values()]
			.filter((owner) => owner.kind === kind)
			.map(({ name, removed }) => ({ name, removed }))
			.sort((a, b) => b.removed.published.localeCompare(a.removed.published))
	}

	private deprecate(
		key: string,
		change: "set" | "clear" | undefined,
		event: HistoryEvent,
	): void {
		if (change === "set") {
			this.deprecations.set(key, event)
		} else if (change === "clear") {
			this.deprecations.delete(key)
		}
	}

	private trackRemoval(owner: string, entry: ChangeEntry, event: HistoryEvent): void {
		const removed =
			this.removed.get(owner) ?? new Map<string, Removed<Member | EnumItem>>()
		if (entry.status === "Removed" && entry.before !== undefined) {
			removed.set(entry.name, { value: entry.before, removed: event })
		} else if (entry.status === "Added") {
			removed.delete(entry.name)
		}
		this.removed.set(owner, removed)
	}

	removedOf(owner: string): Removed<Member | EnumItem>[] {
		return [...(this.removed.get(owner)?.values() ?? [])].sort((a, b) =>
			b.removed.published.localeCompare(a.removed.published),
		)
	}

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
			const key = ownerKey(owner.kind, owner.name)
			if (owner.status === "Added") {
				this.removed.delete(key)
				this.removedOwners.delete(key)
			} else if (owner.status === "Removed") {
				this.removedOwners.set(key, {
					kind: owner.kind,
					name: owner.name,
					removed: event,
				})
			}
			this.deprecate(key, ownerDeprecation(owner), event)
			this.push(key, {
				...event,
				kinds: ownerKinds(owner),
				...(members === undefined ? {} : { members }),
			})
			for (const entry of owner.entries) {
				this.deprecate(entryKey(entry), entryDeprecation(entry), event)
				this.trackRemoval(key, entry, event)
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
		const deprecated = this.deprecations.get(key)
		if (deprecated !== undefined) {
			history.deprecated = deprecated
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
