import type {
	ChangeEntry,
	DescriptionDetail,
	OwnerChanges,
	OwnerKind,
} from "../../../src/api/update-types.ts"
import type { DescriptionChange, DumpChange } from "../../diff/diff-types.ts"
import { normalizeEnumItem } from "../normalize-enum-item.ts"
import { normalizeMember } from "../normalize-member.ts"
import { enumItemEntry, memberEntry, ownerDetails, statuses } from "./api-entries.ts"

function describedEntry(
	change: Extract<DescriptionChange, { element: "Member" | "EnumItem" }>,
): ChangeEntry {
	switch (change.element) {
		case "Member": {
			const member = normalizeMember(change.value)
			return {
				kind: "Member",
				status: "Changed",
				owner: change.class,
				name: change.member,
				details: [],
				descriptions: [],
				before: member,
				after: member,
			}
		}
		case "EnumItem": {
			const item = normalizeEnumItem(change.value)
			return {
				kind: "EnumItem",
				status: "Changed",
				owner: change.enum,
				name: change.item,
				details: [],
				descriptions: [],
				before: item,
				after: item,
			}
		}
	}
}

export class OwnerTable {
	readonly owners = new Map<string, OwnerChanges>()

	get(kind: OwnerKind, name: string): OwnerChanges {
		const key = `${kind}:${name}`
		let owner = this.owners.get(key)
		if (owner === undefined) {
			owner = { kind, name, details: [], descriptions: [], entries: [] }
			this.owners.set(key, owner)
		}
		return owner
	}

	add(change: DumpChange): void {
		switch (change.element) {
			case "Class":
			case "Enum": {
				const name = change.element === "Class" ? change.class : change.enum
				const owner = this.get(change.element, name)
				owner.status = statuses[change.type]
				owner.details = ownerDetails(change)
				return
			}
			case "Member":
				this.addEntry("Class", memberEntry(change))
				return
			case "EnumItem":
				this.addEntry("Enum", enumItemEntry(change))
		}
	}

	addDescription(change: DescriptionChange, detail: DescriptionDetail): void {
		if (change.element === "Class" || change.element === "Enum") {
			const name = change.element === "Class" ? change.class : change.enum
			this.get(change.element, name).descriptions.push(detail)
			return
		}
		const entry = describedEntry(change)
		const owner = this.get(entry.kind === "Member" ? "Class" : "Enum", entry.owner)
		const existing = owner.entries.find(
			(current) => current.name === entry.name && current.status !== "Removed",
		)
		if (existing === undefined) {
			entry.descriptions.push(detail)
			owner.entries.push(entry)
		} else {
			existing.descriptions.push(detail)
		}
	}

	private addEntry(kind: OwnerKind, entry: ChangeEntry | undefined): void {
		if (entry !== undefined) {
			this.get(kind, entry.owner).entries.push(entry)
		}
	}
}
