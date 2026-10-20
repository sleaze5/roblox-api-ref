import { createSignal } from "solid-js"
import type { SearchEngine } from "../../search/engine.ts"

export const [isSearchOpen, setSearchOpen] = createSignal(false)
export const [searchQuery, setSearchQuery] = createSignal("")
export const [activeResult, setActiveResult] = createSignal(0)
export const [searchEngine, setSearchEngine] = createSignal<SearchEngine | undefined>(
	undefined,
)
export const [searchError, setSearchError] = createSignal<string | undefined>(undefined)

export function openSearch(query?: string): void {
	if (query !== undefined) {
		setSearchQuery(query)
		setActiveResult(0)
	}
	setSearchOpen(true)
}
