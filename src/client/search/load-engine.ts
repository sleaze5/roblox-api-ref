import indexUrl from "../../generated/search.bin?url"
import type { Search } from "../../generated/search.js"

let pending: Promise<Search> | undefined

async function fetchIndex(): Promise<Uint8Array> {
	const response = await fetch(indexUrl)
	if (!response.ok) {
		throw new Error(`Failed to load the search index: HTTP ${response.status}`)
	}
	return new Uint8Array(await response.arrayBuffer())
}

export function loadEngine(): Promise<Search> {
	pending ??= Promise.all([
		import("./search-module.ts").then((module) => module.loadSearchModule()),
		fetchIndex(),
	]).then(([module, index]) => new module.Search(index))
	return pending
}
