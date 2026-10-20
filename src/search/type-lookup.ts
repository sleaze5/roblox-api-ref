import { luauName } from "../api/luau-type.ts"
import type { SearchIndex } from "./index-format.ts"

const enumPrefix = "enum."

export class TypeLookup {
	private readonly index: SearchIndex
	private readonly lowerNames: string[]
	private readonly idsByEntry = new Map<number, number>()

	constructor(index: SearchIndex) {
		this.index = index
		this.lowerNames = index.typeNames.map((name) => name.toLowerCase())
		index.typeEntries.forEach((entry, id) => {
			if (entry !== -1) {
				this.idsByEntry.set(entry, id)
			}
		})
	}

	nameOf(id: number): string | undefined {
		return this.index.typeNames[id]
	}

	idOfEntry(entry: number): number | undefined {
		return this.idsByEntry.get(entry)
	}

	matching(value: string): Set<number> {
		const lower = value.toLowerCase()
		const ids = new Set<number>()
		this.lowerNames.forEach((name, id) => {
			if (
				name === lower ||
				name === `${enumPrefix}${lower}` ||
				luauName(this.index.typeNames[id] ?? "").toLowerCase() === lower
			) {
				ids.add(id)
			}
		})
		return ids
	}
}

export function hasAny(list: number[] | undefined, ids: Set<number>): boolean {
	return list?.some((id) => ids.has(id)) ?? false
}
