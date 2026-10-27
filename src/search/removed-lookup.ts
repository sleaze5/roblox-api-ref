import type { RemovedEntry } from "./index-format.ts"
import type { ParsedQuery } from "./query.ts"
import { scoreName } from "./score.ts"

function scoreEntry(entry: RemovedEntry, query: ParsedQuery): number {
	if (query.kinds.size > 0 && !query.kinds.has(entry.kind)) {
		return 0
	}
	if (query.scope !== undefined) {
		const sameParent = entry.parent?.toLowerCase() === query.scope.toLowerCase()
		if (!sameParent || (query.enumScope === true && entry.kind !== "EnumItem")) {
			return 0
		}
	}
	const word = query.words[0] ?? ""
	if (word === "") {
		return query.scope === undefined ? 0 : 1
	}
	if (query.exact) {
		return entry.name.toLowerCase() === word.toLowerCase() ? 1 : 0
	}
	return scoreName(word.toLowerCase(), entry.name, entry.name.toLowerCase())
}

export function matchRemoved(
	removed: RemovedEntry[],
	query: ParsedQuery,
	limit: number,
): RemovedEntry[] {
	return removed
		.map((entry) => ({ entry, score: scoreEntry(entry, query) }))
		.filter((item) => item.score > 0)
		.sort(
			(a, b) =>
				b.score - a.score || b.entry.published.localeCompare(a.entry.published),
		)
		.slice(0, limit)
		.map((item) => item.entry)
}
