import { createEffect, createMemo, Show } from "solid-js"
import { Icon } from "../../components/Icon.tsx"
import { complete } from "../../search/complete.ts"
import { parseQuery } from "../../search/query.ts"
import { groupResults, resultHref } from "../../search/result-info.ts"
import { ResultGroups } from "./ResultGroups.tsx"
import { optionId } from "./ResultOption.tsx"
import {
	activeResult,
	isSearchOpen,
	searchEngine,
	searchError,
	searchQuery,
	setActiveResult,
	setSearchOpen,
	setSearchQuery,
} from "./search-state.ts"
import { SearchHelp } from "./SearchHelp.tsx"
import { writeParams } from "../url-state.ts"

const globalLimit = 80
const scopedLimit = 300
const perGroupLimit = 8
const usageLimit = 12

export function SearchDialog() {
	let dialog: HTMLDialogElement | undefined
	let input: HTMLInputElement | undefined

	const parsed = createMemo(() => parseQuery(searchQuery()))
	const scoped = createMemo(() => parsed().scope !== undefined)
	const groups = createMemo(() => {
		const engine = searchEngine()
		if (engine === undefined) {
			return []
		}
		const results = engine.search(parsed(), scoped() ? scopedLimit : globalLimit)
		return groupResults(
			engine,
			results,
			scoped() ? scopedLimit : perGroupLimit,
			engine.typeUsers(parsed(), usageLimit),
		)
	})
	const flat = createMemo(() => groups().flatMap((group) => group.results))
	const completion = createMemo(() => {
		const engine = searchEngine()
		return engine === undefined
			? undefined
			: complete(searchQuery(), engine, flat()[0])
	})
	const ghost = createMemo(() => {
		const value = completion()
		const query = searchQuery()
		return value !== undefined &&
			value.toLowerCase().startsWith(query.toLowerCase())
			? value.slice(query.length)
			: ""
	})
	const firstWord = createMemo(() => (parsed().words[0] ?? "").toLowerCase())

	createEffect(
		() => isSearchOpen(),
		(open) => {
			if (dialog === undefined) {
				return
			}
			if (open && !dialog.open) {
				dialog.showModal()
				input?.select()
			} else if (!open && dialog.open) {
				dialog.close()
			}
		},
	)

	createEffect(
		() => (isSearchOpen() ? searchQuery() : undefined),
		(query) => writeParams({ search: query }),
	)

	createEffect(
		() => activeResult(),
		(index) => {
			document
				.getElementById(optionId(index))
				?.scrollIntoView({ block: "nearest" })
		},
	)

	const move = (step: number) => {
		const count = flat().length
		if (count > 0) {
			setActiveResult((index) => (index + step + count) % count)
		}
	}

	const openActive = (newTab: boolean) => {
		const engine = searchEngine()
		const result = flat()[activeResult()]
		if (engine === undefined || result === undefined) {
			return
		}
		const href = resultHref(engine, result)
		if (newTab) {
			window.open(href, "_blank", "noopener")
		} else {
			window.location.href = href
		}
	}

	const accept = (): boolean => {
		const value = completion()
		if (value === undefined) {
			return false
		}
		setSearchQuery(value)
		setActiveResult(0)
		return true
	}

	const onKeyDown = (event: KeyboardEvent) => {
		if (event.key === "ArrowDown") {
			event.preventDefault()
			move(1)
		} else if (event.key === "ArrowUp") {
			event.preventDefault()
			move(-1)
		} else if (event.key === "Enter") {
			event.preventDefault()
			openActive(event.ctrlKey || event.metaKey)
		} else if (event.key === "Tab" && !event.shiftKey) {
			if (accept()) {
				event.preventDefault()
			}
		} else if (
			event.key === "ArrowRight" &&
			input !== undefined &&
			input.selectionStart === input.value.length &&
			ghost() !== ""
		) {
			event.preventDefault()
			accept()
		}
	}

	return (
		<dialog
			ref={(element) => (dialog = element)}
			aria-label="Search the API"
			onClose={() => setSearchOpen(false)}
			onClick={(event) => {
				if (event.target === dialog) {
					setSearchOpen(false)
				}
			}}
			class="m-auto mt-3 w-dialog motion-dialog overflow-hidden rounded-lg border border-line-strong bg-overlay p-0 text-fg shadow-float md:mt-24">
			<div class="flex h-12 items-center gap-3 border-b border-line px-4">
				<Icon name="magnifying-glass" size={16} class="text-fg-faint" />
				<div class="relative flex-1">
					<div
						aria-hidden="true"
						class="pointer-events-none absolute inset-0 flex items-center overflow-hidden font-mono whitespace-pre text-fg-faint">
						<span class="invisible">{searchQuery()}</span>
						{ghost()}
					</div>
					<input
						ref={(element) => (input = element)}
						type="text"
						role="combobox"
						aria-label="Search the API"
						aria-expanded={flat().length > 0 ? "true" : "false"}
						aria-controls="search-results"
						aria-activedescendant={
							flat().length > 0 ? optionId(activeResult()) : undefined
						}
						aria-autocomplete="list"
						autofocus
						autocomplete="off"
						spellcheck={false}
						placeholder="Search classes, members, enums, and data types"
						value={searchQuery()}
						onInput={(event) => {
							setSearchQuery(event.currentTarget.value)
							setActiveResult(0)
						}}
						onKeyDown={onKeyDown}
						class="relative h-12 w-full bg-transparent font-mono text-fg placeholder:font-sans placeholder:text-fg-faint focus:outline-none"
					/>
				</div>
				<button
					type="button"
					aria-label="Close search"
					onClick={() => setSearchOpen(false)}
					class="grid size-9 shrink-0 place-items-center rounded-sm text-fg-muted motion-colors hover:bg-hover hover:text-fg md:size-auto md:hover:bg-transparent">
					<Icon name="x" size={16} class="md:hidden" />
					<kbd class="hidden kbd md:block">Esc</kbd>
				</button>
			</div>
			<div class="max-h-results overflow-y-auto overscroll-contain p-1">
				<Show
					when={searchError() === undefined}
					fallback={<p class="px-2 py-6 text-fg-muted">{searchError()}</p>}>
					<Show
						when={searchEngine()}
						fallback={
							<p class="px-2 py-6 text-fg-muted">
								Loading the search index…
							</p>
						}>
						{(engine) => (
							<Show
								when={searchQuery().trim() !== ""}
								fallback={
									<SearchHelp
										onPick={(value) => setSearchQuery(value)}
									/>
								}>
								<Show
									when={flat().length > 0}
									fallback={
										<p class="px-2 py-6 text-fg-muted">
											No results for{" "}
											<code class="text-fg">
												{searchQuery().trim()}
											</code>
											. Check the spelling, or remove a filter.
										</p>
									}>
									<div
										id="search-results"
										role="listbox"
										aria-label="Results">
										<ResultGroups
											engine={engine()}
											groups={groups()}
											word={firstWord()}
										/>
									</div>
								</Show>
							</Show>
						)}
					</Show>
				</Show>
			</div>
			<div class="hidden h-10 items-center gap-4 border-t border-line px-4 text-xs text-fg-muted md:flex">
				<span class="flex items-center gap-1.5">
					<kbd class="kbd">↑↓</kbd>
					Move
				</span>
				<span class="flex items-center gap-1.5">
					<kbd class="kbd">Enter</kbd>
					Open
				</span>
				<span class="flex items-center gap-1.5">
					<kbd class="kbd">Tab</kbd>
					Complete
				</span>
			</div>
		</dialog>
	)
}
