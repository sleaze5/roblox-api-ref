import { coverageLabels, docsLead, percentText } from "../../../api/stats-text.ts"
import type { Stats } from "../../../api/stats-types.ts"
import { DataTable } from "../DataTable.tsx"
import { StatsSection } from "../StatsSection.tsx"

export function DocsSection(props: { docs: Stats["docs"] }) {
	return (
		<StatsSection id="descriptions" title="Descriptions" lead={docsLead}>
			<DataTable
				caption="Description coverage"
				columns={[
					{ label: "Kind" },
					{ label: "Described", number: true },
					{ label: "Total", number: true },
					{ label: "Share", number: true },
				]}
				rows={props.docs.coverage.map((row) => ({
					key: `coverage-${row.kind}`,
					cells: [
						coverageLabels[row.kind],
						row.described,
						row.total,
						percentText(row.described, row.total),
					],
				}))}
			/>
			<DataTable
				caption="Description changes by year"
				columns={[
					{ label: "Year" },
					{
						label: "Added",
						hint: "Descriptions written where there was none.",
						number: true,
					},
					{ label: "Changed", hint: "Descriptions rewritten.", number: true },
					{
						label: "Removed",
						hint: "Descriptions taken away.",
						number: true,
					},
				]}
				rows={props.docs.years.toReversed().map((year) => ({
					key: `descriptions-${year.period}`,
					cells: [year.period, year.added, year.changed, year.removed],
				}))}
			/>
		</StatsSection>
	)
}
