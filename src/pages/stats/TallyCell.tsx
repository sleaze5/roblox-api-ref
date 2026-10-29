import type { ChangeTally } from "../../api/stats-types.ts"
import { ChangeCounts } from "../../components/ChangeCounts.tsx"

export function TallyCell(props: { tally: ChangeTally; changes?: boolean }) {
	const empty =
		props.tally.added === 0 &&
		props.tally.removed === 0 &&
		(!props.changes || props.tally.changed === 0)
	return (
		<td>
			{empty ? (
				<span class="text-fg-faint">0</span>
			) : (
				<ChangeCounts counts={props.tally} changes={props.changes ?? false} />
			)}
		</td>
	)
}
