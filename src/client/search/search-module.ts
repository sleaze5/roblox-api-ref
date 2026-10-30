import init, { Matcher, Search } from "../../generated/search.js"
import wasmUrl from "../../generated/search_bg.wasm?url"
import { cacheUntilFailure } from "./cache-until-failure.ts"

export type SearchModule = {
	Matcher: typeof Matcher
	Search: typeof Search
}

export const loadSearchModule: () => Promise<SearchModule> = cacheUntilFailure(() =>
	init({ module_or_path: wasmUrl }).then(() => ({ Matcher, Search })),
)
