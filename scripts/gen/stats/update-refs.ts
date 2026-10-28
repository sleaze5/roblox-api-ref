import type { StatRef, UpdateEvent } from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import { changeAnchor, ownerAnchor } from "../../../src/api/update-text.ts"
import type {
	ChangeEntry,
	OwnerChanges,
	Update,
} from "../../../src/api/update-types.ts"

const dayLength = 24 * 60 * 60 * 1000

export function oldestFirst(updates: Update[]): Update[] {
	return updates.toReversed()
}

export function eventOf(update: Update): UpdateEvent {
	return { version: update.version, published: update.published }
}

export function daysBetween(from: string, to: string): number {
	return (Date.parse(to) - Date.parse(from)) / dayLength
}

function entrySeparator(entry: ChangeEntry): string {
	return entry.kind === "Member" && (entry.after ?? entry.before)?.kind === "Method"
		? ":"
		: "."
}

export function entryLabel(owner: OwnerChanges, entry: ChangeEntry): string {
	return owner.kind === "Enum"
		? `Enum.${owner.name}.${entry.name}`
		: `${owner.name}${entrySeparator(entry)}${entry.name}`
}

export function ownerLabel(owner: Pick<OwnerChanges, "kind" | "name">): string {
	return owner.kind === "Enum" ? `Enum.${owner.name}` : owner.name
}

export function removedEntryRef(
	owner: OwnerChanges,
	entry: ChangeEntry,
	version: string,
): StatRef {
	return {
		kind: "Removed",
		label: entryLabel(owner, entry),
		version,
		anchor: changeAnchor(owner.kind, owner.name, entry.name),
	}
}

export function removedOwnerRef(owner: OwnerChanges, version: string): StatRef {
	return {
		kind: "Removed",
		label: ownerLabel(owner),
		version,
		anchor: ownerAnchor(owner),
	}
}

export function currentEntryRef(
	api: Api,
	owner: OwnerChanges,
	entry: ChangeEntry,
): StatRef | undefined {
	if (owner.kind === "Enum") {
		return api.enums[owner.name]?.items.some((item) => item.name === entry.name)
			? { kind: "EnumItem", owner: owner.name, name: entry.name }
			: undefined
	}
	const member = api.classes[owner.name]?.members.find(
		(item) => item.name === entry.name,
	)
	return member === undefined
		? undefined
		: { kind: member.kind, owner: owner.name, name: member.name }
}

export function currentOwnerRef(api: Api, owner: OwnerChanges): StatRef | undefined {
	if (owner.kind === "Enum") {
		return Object.hasOwn(api.enums, owner.name)
			? { kind: "Enum", name: owner.name }
			: undefined
	}
	return Object.hasOwn(api.classes, owner.name)
		? { kind: "Class", name: owner.name }
		: undefined
}
