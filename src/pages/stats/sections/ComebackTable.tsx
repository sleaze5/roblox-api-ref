import type { ComebackBatch } from "../../../api/stats-types.ts"
import { DataTable } from "../DataTable.tsx"
import { UpdateCell } from "../UpdateCell.tsx"

export function ComebackTable(props: { batches: ComebackBatch[] }) {
	return (
		<DataTable
			caption="Comebacks"
			columns={[
				{ label: "Removed in" },
				{ label: "Added back in" },
				{ label: "Items", number: true },
				{ label: "Days gone", number: true },
			]}
			rows={props.batches.map((batch) => ({
				key: `comeback-${batch.removed.version}-${batch.returned.version}`,
				cells: [
					<UpdateCell update={batch.removed} />,
					<UpdateCell update={batch.returned} />,
					batch.count,
					batch.days,
				],
			}))}
		/>
	)
}
