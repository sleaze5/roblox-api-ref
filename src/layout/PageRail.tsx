export type RailEntry = {
	id: string
	label: string
}

export type RailLink = RailEntry & {
	count: number
	countHint?: string
	entries?: RailEntry[]
}

function RailEntries(props: { entries: RailEntry[] | undefined }) {
	return props.entries === undefined || props.entries.length === 0 ? null : (
		<ul
			data-rail-entries=""
			class="ml-2 flex flex-col border-l border-line-subtle pl-1">
			{props.entries.map((entry) => (
				<li>
					<a
						href={`#${entry.id}`}
						data-rail-entry={entry.id}
						class="flex h-7 items-center rounded-sm px-2 font-mono text-fg-muted motion-colors hover:bg-hover hover:text-fg data-active:bg-active data-active:text-fg-bright">
						<span class="truncate">{entry.label}</span>
					</a>
				</li>
			))}
		</ul>
	)
}

export function PageRail(props: { links: RailLink[] }) {
	return (
		<nav data-rail="" aria-labelledby="rail-title" class="flex flex-col gap-1">
			<p id="rail-title" class="px-2 pb-1 text-xs font-medium text-fg-muted">
				On this page
			</p>
			{props.links.map((link) => (
				<div data-rail-section={link.id} class="group/rail flex flex-col">
					<a
						href={`#${link.id}`}
						class="flex h-7 items-center justify-between rounded-sm px-2 text-fg-muted motion-colors group-data-active/rail:text-fg hover:bg-hover hover:text-fg">
						<span>{link.label}</span>
						<span
							data-tooltip={link.countHint}
							class="text-xs text-fg-faint">
							{link.count}
						</span>
					</a>
					<RailEntries entries={link.entries} />
				</div>
			))}
		</nav>
	)
}
