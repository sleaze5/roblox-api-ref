import indexUrl from "../../generated/search.bin?url"
import type { Search } from "../../generated/search.js"
import { cacheUntilFailure } from "./cache-until-failure.ts"

async function fetchIndex(): Promise<Uint8Array> {
	const response = await fetch(indexUrl)
	if (!response.ok) {
		throw new Error(`Failed to load the search index: HTTP ${response.status}`)
	}
	return new Uint8Array(await response.arrayBuffer())
}

export const loadEngine: () => Promise<Search> = cacheUntilFailure(() =>
	Promise.all([
		import("./search-module.ts").then((module) => module.loadSearchModule()),
		fetchIndex(),
	]).then(([module, index]) => new module.Search(index)),
)
