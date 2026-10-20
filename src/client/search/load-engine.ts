import searchIndexUrl from "../../generated/search.json?url"
import { SearchEngine } from "../../search/engine.ts"
import type { SearchIndex } from "../../search/index-format.ts"

let pending: Promise<SearchEngine> | undefined

export function loadEngine(): Promise<SearchEngine> {
	pending ??= fetch(searchIndexUrl)
		.then((response) => {
			if (!response.ok) {
				throw new Error(
					`Failed to load the search index: HTTP ${response.status}`,
				)
			}
			return response.json() as Promise<SearchIndex>
		})
		.then((index) => new SearchEngine(index))
	return pending
}
