import { compareNames } from "../../src/api/compare-names.ts"
import { accessRank, isHidden, isInternal } from "../../src/api/members.ts"
import type { Api, Class } from "../../src/api/types.ts"
import type { Update } from "../../src/api/update-types.ts"
import {
	entryFlags,
	entryKinds,
	type EntryKind,
	type RemovedEntry,
	type SearchIndex,
} from "../../src/search/index-format.ts"
import type { HistoryEvent, RemovedOwner } from "../../src/api/item-history.ts"
import { parameterTypeKeys, valueTypeKeys, type TypeKey } from "./search-types.ts"

const tagFlags: Record<string, number> = {
	Deprecated: entryFlags.deprecated,
	Hidden: entryFlags.hidden,
	NotBrowsable: entryFlags.hidden,
	NotScriptable: entryFlags.notScriptable,
	Service: entryFlags.service,
	ReadOnly: entryFlags.readOnly,
	Yields: entryFlags.yields,
	NotCreatable: entryFlags.notCreatable,
	NotReplicated: entryFlags.notReplicated,
}

function flagsOf(tags: string[]): number {
	let flags = 0
	for (const tag of tags) {
		flags |= tagFlags[tag] ?? 0
	}
	return flags
}

function byName(a: { name: string }, b: { name: string }): number {
	return compareNames(a.name, b.name)
}

class IndexBuilder {
	readonly index: SearchIndex = {
		names: [],
		kinds: [],
		parents: [],
		flags: [],
		security: [],
		valueTypes: [],
		parameterTypes: [],
		typeNames: [],
		typeEntries: [],
		updates: [],
		removed: [],
		classIcons: [],
	}
	readonly primaryEntries = new Map<string, number>()
	private readonly typeIdsByKey = new Map<string, number>()
	private readonly typeKeys: string[] = []

	add(
		kind: EntryKind,
		name: string,
		parent: number,
		tags: string[],
		security = 0,
	): number {
		this.index.names.push(name)
		this.index.kinds.push(entryKinds.indexOf(kind))
		this.index.parents.push(parent)
		this.index.flags.push(flagsOf(tags))
		this.index.security.push(security)
		this.index.valueTypes.push([])
		this.index.parameterTypes.push([])
		return this.index.names.length - 1
	}

	addClass(item: Class): number {
		const entry = this.add("Class", item.name, -1, item.tags)
		this.index.classIcons[entry] = item.icon ?? null
		this.primaryEntries.set(`Class:${item.name}`, entry)
		return entry
	}

	typeIds(keys: TypeKey[]): number[] {
		return keys.map(({ key, display }) => {
			let id = this.typeIdsByKey.get(key)
			if (id === undefined) {
				id = this.index.typeNames.length
				this.index.typeNames.push(display)
				this.typeKeys.push(key)
				this.typeIdsByKey.set(key, id)
			}
			return id
		})
	}

	linkTypes(): void {
		const { typeNames } = this.index
		const order = this.typeKeys
			.map((_, id) => id)
			.sort((a, b) => compareNames(typeNames[a] ?? "", typeNames[b] ?? ""))
		const sortedIds: number[] = []
		order.forEach((id, sorted) => {
			sortedIds[id] = sorted
		})
		const remap = (ids: number[]) => ids.map((id) => sortedIds[id] ?? id)
		this.index.valueTypes = this.index.valueTypes.map(remap)
		this.index.parameterTypes = this.index.parameterTypes.map(remap)
		this.index.typeNames = order.map((id) => typeNames[id] ?? "")
		this.index.typeEntries = order.map(
			(id) => this.primaryEntries.get(this.typeKeys[id] ?? "") ?? -1,
		)
	}
}

function removedEntry(
	kind: EntryKind,
	name: string,
	parent: string | undefined,
	removed: HistoryEvent,
): RemovedEntry {
	const entry: RemovedEntry = {
		kind,
		name,
		version: removed.version,
		published: removed.published,
	}
	if (parent !== undefined) {
		entry.parent = parent
	}
	return entry
}

function removedEntries(api: Api): RemovedEntry[] {
	const owners = (kind: "Class" | "Enum", list: RemovedOwner[] | undefined) =>
		(list ?? []).map((owner) =>
			removedEntry(kind, owner.name, undefined, owner.removed),
		)
	return [
		...owners("Class", api.removed?.classes),
		...owners("Enum", api.removed?.enums),
		...Object.values(api.classes).flatMap((item) =>
			(item.removedMembers ?? []).map((member) =>
				removedEntry(
					member.value.kind,
					member.value.name,
					item.name,
					member.removed,
				),
			),
		),
		...Object.values(api.enums).flatMap((item) =>
			(item.removedItems ?? []).map((enumItem) =>
				removedEntry(
					"EnumItem",
					enumItem.value.name,
					item.name,
					enumItem.removed,
				),
			),
		),
	]
}

export function buildSearchIndex(api: Api, updates: Update[]): SearchIndex {
	const builder = new IndexBuilder()
	const { index } = builder
	index.removed = removedEntries(api)
	index.updates = updates.map(({ version, hash, published }) => ({
		version,
		hash,
		published,
	}))
	const classes = Object.values(api.classes).sort(byName)
	const classEntries = new Map<string, number>()
	for (const item of classes) {
		classEntries.set(item.name, builder.addClass(item))
	}
	for (const item of classes) {
		const entry = classEntries.get(item.name) ?? -1
		index.parents[entry] =
			item.superclass === undefined
				? -1
				: (classEntries.get(item.superclass) ?? -1)
	}
	for (const item of classes) {
		const parent = classEntries.get(item.name) ?? -1
		for (const member of item.members) {
			const entry = builder.add(
				member.kind,
				member.name,
				parent,
				member.tags,
				accessRank(member),
			)
			index.valueTypes[entry] = builder.typeIds(valueTypeKeys(member))
			index.parameterTypes[entry] = builder.typeIds(parameterTypeKeys(member))
			index.flags[entry] =
				(index.flags[entry] ?? 0) |
				(isHidden(member) ? entryFlags.hidden : 0) |
				(isInternal(member) ? entryFlags.internal : 0)
		}
	}
	for (const item of Object.values(api.enums).sort(byName)) {
		const parent = builder.add("Enum", item.name, -1, item.tags)
		builder.primaryEntries.set(`Enum:${item.name}`, parent)
		for (const enumItem of item.items) {
			builder.add("EnumItem", enumItem.name, parent, enumItem.tags)
		}
	}
	for (const item of Object.values(api.dataTypes).sort(byName)) {
		const entry = builder.add("DataType", item.name, -1, [])
		builder.primaryEntries.set(`${item.category}:${item.name}`, entry)
	}
	builder.linkTypes()
	return index
}
