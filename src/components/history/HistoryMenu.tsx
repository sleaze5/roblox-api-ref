import type { ItemHistory } from "../../api/item-history.ts"
import { updateHref } from "../../api/routes.ts"
import { formatDate } from "../../api/update-text.ts"
import { entryText } from "./history-text.ts"

function menuId(id: string): string {
	return `history-${id}`
}

export function HistoryButton(props: {
	history: ItemHistory | undefined
	id: string
	name: string
}) {
	const id = menuId(props.id)
	return props.history === undefined ? null : (
		<button
			type="button"
			popovertarget={id}
			data-tooltip="History"
			aria-label={`History of ${props.name}`}
			style={`anchor-name: --${id}`}
			class="history-link"
		/>
	)
}

export function HistoryMenu(props: {
	history: ItemHistory | undefined
	id: string
	name: string
	fragment: string
}) {
	const id = menuId(props.id)
	return props.history === undefined ? null : (
		<div
			id={id}
			popover="auto"
			role="dialog"
			aria-label={`History of ${props.name}`}
			style={`position-anchor: --${id}`}
			class="menu">
			<p class="border-b border-line menu-label">History</p>
			<ul class="flex max-h-80 flex-col overflow-y-auto overscroll-contain p-1">
				{props.history.entries.map((entry) => (
					<li>
						<a
							href={`${updateHref(entry.version)}#${props.fragment}`}
							class="flex flex-col rounded-sm px-2 py-1.5 text-fg-muted motion-colors hover:bg-hover hover:text-fg">
							<span class="flex items-baseline justify-between gap-3">
								<span class="font-mono text-fg">{entry.version}</span>
								<span class="text-xs text-fg-faint">
									{formatDate(entry.published)}
								</span>
							</span>
							<span class="text-xs">{entryText(entry)}</span>
						</a>
					</li>
				))}
			</ul>
		</div>
	)
}
