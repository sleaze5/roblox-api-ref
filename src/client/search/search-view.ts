import { escapeHtml } from "../html.ts"

export type ViewUpdate = [version: string, upload: string, date: string, href: string]

export type ViewResult = [
	kind: number,
	classIcon: number,
	name: string,
	marks: number[],
	context: string,
	role: string,
	deprecated: number,
	href: string,
]

export type ViewGroup = [title: string, results: ViewResult[]]

export type ViewRemoved = [
	kind: number,
	name: string,
	parent: string,
	version: string,
	href: string,
]

export type SearchView = {
	u: ViewUpdate[]
	g: ViewGroup[]
	x: ViewRemoved[]
	c: string | null
}

export const emptyView: SearchView = { u: [], g: [], x: [], c: null }

export const kindNames = [
	"Class",
	"Enum",
	"Property",
	"Method",
	"Event",
	"Callback",
	"EnumItem",
	"DataType",
] as const

export function parseView(json: string): SearchView {
	return JSON.parse(json) as SearchView
}

const optionPrefix = "search-option-"

export function optionId(index: number): string {
	return `${optionPrefix}${index}`
}

export function optionIndex(id: string): number {
	const index = id.startsWith(optionPrefix)
		? Number(id.slice(optionPrefix.length))
		: -1
	return Number.isInteger(index) ? index : -1
}

export function optionHrefs(view: SearchView): string[] {
	return [
		...view.u.map((update) => update[3]),
		...view.g.flatMap(([, results]) => results.map((result) => result[7])),
		...view.x.map((entry) => entry[4]),
	]
}

export function ghostText(view: SearchView, query: string): string {
	const completion = view.c
	return completion !== null &&
		completion.toLowerCase().startsWith(query.toLowerCase())
		? completion.slice(query.length)
		: ""
}

export function iconHtml(
	sprite: string,
	name: string,
	size: string,
	extra = "",
): string {
	const classes = ["shrink-0", size, extra].filter((part) => part !== "").join(" ")
	return `<svg class="${classes}" aria-hidden="true"><use href="${escapeHtml(`${sprite}#${name}`)}"></use></svg>`
}

function kindIcon(kind: number, classIcon: number): string {
	const style = classIcon < 0 ? "" : ` style="--icon: ${classIcon}"`
	return `<span aria-hidden="true" data-kind="${kindNames[kind] ?? "Class"}"${style} class="kind-icon text-fg-faint"></span>`
}

export function highlight(name: string, marks: number[]): string {
	let html = ""
	let at = 0
	for (let index = 0; index + 1 < marks.length; index += 2) {
		const start = marks[index] ?? 0
		const end = marks[index + 1] ?? start
		html += escapeHtml(name.slice(at, start))
		html += `<span class="text-fg-bright">${escapeHtml(name.slice(start, end))}</span>`
		at = end
	}
	return html + escapeHtml(name.slice(at))
}

const optionClass =
	"flex h-8 items-center gap-2 rounded-sm px-2 text-fg-muted aria-selected:bg-hover aria-selected:text-fg"

function option(index: number, href: string, content: string): string {
	return `<a id="${optionId(index)}" role="option" aria-selected="false" href="${escapeHtml(href)}" class="${optionClass}">${content}</a>`
}

function group(label: string, options: string): string {
	return `<div role="group" aria-label="${escapeHtml(label)}" class="pb-1"><p aria-hidden="true" class="px-2 pt-2 pb-1 text-xs font-medium text-fg-muted">${escapeHtml(label)}</p>${options}</div>`
}

function updateOption(sprite: string, update: ViewUpdate, index: number): string {
	const [version, upload, date, href] = update
	return option(
		index,
		href,
		`${iconHtml(sprite, "git-diff", "size-3.5", "text-fg-faint")}<span class="shrink-0 font-mono text-fg">${escapeHtml(version)}</span><span class="truncate font-mono text-fg-faint">${escapeHtml(upload)}</span><span class="ml-auto shrink-0 text-xs text-fg-faint">${escapeHtml(date)}</span>`,
	)
}

