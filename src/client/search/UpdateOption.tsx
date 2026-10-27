import { For, Show } from "solid-js"
import { uploadName } from "../../api/release.ts"
import { updateHref } from "../../api/routes.ts"
import { formatDate } from "../../api/update-text.ts"
import { Icon } from "../../components/Icon.tsx"
import type { SearchUpdate } from "../../search/index-format.ts"
import { optionId } from "./ResultOption.tsx"
import { activeResult, setActiveResult } from "./search-state.ts"

export function UpdateOption(props: { update: SearchUpdate; index: number }) {
	return (
		<a
			id={optionId(props.index)}
			role="option"
			aria-selected={activeResult() === props.index ? "true" : "false"}
			href={updateHref(props.update.version)}
			onPointerMove={() => setActiveResult(props.index)}
			class="flex h-8 items-center gap-2 rounded-sm px-2 text-fg-muted aria-selected:bg-hover aria-selected:text-fg">
			<Icon name="git-diff" size={14} class="text-fg-faint" />
			<span class="shrink-0 font-mono text-fg">{props.update.version}</span>
			<span class="truncate font-mono text-fg-faint">
				{uploadName(props.update.hash)}
			</span>
			<span class="ml-auto shrink-0 text-xs text-fg-faint">
				{formatDate(props.update.published)}
			</span>
		</a>
	)
}

export function UpdateGroup(props: { updates: SearchUpdate[] }) {
	return (
		<Show when={props.updates.length > 0}>
			<div role="group" aria-label="Updates" class="pb-1">
				<p
					aria-hidden="true"
					class="px-2 pt-2 pb-1 text-xs font-medium text-fg-muted">
					Updates
				</p>
				<For each={props.updates} keyed={false}>
					{(update, index) => (
						<UpdateOption update={update()} index={index} />
					)}
				</For>
			</div>
		</Show>
	)
}
