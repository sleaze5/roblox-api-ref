import type { ChangeStatus } from "../../api/update-types.ts"

const marks: Record<ChangeStatus, string> = {
	Added: "+",
	Removed: "−",
	Changed: "~",
}

export function ChangeMark(props: {
	status: ChangeStatus | undefined
	label?: string | undefined
}) {
	return (
		<span class="change-mark" data-status={props.status}>
			<span aria-hidden="true">
				{props.status === undefined ? "" : marks[props.status]}
			</span>
			{props.label === undefined ? null : (
				<span class="sr-only">{props.label}</span>
			)}
		</span>
	)
}