function resultOption(result: ViewResult, index: number): string {
	const [kind, classIcon, name, marks, context, role, deprecated, href] = result
	const tone = deprecated === 1 ? "text-fg-muted" : "text-fg"
	const roleHtml =
		role === ""
			? ""
			: `<span class="text-xs text-fg-faint">${escapeHtml(role)}</span>`
	const deprecatedHtml =
		deprecated === 1 ? `<span class="tag-chip tag-warning">Deprecated</span>` : ""
	return option(
		index,
		href,
		`${kindIcon(kind, classIcon)}<span class="shrink-0 font-mono ${tone}">${highlight(name, marks)}</span><span class="truncate font-mono text-fg-faint">${escapeHtml(context)}</span><span class="ml-auto flex shrink-0 items-center gap-2">${roleHtml}${deprecatedHtml}</span>`,
	)
}

function removedOption(entry: ViewRemoved, index: number): string {
	const [kind, name, parent, version, href] = entry
	return option(
		index,
		href,
		`<span class="change-mark w-4 shrink-0" data-status="Removed"><span class="sr-only">Removed:</span></span>${kindIcon(kind, -1)}<span class="shrink-0 font-mono text-fg-muted">${escapeHtml(name)}</span><span class="truncate font-mono text-fg-faint">${escapeHtml(parent)}</span><span class="ml-auto shrink-0 text-xs text-fg-faint">Removed in ${escapeHtml(version)}</span>`,
	)
}

export function resultsHtml(view: SearchView, sprite: string): string {
	let index = 0
	let html = ""
	if (view.u.length > 0) {
		html += group(
			"Updates",
			view.u.map((update) => updateOption(sprite, update, index++)).join(""),
		)
	}
	for (const [title, results] of view.g) {
		html += group(
			title,
			results.map((result) => resultOption(result, index++)).join(""),
		)
	}
	if (view.x.length > 0) {
		html += group(
			"Removed",
			view.x.map((entry) => removedOption(entry, index++)).join(""),
		)
	}
	return `<div id="search-results" role="listbox" aria-label="Results">${html}</div>`
}

const examples: [query: string, description: string][] = [
	["Part.", "Every member of Part, including inherited members"],
	["Humanoid.Move", "Members of Humanoid that match Move"],
	[".Touched", "Members named Touched on any class"],
	["gtp", "Acronyms also match, such as GetTouchingParts"],
	[
		"is:enum Material",
		"Only enums. Other kinds: class, type, property, method, event, callback, item, and member",
	],
	["tag:yields !tag:deprecated", "Members that yield and are not deprecated"],
	["Material", "The Material enum, and every member that uses it"],
	["Enum.Material.Plas", "Items of the Material enum that match Plas"],
	["type:Color3 is:property", "Properties whose type is Color3"],
	[
		"uses:Enum.KeyCode",
		"Members that use KeyCode as a value, parameter, or return value",
	],
	["access:game Get", "Members that game scripts can use"],
	['"Name"', "Exact names only"],
]

export function helpHtml(latestUpload: string | undefined): string {
	const all =
		latestUpload === undefined
			? examples
			: [
					...examples,
					[
						latestUpload,
						"The update for a Studio version, by its hash or version number",
					] as [string, string],
				]
	const buttons = all
		.map(
			([query, description]) =>
				`<button type="button" data-search-pick="${escapeHtml(query)}" class="flex items-baseline gap-3 rounded-sm px-2 py-1.5 text-left motion-colors hover:bg-hover"><code class="shrink-0 text-fg-bright">${escapeHtml(query)}</code><span class="text-fg-muted">${escapeHtml(description)}</span></button>`,
		)
		.join("")
	return `<div class="flex flex-col gap-1 p-1"><p class="px-1 pb-1 text-xs font-medium text-fg-muted">Search syntax</p>${buttons}</div>`
}

export function noResultsHtml(query: string): string {
	return `<p class="px-2 py-6 text-fg-muted">No results for <code class="text-fg">${escapeHtml(query.trim())}</code>. Check the spelling, or remove a filter.</p>`
}

export function messageHtml(message: string): string {
	return `<p class="px-2 py-6 text-fg-muted">${escapeHtml(message)}</p>`
}

export function loadingHtml(sprite: string): string {
	return `<p role="status" class="flex motion-loading items-center gap-2 px-2 py-6 text-fg-muted">${iconHtml(sprite, "circle-notch", "size-4", "motion-spin")}Preparing the search index…</p>`
}
