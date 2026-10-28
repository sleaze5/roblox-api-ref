import { changeTotal, monthLabel, monthsNote } from "../../api/stats-text.ts"
import type { PeriodTrend } from "../../api/stats-types.ts"
import { ColumnChart } from "./charts/ColumnChart.tsx"
import { StatsSection } from "./StatsSection.tsx"
import { TrendTable } from "./TrendTable.tsx"

function listText(labels: string[]): string {
	return labels.length <= 2
		? labels.join(" and ")
		: `${labels.slice(0, -1).join(", ")}, and ${labels.at(-1) ?? ""}`
}

function extremesText(
	months: PeriodTrend[],
	value: (month: PeriodTrend) => number,
): string {
	const values = months.map(value)
	const labelsOf = (target: number) =>
		listText(
			months
				.filter((month) => value(month) === target)
				.map((month) => monthLabel(month.period)),
		)
	return `Most in ${labelsOf(Math.max(...values))}, fewest in ${labelsOf(Math.min(...values))}`
}

function hint(month: PeriodTrend): string {
	const changes = changeTotal(month.tallies)
	return `${monthLabel(month.period)}: ${changes} changes in ${month.updates} updates, ${Math.round(changes / month.updates)} per update`
}

export function MonthTrends(props: { months: PeriodTrend[] }) {
	return (
		<StatsSection id="changes-by-month" title="Changes by month" lead={monthsNote}>
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
				<ColumnChart
					title="Changes"
					summary={extremesText(props.months, (month) =>
						changeTotal(month.tallies),
					)}
					columns={props.months.map((month) => ({
						label: monthLabel(month.period).slice(0, 1),
						value: changeTotal(month.tallies),
						hint: hint(month),
					}))}
				/>
				<ColumnChart
					title="Updates"
					summary={extremesText(props.months, (month) => month.updates)}
					columns={props.months.map((month) => ({
						label: monthLabel(month.period).slice(0, 1),
						value: month.updates,
						hint: hint(month),
					}))}
				/>
			</div>
			<TrendTable
				caption="Changes by month"
				headingLabel="Month"
				detailLabel="Updates"
				rows={props.months.map((month) => ({
					key: `month-${month.period}`,
					heading: monthLabel(month.period),
					detail: month.updates,
					tallies: month.tallies,
				}))}
			/>
		</StatsSection>
	)
}
