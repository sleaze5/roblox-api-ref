import { compareNames } from "../api/compare-names.ts"
import type { TypeRole } from "../api/types.ts"
import {
	entryFlags,
	entryKinds,
	type EntryKind,
	type SearchIndex,
} from "./index-format.ts"
import { accessFilters, hasFilters, type ParsedQuery } from "./query.ts"
import { scoreName } from "./score.ts"
import { hasAny, TypeLookup } from "./type-lookup.ts"

export type SearchResult = {
	entry: number
	score: number
	scope?: number
	role?: TypeRole
}

export type TypeUsers = {
	entry: number
	typeName: string
	results: SearchResult[]
}

type PreparedQuery = {
	query: ParsedQuery
	words: string[]
	valueTypes: Set<number> | undefined
	usedTypes: Set<number> | undefined
	maxAccess: number
}

const kindBoost: Record<EntryKind, number> = {
	Class: 40,
	Enum: 30,
	DataType: 30,
	Property: 10,
	Method: 10,
	Event: 10,
	Callback: 5,
	EnumItem: 0,
}

const kindOrder: Record<EntryKind, number> = {
	Class: 0,
	DataType: 1,
	Enum: 2,
	Property: 3,
	Method: 4,
	Event: 5,
	Callback: 6,
	EnumItem: 7,
}

const relatedKinds: EntryKind[] = ["Enum", "DataType", "Class"]

const scopeThreshold = 800
const maxScopes = 3

export class SearchEngine {
	readonly index: SearchIndex
	readonly lowerNames: string[]
	readonly types: TypeLookup
	private readonly children = new Map<number, number[]>()
	private readonly primaries: number[] = []

	constructor(index: SearchIndex) {
		this.index = index
		this.lowerNames = index.names.map((name) => name.toLowerCase())
		this.types = new TypeLookup(index)
		index.kinds.forEach((_, entry) => {
			if (this.isPrimary(entry)) {
				this.primaries.push(entry)
				return
			}
			const parent = index.parents[entry] ?? -1
			const list = this.children.get(parent) ?? []
			list.push(entry)
			this.children.set(parent, list)
		})
	}

	get size(): number {
		return this.index.names.length
	}

	kindOf(entry: number): EntryKind {
		return entryKinds[this.index.kinds[entry] ?? 0] ?? "Class"
	}

	nameOf(entry: number): string {
		return this.index.names[entry] ?? ""
	}

	parentOf(entry: number): number {
		return this.index.parents[entry] ?? -1
	}

	flagsOf(entry: number): number {
		return this.index.flags[entry] ?? 0
	}

	typeOf(entry: number): string | undefined {
		const [first] = this.index.valueTypes[entry] ?? []
		return first === undefined ? undefined : this.types.nameOf(first)
	}

	roleOf(entry: number, typeIds: Set<number>): TypeRole {
		if (!hasAny(this.index.valueTypes[entry], typeIds)) {
			return "Parameter"
		}
		return this.kindOf(entry) === "Property" ? "Value" : "Return"
	}

	isPrimary(entry: number): boolean {
		const kind = this.kindOf(entry)
		return kind === "Class" || kind === "Enum" || kind === "DataType"
	}

	ancestorsOf(entry: number): number[] {
		const chain: number[] = []
		let current = entry
		while (current !== -1 && chain.length < 64) {
			chain.push(current)
			current = this.kindOf(current) === "Class" ? this.parentOf(current) : -1
		}
		return chain
	}

	membersOf(primary: number): number[] {
		return this.ancestorsOf(primary).flatMap(
			(owner) => this.children.get(owner) ?? [],
		)
	}

	private prepare(query: ParsedQuery): PreparedQuery {
		return {
			query,
			words: query.words.map((word) => word.toLowerCase()),
			valueTypes:
				query.valueType === undefined
					? undefined
					: this.types.matching(query.valueType),
			usedTypes:
				query.uses === undefined ? undefined : this.types.matching(query.uses),
			maxAccess:
				query.access === undefined
					? Number.POSITIVE_INFINITY
					: accessFilters[query.access],
		}
	}

	private matchesTypes(entry: number, prepared: PreparedQuery): boolean {
		const values = this.index.valueTypes[entry]
		if (prepared.valueTypes !== undefined && !hasAny(values, prepared.valueTypes)) {
			return false
		}
		return (
			prepared.usedTypes === undefined ||
			hasAny(values, prepared.usedTypes) ||
			hasAny(this.index.parameterTypes[entry], prepared.usedTypes)
		)
	}

	private matchesFilters(entry: number, prepared: PreparedQuery): boolean {
		const { query } = prepared
		if (query.kinds.size > 0 && !query.kinds.has(this.kindOf(entry))) {
			return false
		}
		if (!this.matchesTypes(entry, prepared)) {
			return false
		}
		if ((this.index.security[entry] ?? 0) > prepared.maxAccess) {
			return false
		}
		const flags = this.flagsOf(entry)
		return (
			(flags & query.include) === query.include && (flags & query.exclude) === 0
		)
	}

