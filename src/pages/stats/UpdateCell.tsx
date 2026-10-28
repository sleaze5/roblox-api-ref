import { updateHref } from "../../api/routes.ts"
import type { UpdateEvent } from "../../api/stats-types.ts"
import { formatDate } from "../../api/update-text.ts"

export function UpdateCell(props: { update: UpdateEvent | undefined }) {
	return props.update === undefined ? (
		<span class="text-fg-faint">Not recorded</span>
	) : (
		<span class="flex flex-wrap items-baseline gap-x-2">
			<a
				href={updateHref(props.update.version)}
				class="font-mono font-normal text-fg hover:text-fg-bright hover:underline">
				{props.update.version}
			</a>
			<span class="text-xs text-fg-faint">
				{formatDate(props.update.published)}
			</span>
		</span>
	)
}
