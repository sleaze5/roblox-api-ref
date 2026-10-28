import type { ChangeTally } from "../../api/stats-types.ts"

const parts = [
	{ key: "added", mark: "+", label: "Added", class: "text-success" },
	{ key: "removed", mark: "−", label: "Removed", class: "text-danger" },
	{ key: "changed", mark: "~", label: "Changed", class: "text-fg-faint" },
] as const

export function TallyCell(props: { tally: ChangeTally; changes?: boolean }) {
	const shown = parts.filter(
		(part) =>
			props.tally[part.key] > 0 && (props.changes || part.key !== "changed"),
	)
	return (
		<td>
			{shown.length === 0 ? (
				<span class="text-fg-faint">0</span>
			) : (
				<span class="flex gap-2 tabular-nums">
					{shown.map((part) => (
						<span>
							<span aria-hidden="true" class={["font-mono", part.class]}>
								{part.mark}
							</span>
							<span class="sr-only">{part.label} </span>
							{props.tally[part.key]}
						</span>
					))}
				</span>
			)}
		</td>
	)
}
