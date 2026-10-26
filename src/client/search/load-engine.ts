import searchIndexUrl from "../../generated/search.json?url"
import { SearchEngine } from "../../search/engine.ts"
import type { SearchIndex } from "../../search/index-format.ts"

let pending: Promise<SearchEngine> | undefined

async function fetchEngine(): Promise<SearchEngine> {
	const response = await fetch(searchIndexUrl)
	if (!response.ok) {
		throw new Error(`Failed to load the search index: HTTP ${response.status}`)
	}
	return new SearchEngine((await response.json()) as SearchIndex)
}

export function loadEngine(): Promise<SearchEngine> {
	pending ??= fetchEngine().catch((error: unknown) => {
		pending = undefined
		throw error
	})
	return pending
}
