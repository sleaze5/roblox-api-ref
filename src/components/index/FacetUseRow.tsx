import { useHref, useParts, useText } from "../../api/facets/facet-use.ts"
import type { FacetUse } from "../../api/types.ts"

function nameTone(use: FacetUse): string {
	switch (use.kind) {
		case "Class":
		case "Enum":
			return "text-code-type"
		case "EnumItem":
			return "text-code-constant"
		default:
			return "text-fg-bright"
	}
}

export function FacetUseRow(props: { use: FacetUse }) {
	const parts = useParts(props.use)
	const access = "access" in props.use ? props.use.access : undefined
	return (
		<li data-name={useText(props.use)}>
			<a
				href={useHref(props.use)}
				class="flex h-7 items-center gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover">
				<span
					aria-hidden="true"
					data-kind={props.use.kind}
					class="kind-icon text-fg-muted"
				/>
				<span class="truncate">
					{parts.prefix}
					{parts.owner === undefined ? null : (
						<span class="text-code-type">{parts.owner}</span>
					)}
					{parts.separator}
					<span class={nameTone(props.use)}>{parts.name}</span>
				</span>
				{access === undefined ? null : (
					<span class="ml-auto shrink-0 font-sans text-xs text-fg-muted">
						{access} access
					</span>
				)}
			</a>
		</li>
	)
}
