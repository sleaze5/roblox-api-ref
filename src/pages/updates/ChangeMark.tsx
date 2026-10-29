import type { ChangeStatus } from "../../api/update-types.ts"

export function ChangeMark(props: {
	status: ChangeStatus | undefined
	label?: string | undefined
}) {
	return (
		<span class="change-mark" data-status={props.status}>
			{props.label === undefined ? null : (
				<span class="sr-only">{props.label}</span>
			)}
		</span>
	)
}
