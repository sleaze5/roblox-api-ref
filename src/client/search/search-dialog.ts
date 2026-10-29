import type { Search } from "../../generated/search.js"
import sprite from "../../generated/icons.svg?url"
import { writeParams } from "../url-state.ts"
import { loadEngine } from "./load-engine.ts"
import {
	emptyView,
	ghostText,
	helpHtml,
	iconHtml,
	loadingHtml,
	messageHtml,
	noResultsHtml,
	optionHrefs,
	optionId,
	optionIndex,
	parseView,
	resultsHtml,
	type SearchView,
} from "./search-view.ts"

const dialogClass =
	"m-auto mt-3 w-dialog motion-dialog overflow-hidden rounded-lg border border-line-strong bg-overlay p-0 text-fg shadow-float md:mt-24"

const dialogHtml = `<div class="flex h-12 items-center gap-3 border-b border-line px-4">${iconHtml(sprite, "magnifying-glass", "size-4", "text-fg-faint")}<div class="relative flex-1"><div aria-hidden="true" class="pointer-events-none absolute inset-0 flex items-center overflow-hidden font-mono whitespace-pre text-fg-faint"><span class="invisible" data-search-echo></span><span data-search-ghost></span></div><input type="text" role="combobox" aria-label="Search the API" aria-expanded="false" aria-controls="search-results" aria-autocomplete="list" autofocus autocomplete="off" spellcheck="false" placeholder="Search the API, or paste a version hash" class="relative h-12 w-full bg-transparent font-mono text-fg placeholder:font-sans placeholder:text-fg-faint focus:outline-none"></div><button type="button" aria-label="Close search" data-search-close class="grid size-9 shrink-0 place-items-center rounded-sm text-fg-muted motion-colors hover:bg-hover hover:text-fg md:size-auto md:hover:bg-transparent">${iconHtml(sprite, "x", "size-4", "md:hidden")}<kbd class="hidden kbd md:block">Esc</kbd></button></div><div class="max-h-results overflow-y-auto overscroll-contain p-1" data-search-body></div><div class="hidden h-10 items-center gap-4 border-t border-line px-4 text-xs text-fg-muted md:flex"><span class="flex items-center gap-1.5"><kbd class="kbd">↑↓</kbd>Move</span><span class="flex items-center gap-1.5"><kbd class="kbd">Enter</kbd>Open</span><span class="flex items-center gap-1.5"><kbd class="kbd">Tab</kbd>Complete</span></div>`

function part(root: Element, selector: string): HTMLElement {
	const element = root.querySelector(selector)
	if (!(element instanceof HTMLElement)) {
		throw new Error(`The search dialog has no ${selector}.`)
	}
	return element
}

/** The search dialog: one input, and results that the WebAssembly engine computes. */
class SearchDialog {
	readonly dialog: HTMLDialogElement
	readonly input: HTMLInputElement
	readonly echo: HTMLElement
	readonly ghost: HTMLElement
	readonly body: HTMLElement
	engine: Search | undefined
	error: string | undefined
	view: SearchView = emptyView
	hrefs: string[] = []
	active = 0

	constructor(root: Element) {
		this.dialog = document.createElement("dialog")
		this.dialog.setAttribute("aria-label", "Search the API")
		this.dialog.className = dialogClass
		this.dialog.innerHTML = dialogHtml
		root.append(this.dialog)
		const input = part(this.dialog, "input")
		if (!(input instanceof HTMLInputElement)) {
			throw new Error("The search dialog has no input.")
		}
		this.input = input
		this.echo = part(this.dialog, "[data-search-echo]")
		this.ghost = part(this.dialog, "[data-search-ghost]")
		this.body = part(this.dialog, "[data-search-body]")
		this.listen()
		this.render()
	}

	listen(): void {
		this.dialog.addEventListener("close", () => writeParams({ search: undefined }))
		this.dialog.addEventListener("click", (event) => {
			const target = event.target instanceof Element ? event.target : null
			if (target === this.dialog || target?.closest("[data-search-close]")) {
				this.dialog.close()
				return
			}
			const pick = target?.closest<HTMLElement>("[data-search-pick]")
			if (pick !== null && pick !== undefined) {
				this.setQuery(pick.dataset["searchPick"] ?? "")
				this.input.focus()
			}
		})
		this.body.addEventListener("pointermove", (event) => {
			const option =
				event.target instanceof Element
					? event.target.closest<HTMLElement>("[role=option]")
					: null
			const index = optionIndex(option?.id ?? "")
			if (index >= 0 && index !== this.active) {
				this.select(index, false)
			}
		})
		this.input.addEventListener("input", () => this.setQuery(this.input.value))
		this.input.addEventListener("keydown", (event) => this.onKeyDown(event))
	}

