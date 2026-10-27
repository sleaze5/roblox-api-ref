import type { ItemHistory } from "../../api/item-history.ts"
import { updateHref } from "../../api/routes.ts"
import { formatDate } from "../../api/update-text.ts"
import { entryText } from "./history-text.ts"

export function HistoryList(props: {
	history: ItemHistory | undefined
	fragment: string
	class?: string
}) {
	return props.history === undefined ? null : (
		<details class={["history-disclosure", props.class]}>
			<summary>
				History
				<span class="text-fg-faint">{props.history.entries.length}</span>
			</summary>
			<ul class="flex flex-col gap-1">
				{props.history.entries.map((entry) => (
					<li class="flex flex-wrap items-baseline gap-x-3">
						<a
							href={`${updateHref(entry.version)}#${props.fragment}`}
							class="font-mono text-fg hover:text-fg-bright hover:underline">
							{entry.version}
						</a>
						<span class="text-xs text-fg-faint">
							{formatDate(entry.published)}
						</span>
						<span class="text-fg-muted">{entryText(entry)}</span>
					</li>
				))}
				{props.history.addedBefore === undefined ? null : (
					<li class="text-fg-faint">
						Added: not recorded (
						<span class="font-mono">{props.history.addedBefore}</span> or
						earlier)
					</li>
				)}
			</ul>
		</details>
	)
}
