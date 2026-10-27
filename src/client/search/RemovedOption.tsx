import { For, Show } from "solid-js"
import { updateHref } from "../../api/routes.ts"
import { changeAnchor, ownerAnchor } from "../../api/update-text.ts"
import type { RemovedEntry } from "../../search/index-format.ts"
import { KindIcon } from "./KindIcon.tsx"
import { optionId } from "./ResultOption.tsx"
import { activeResult, setActiveResult } from "./search-state.ts"

export function removedHref(entry: RemovedEntry): string {
	const anchor =
		entry.parent === undefined
			? ownerAnchor({
					kind: entry.kind === "Enum" ? "Enum" : "Class",
					name: entry.name,
				})
			: changeAnchor(
					entry.kind === "EnumItem" ? "Enum" : "Class",
					entry.parent,
					entry.name,
				)
	return `${updateHref(entry.version)}#${anchor}`
}

function RemovedOption(props: { entry: RemovedEntry; index: number }) {
	return (
		<a
			id={optionId(props.index)}
			role="option"
			aria-selected={activeResult() === props.index ? "true" : "false"}
			href={removedHref(props.entry)}
			onPointerMove={() => setActiveResult(props.index)}
			class="flex h-8 items-center gap-2 rounded-sm px-2 text-fg-muted aria-selected:bg-hover aria-selected:text-fg">
			<KindIcon kind={props.entry.kind} class="text-fg-faint" />
			<span class="shrink-0 font-mono text-fg-muted line-through">
				{props.entry.name}
			</span>
			<span class="truncate font-mono text-fg-faint">
				{props.entry.parent ?? ""}
			</span>
			<span class="ml-auto shrink-0 text-xs text-fg-faint">
				Removed in {props.entry.version}
			</span>
		</a>
	)
}

export function RemovedGroup(props: { entries: RemovedEntry[]; start: number }) {
	return (
		<Show when={props.entries.length > 0}>
			<div role="group" aria-label="Removed" class="pb-1">
				<p
					aria-hidden="true"
					class="px-2 pt-2 pb-1 text-xs font-medium text-fg-muted">
					Removed
				</p>
				<For each={props.entries} keyed={false}>
					{(entry, index) => (
						<RemovedOption entry={entry()} index={props.start + index} />
					)}
				</For>
			</div>
		</Show>
	)
}
