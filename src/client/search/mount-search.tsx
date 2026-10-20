import { render } from "@solidjs/web"
import { loadEngine } from "./load-engine.ts"
import { SearchDialog } from "./SearchDialog.tsx"
import { openSearch, setSearchEngine, setSearchError } from "./search-state.ts"

let mounted = false

export function warmSearch(): void {
	loadEngine().then(
		(engine) => setSearchEngine(engine),
		(error: unknown) =>
			setSearchError(
				error instanceof Error
					? error.message
					: "The search index could not be loaded.",
			),
	)
}

export function showSearch(query?: string): void {
	const root = document.querySelector("[data-search-root]")
	if (!mounted && root !== null) {
		render(() => <SearchDialog />, root)
		mounted = true
	}
	warmSearch()
	openSearch(query)
}
