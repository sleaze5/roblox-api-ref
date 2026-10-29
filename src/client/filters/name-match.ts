import type { Matcher } from "../../generated/search.js"

export type NameMatch = (name: string) => boolean

const matchAll: NameMatch = () => true

/**
 * Matches the names on a page against filter text with the search engine's
 * scoring, in WebAssembly. The module loads on first use, or earlier with
 * {@link NameMatcher.preload}.
 */
export class NameMatcher {
	readonly #indexes = new Map<string, number>()
	#matcher: Promise<Matcher> | undefined

	constructor(names: Iterable<string>) {
		for (const name of names) {
			if (!this.#indexes.has(name)) {
				this.#indexes.set(name, this.#indexes.size)
			}
		}
	}

	preload(): void {
		this.#matcher ??= import("../search/search-module.ts")
			.then((module) => module.loadSearchModule())
			.then((module) => new module.Matcher([...this.#indexes.keys()].join("\n")))
	}

	/** A test for each known name; an empty query matches every name. */
	async match(query: string): Promise<NameMatch> {
		if (query === "") {
			return matchAll
		}
		this.preload()
		const matcher = await this.#matcher
		const mask = matcher?.matches(query) ?? new Uint8Array()
		return (name) => mask[this.#indexes.get(name) ?? -1] === 1
	}
}
