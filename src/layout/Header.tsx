import { homeHref } from "../api/routes.ts"
import type { Api } from "../api/types.ts"
import { Icon } from "../components/Icon.tsx"
import { sectionLinks, type Section } from "./section-links.ts"

export type Release = Pick<Api, "version" | "hash">

function SectionNav(props: { section: Section | undefined }) {
	return (
		<nav aria-label="Sections" class="hidden items-center gap-1 md:flex">
			{sectionLinks.map((link) => (
				<a
					href={link.href}
					aria-current={props.section === link.section ? "page" : undefined}
					class={[
						"inline-flex h-8 items-center rounded-sm px-2 font-medium whitespace-nowrap motion-colors hover:bg-hover hover:text-fg",
						props.section === link.section ? "text-fg" : "text-fg-muted",
					]}>
					{link.label}
				</a>
			))}
		</nav>
	)
}

function SearchButton() {
	return (
		<button
			type="button"
			data-search-open=""
			aria-label="Search the API"
			aria-keyshortcuts="Control+K /"
			class="ml-auto flex h-10 shrink-0 items-center gap-2 rounded-sm px-2.5 text-fg-muted motion-colors hover:bg-hover hover:text-fg md:ml-0 md:h-8 md:w-80 md:shrink md:border md:border-line md:bg-raised md:text-fg-faint md:hover:border-line-strong md:hover:bg-raised md:hover:text-fg-muted">
			<Icon name="magnifying-glass" size={16} />
			<span class="hidden md:inline">Search the API</span>
			<kbd class="ml-auto hidden kbd md:block">Ctrl K</kbd>
		</button>
	)
}

export function Header(props: { section: Section | undefined; release: Release }) {
	return (
		<header class="sticky top-0 z-20 flex h-12 items-center gap-2 border-b border-line bg-surface px-2 md:gap-3 md:px-4">
			<div class="flex min-w-0 flex-1 items-center gap-2 md:gap-3">
				<button
					type="button"
					popovertarget="sidebar"
					aria-label="Open navigation"
					class="grid size-10 shrink-0 place-items-center rounded-sm text-fg-muted motion-colors hover:bg-hover hover:text-fg md:hidden">
					<Icon name="list" size={20} />
				</button>
				<a
					href={homeHref()}
					class="truncate font-semibold whitespace-nowrap text-fg">
					Roblox API Reference
				</a>
				<SectionNav section={props.section} />
			</div>
			<SearchButton />
			<div class="hidden min-w-0 flex-1 items-center justify-end gap-3 lg:flex">
				<span
					data-tooltip={`version-${props.release.hash} (${props.release.version})`}
					class="min-w-0 truncate font-mono text-xs text-fg-faint">
					version-{props.release.hash} ({props.release.version})
				</span>
			</div>
		</header>
	)
}