	get query(): string {
		return this.input.value
	}

	setQuery(query: string): void {
		if (this.input.value !== query) {
			this.input.value = query
		}
		this.active = 0
		this.render()
		writeParams({ search: this.dialog.open ? query : undefined })
	}

	open(query: string | undefined): void {
		if (query !== undefined) {
			this.input.value = query
			this.active = 0
			this.render()
		}
		if (!this.dialog.open) {
			this.dialog.showModal()
			this.input.select()
		}
		writeParams({ search: this.query })
	}

	setEngine(engine: Search): void {
		this.engine = engine
		this.render()
	}

	setError(message: string): void {
		this.error = message
		this.render()
	}

	render(): void {
		const query = this.query
		const searching = this.engine !== undefined && query.trim() !== ""
		this.view =
			searching && this.engine ? parseView(this.engine.view(query)) : emptyView
		this.hrefs = optionHrefs(this.view)
		this.echo.textContent = query
		this.ghost.textContent = ghostText(this.view, query)
		this.input.setAttribute(
			"aria-expanded",
			this.hrefs.length > 0 ? "true" : "false",
		)
		this.body.innerHTML = this.bodyHtml(searching)
		this.select(this.active, true)
	}

	bodyHtml(searching: boolean): string {
		if (this.error !== undefined) {
			return messageHtml(this.error)
		}
		if (this.engine === undefined) {
			return loadingHtml(sprite)
		}
		if (!searching) {
			return helpHtml(this.engine.latestUpload())
		}
		return this.hrefs.length === 0
			? noResultsHtml(this.query)
			: resultsHtml(this.view, sprite)
	}

	select(index: number, scroll: boolean): void {
		this.body
			.querySelector(`#${optionId(this.active)}`)
			?.setAttribute("aria-selected", "false")
		this.active = index
		const option = this.body.querySelector(`#${optionId(index)}`)
		option?.setAttribute("aria-selected", "true")
		if (option === null || option === undefined) {
			this.input.removeAttribute("aria-activedescendant")
			return
		}
		this.input.setAttribute("aria-activedescendant", optionId(index))
		if (scroll) {
			option.scrollIntoView({ block: "nearest" })
		}
	}

	move(step: number): void {
		const count = this.hrefs.length
		if (count > 0) {
			this.select((this.active + step + count) % count, true)
		}
	}

	openActive(newTab: boolean): void {
		const href = this.hrefs[this.active]
		if (href === undefined) {
			return
		}
		if (newTab) {
			window.open(href, "_blank", "noopener")
		} else {
			window.location.href = href
		}
	}

	accept(): boolean {
		const completion = this.view.c
		if (completion === null) {
			return false
		}
		this.setQuery(completion)
		return true
	}

	onKeyDown(event: KeyboardEvent): void {
		if (event.key === "ArrowDown") {
			event.preventDefault()
			this.move(1)
		} else if (event.key === "ArrowUp") {
			event.preventDefault()
			this.move(-1)
		} else if (event.key === "Enter") {
			event.preventDefault()
			this.openActive(event.ctrlKey || event.metaKey)
		} else if (event.key === "Tab" && !event.shiftKey) {
			if (this.accept()) {
				event.preventDefault()
			}
		} else if (
			event.key === "ArrowRight" &&
			this.input.selectionStart === this.input.value.length &&
			this.ghost.textContent !== ""
		) {
			event.preventDefault()
			this.accept()
		}
	}
}

let dialog: SearchDialog | undefined

export function showSearch(query?: string): void {
	const root = document.querySelector("[data-search-root]")
	if (root === null) {
		return
	}
	if (dialog === undefined) {
		const created = new SearchDialog(root)
		dialog = created
		loadEngine().then(
			(engine) => created.setEngine(engine),
			(error: unknown) =>
				created.setError(
					error instanceof Error
						? error.message
						: "The search index could not be loaded.",
				),
		)
	}
	dialog.open(query)
}
