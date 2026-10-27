import type { HistoryEvent, ItemHistory } from "../../../src/api/item-history.ts"
import type { DescriptionTarget, DumpChange, DumpDiff } from "../../diff/diff-types.ts"

export type DatedDiff = {
	diff: DumpDiff
	published: string
}

export function classKey(name: string): string {
	return `Class:${name}`
}

export function enumKey(name: string): string {
	return `Enum:${name}`
}

export function memberKey(owner: string, name: string): string {
	return `Member:${owner}.${name}`
}

export function enumItemKey(owner: string, name: string): string {
	return `EnumItem:${owner}.${name}`
}

function changeKeys(change: DumpChange): { key: string; owner?: string } {
	switch (change.element) {
		case "Class":
			return { key: classKey(change.class) }
		case "Enum":
			return { key: enumKey(change.enum) }
		case "Member":
			return {
				key: memberKey(change.class, change.member),
				owner: classKey(change.class),
			}
		case "EnumItem":
			return {
				key: enumItemKey(change.enum, change.item),
				owner: enumKey(change.enum),
			}
	}
}

function targetKey(target: DescriptionTarget): string {
	switch (target.element) {
		case "Class":
			return classKey(target.class)
		case "Enum":
			return enumKey(target.enum)
		case "Member":
			return memberKey(target.class, target.member)
		case "EnumItem":
			return enumItemKey(target.enum, target.item)
	}
}

export class HistoryTable {
	readonly items = new Map<string, ItemHistory>()

	private entry(key: string): ItemHistory {
		let history = this.items.get(key)
		if (history === undefined) {
			history = {}
			this.items.set(key, history)
		}
		return history
	}

	private changeOwner(owner: string, event: HistoryEvent): void {
		if (this.items.get(owner)?.added?.version !== event.version) {
			this.entry(owner).changed = event
		}
	}

	private applyChange(change: DumpChange, event: HistoryEvent): void {
		const { key, owner } = changeKeys(change)
		switch (change.type) {
			case "Add":
				this.items.set(key, { added: event })
				break
			case "Remove":
				this.items.delete(key)
				break
			case "Change":
				this.entry(key).changed = event
				break
		}
		if (owner !== undefined) {
			this.changeOwner(owner, event)
		}
	}

	apply({ diff, published }: DatedDiff): void {
		const event = { version: diff.to.version, published }
		for (const change of diff.changes) {
			this.applyChange(change, event)
		}
		for (const description of diff.descriptions) {
			this.entry(targetKey(description)).described = event
		}
	}
}

export function buildHistory(diffs: DatedDiff[]): HistoryTable {
	const table = new HistoryTable()
	for (const diff of diffs) {
		table.apply(diff)
	}
	return table
}