	private scoreEntry(entry: number, prepared: PreparedQuery): number {
		const name = this.nameOf(entry)
		const lower = this.lowerNames[entry] ?? ""
		let total = 0
		for (const word of prepared.words) {
			const score = prepared.query.exact
				? lower === word
					? 1000
					: 0
				: scoreName(word, name, lower)
			if (score === 0) {
				return 0
			}
			total += score
		}
		const flags = this.flagsOf(entry)
		total += kindBoost[this.kindOf(entry)]
		if ((flags & entryFlags.deprecated) !== 0) {
			total -= 60
		}
		if ((flags & entryFlags.hidden) !== 0) {
			total -= 40
		}
		if ((flags & entryFlags.internal) !== 0) {
			total -= 40
		}
		return Math.max(total, 1)
	}

	resolveScopes(scope: string, kind?: EntryKind): number[] {
		const lowerScope = scope.toLowerCase()
		const primaries =
			kind === undefined
				? this.primaries
				: this.primaries.filter((entry) => this.kindOf(entry) === kind)
		const exact = primaries.filter((entry) => this.lowerNames[entry] === lowerScope)
		if (exact.length > 0) {
			return exact
		}
		return primaries
			.map((entry) => ({
				entry,
				score: scoreName(
					lowerScope,
					this.nameOf(entry),
					this.lowerNames[entry] ?? "",
				),
			}))
			.filter((candidate) => candidate.score >= scopeThreshold)
			.sort((a, b) => b.score - a.score)
			.slice(0, maxScopes)
			.map((candidate) => candidate.entry)
	}

	private compareNames(a: number, b: number): number {
		const left = this.lowerNames[a] ?? ""
		const right = this.lowerNames[b] ?? ""
		return left < right ? -1 : left > right ? 1 : 0
	}

	private compare(a: SearchResult, b: SearchResult): number {
		return (
			b.score - a.score ||
			kindOrder[this.kindOf(a.entry)] - kindOrder[this.kindOf(b.entry)] ||
			this.compareNames(a.entry, b.entry)
		)
	}

	private candidates(query: ParsedQuery): { entry: number; scope?: number }[] {
		if (query.scope === undefined) {
			return query.words.length > 0 || hasFilters(query)
				? this.index.names.map((_, entry) => ({ entry }))
				: []
		}
		if (query.scope === "") {
			return this.index.names.flatMap((_, entry) =>
				this.isPrimary(entry) ? [] : [{ entry }],
			)
		}
		const scopeKind = query.enumScope === true ? "Enum" : undefined
		return this.resolveScopes(query.scope, scopeKind).flatMap((scope) =>
			this.membersOf(scope).map((entry) => ({ entry, scope })),
		)
	}

	search(query: ParsedQuery, limit: number): SearchResult[] {
		const prepared = this.prepare(query)
		const results: SearchResult[] = []
		for (const candidate of this.candidates(query)) {
			if (!this.matchesFilters(candidate.entry, prepared)) {
				continue
			}
			const score = this.scoreEntry(candidate.entry, prepared)
			if (score > 0) {
				results.push(
					prepared.usedTypes === undefined
						? { ...candidate, score }
						: {
								...candidate,
								score,
								role: this.roleOf(candidate.entry, prepared.usedTypes),
							},
				)
			}
		}
		results.sort((a, b) => this.compare(a, b))
		return results.slice(0, limit)
	}

	private namedTypes(query: ParsedQuery): number[] {
		const [word, ...rest] = query.words
		if (word === undefined || rest.length > 0 || query.scope !== undefined) {
			return []
		}
		const lower = word.toLowerCase()
		return relatedKinds
			.filter((kind) => query.kinds.size === 0 || query.kinds.has(kind))
			.flatMap((kind) =>
				this.primaries.filter(
					(entry) =>
						this.kindOf(entry) === kind && this.lowerNames[entry] === lower,
				),
			)
	}

	usersOfEntry(entry: number, limit: number): SearchResult[] {
		const id = this.types.idOfEntry(entry)
		if (id === undefined) {
			return []
		}
		const ids = new Set([id])
		const results: SearchResult[] = []
		this.index.kinds.forEach((_, candidate) => {
			if (
				hasAny(this.index.valueTypes[candidate], ids) ||
				hasAny(this.index.parameterTypes[candidate], ids)
			) {
				results.push({
					entry: candidate,
					score: 0,
					role: this.roleOf(candidate, ids),
				})
			}
		})
		return results.sort((a, b) => this.compareUsers(a, b)).slice(0, limit)
	}

	private compareUsers(a: SearchResult, b: SearchResult): number {
		const deprecated =
			(this.flagsOf(a.entry) & entryFlags.deprecated) -
			(this.flagsOf(b.entry) & entryFlags.deprecated)
		return (
			deprecated ||
			kindOrder[this.kindOf(a.entry)] - kindOrder[this.kindOf(b.entry)] ||
			compareNames(
				this.nameOf(this.parentOf(a.entry)),
				this.nameOf(this.parentOf(b.entry)),
			) ||
			compareNames(this.nameOf(a.entry), this.nameOf(b.entry))
		)
	}

	typeUsers(query: ParsedQuery, limit: number): TypeUsers[] {
		return this.namedTypes(query).flatMap((entry) => {
			const results = this.usersOfEntry(entry, limit)
			const id = this.types.idOfEntry(entry)
			const typeName = id === undefined ? undefined : this.types.nameOf(id)
			return results.length === 0 || typeName === undefined
				? []
				: [{ entry, typeName, results }]
		})
	}
}
