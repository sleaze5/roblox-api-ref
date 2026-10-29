import init, { Matcher, Search } from "../../generated/search.js"
import wasmUrl from "../../generated/search_bg.wasm?url"

export type SearchModule = {
	Matcher: typeof Matcher
	Search: typeof Search
}

let pending: Promise<SearchModule> | undefined

/** Loads and instantiates the search engine's WebAssembly module once. */
export function loadSearchModule(): Promise<SearchModule> {
	pending ??= init({ module_or_path: wasmUrl }).then(() => ({ Matcher, Search }))
	return pending
}
