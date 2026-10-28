import type { HistoryChanges } from "../../../src/api/item-history.ts"
import type { Member } from "../../../src/api/types.ts"
import type {
	ChangeDetail,
	ChangeEntry,
	OwnerChanges,
} from "../../../src/api/update-types.ts"

const fieldNames: Record<string, string> = {
	"Security": "security",
	"Read security": "security",
	"Write security": "security",
	"Thread safety": "thread safety",
	"Capabilities": "capabilities",
	"Read capabilities": "capabilities",
	"Write capabilities": "capabilities",
	"Property category": "category",
	"Serialization": "serialization",
	"Use instead": "replacement",
	"Superclass": "superclass",
	"Memory category": "memory category",
	"Legacy names": "legacy names",
	"Simulation access": "simulation access",
}

const addedTagLabels = new Set(["Added tags", "Tags"])
const deprecatedTag = "Deprecated"

function json(value: unknown): string {
	return JSON.stringify(value)
}

function parameters(member: Member): string {
	return member.kind === "Property"
		? ""
		: json(
				member.parameters.map(({ name, type, default: value }) => [
					name,
					type,
					value,
				]),
			)
}

function memberFields(before: Member, after: Member): string[] {
	if (before.kind !== after.kind) {
		return ["signature"]
	}
	if (before.kind === "Property" && after.kind === "Property") {
		return [
			...(json(before.type) === json(after.type) ? [] : ["type"]),
			...(before.default === after.default ? [] : ["default value"]),
		]
	}
	const returns = (member: Member) =>
		member.kind === "Method" || member.kind === "Callback"
			? json(member.returns)
			: ""
	if (
		parameters(before) === parameters(after) &&
		returns(before) === returns(after)
	) {
		return []
	}
	return [before.kind === "Event" ? "parameters" : "signature"]
}

function valueFields(entry: ChangeEntry): string[] {
	if (
		entry.status !== "Changed" ||
		entry.before === undefined ||
		entry.after === undefined
	) {
		return []
	}
	if (entry.kind === "EnumItem") {
		return entry.before.value === entry.after.value ? [] : ["value"]
	}
	return memberFields(entry.before, entry.after)
}

function summarize(
	details: ChangeDetail[],
	fields: string[],
): HistoryChanges | undefined {
	const changes: HistoryChanges = {
		fields: [...fields],
		tagsAdded: 0,
		tagsRemoved: 0,
	}
	for (const detail of details) {
		if (detail.kind === "tags") {
			const added = addedTagLabels.has(detail.label)
			if (detail.tags.includes(deprecatedTag)) {
				changes.deprecation = added ? "set" : "clear"
			}
			const others = detail.tags.filter((tag) => tag !== deprecatedTag).length
			changes[added ? "tagsAdded" : "tagsRemoved"] += others
			continue
		}
		const field = fieldNames[detail.label] ?? detail.label.toLowerCase()
		if (!changes.fields.includes(field)) {
			changes.fields.push(field)
		}
	}
	return isEmpty(changes) ? undefined : changes
}

export function entryChanges(entry: ChangeEntry): HistoryChanges | undefined {
	return entry.status === "Changed"
		? summarize(entry.details, valueFields(entry))
		: undefined
}

function isEmpty(changes: HistoryChanges): boolean {
	return (
		changes.fields.length === 0 &&
		changes.tagsAdded === 0 &&
		changes.tagsRemoved === 0 &&
		changes.deprecation === undefined &&
		changes.deprecated === undefined
	)
}

function mergeEntry(changes: HistoryChanges, entry: HistoryChanges): void {
	for (const field of entry.fields) {
		if (!changes.fields.includes(field)) {
			changes.fields.push(field)
		}
	}
	changes.tagsAdded += entry.tagsAdded
	changes.tagsRemoved += entry.tagsRemoved
	if (entry.deprecation === "set") {
		changes.deprecated = (changes.deprecated ?? 0) + 1
	}
}

export function ownerChanges(owner: OwnerChanges): HistoryChanges | undefined {
	if (owner.status === "Added" || owner.status === "Removed") {
		return undefined
	}
	const changes = summarize(owner.details, []) ?? {
		fields: [],
		tagsAdded: 0,
		tagsRemoved: 0,
	}
	for (const entry of owner.entries) {
		const summary = entryChanges(entry)
		if (summary !== undefined) {
			mergeEntry(changes, summary)
		}
	}
	return isEmpty(changes) ? undefined : changes
}
