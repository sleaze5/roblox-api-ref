export type ChangeCountValues = {
	added: number
	removed: number
	changed: number
}

const parts = [
	{ key: "added", mark: "+", label: "Added", class: "text-success" },
	{ key: "removed", mark: "−", label: "Removed", class: "text-danger" },
	{ key: "changed", mark: "~", label: "Changed", class: "text-fg-faint" },
] as const

export function ChangeCounts(props: { counts: ChangeCountValues; changes?: boolean }) {
	const shown = parts.filter(
		(part) =>
			props.counts[part.key] > 0 &&
			(props.changes !== false || part.key !== "changed"),
	)
	return shown.length === 0 ? null : (
		<span class="flex gap-2 tabular-nums">
			{shown.map((part) => (
				<span>
					<span aria-hidden="true" class={["font-mono", part.class]}>
						{part.mark}
					</span>
					<span class="sr-only">{part.label} </span>
					{props.counts[part.key]}
				</span>
			))}
		</span>
	)
}
