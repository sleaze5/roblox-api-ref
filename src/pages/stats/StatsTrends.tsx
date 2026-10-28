import { updateHref } from "../../api/routes.ts"
import type { Stats, UpdateTrend } from "../../api/stats-types.ts"
import { formatDate } from "../../api/update-text.ts"
import { DivergingBarChart } from "./charts/DivergingBarChart.tsx"
import { LineChart } from "./charts/LineChart.tsx"
import { yearTicks } from "./charts/chart-ticks.ts"
import { trendsNote } from "../../api/stats-text.ts"
import { MonthTrends } from "./MonthTrends.tsx"
import { StatsSection } from "./StatsSection.tsx"
import { TrendTable } from "./TrendTable.tsx"

export const trendSections = [
	{ id: "api-size", label: "API size" },
	{ id: "members-per-update", label: "Members per update" },
	{ id: "changes-by-year", label: "Changes by year" },
	{ id: "changes-by-month", label: "Changes by month" },
	{ id: "changes-by-update", label: "Changes by update" },
] as const

function pointHint(trend: UpdateTrend, value: number, noun: string): string {
	return `${trend.version} (${formatDate(trend.published)}): ${value} ${noun}`
}

function SizeChart(props: { updates: UpdateTrend[]; kind: "classes" | "enums" }) {
	const first = props.updates[0]
	const last = props.updates.at(-1)
	const title = props.kind === "classes" ? "Classes" : "Enums"
	return first === undefined || last === undefined ? null : (
		<LineChart
			title={title}
			summary={`${first[props.kind]} to ${last[props.kind]}`}
			points={props.updates.map((trend) => ({
				value: trend[props.kind],
				hint: pointHint(trend, trend[props.kind], props.kind),
			}))}
			ticks={yearTicks(props.updates.map((trend) => trend.published))}
		/>
	)
}

function memberSummary(updates: UpdateTrend[]): string {
	const added = updates.reduce((sum, trend) => sum + trend.tallies.Member.added, 0)
	const removed = updates.reduce(
		(sum, trend) => sum + trend.tallies.Member.removed,
		0,
	)
	return `${added} added, ${removed} removed`
}

export function StatsTrends(props: { stats: Stats }) {
	const updates = props.stats.updates
	return (
		<>
			<StatsSection
				id="api-size"
				title="API size"
				lead="The number of classes and enums after each update.">
				<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
					<SizeChart updates={updates} kind="classes" />
					<SizeChart updates={updates} kind="enums" />
				</div>
			</StatsSection>
			<StatsSection
				id="members-per-update"
				title="Members per update"
				lead="The members that each update added and removed. A number marks a bar that runs past the axis.">
				<DivergingBarChart
					title="Members added and removed"
					summary={memberSummary(updates)}
					upLabel="Added"
					downLabel="Removed"
					columns={updates.map((trend) => ({
						up: trend.tallies.Member.added,
						down: trend.tallies.Member.removed,
						hint: `${trend.version} (${formatDate(trend.published)}): ${trend.tallies.Member.added} added, ${trend.tallies.Member.removed} removed`,
					}))}
					ticks={yearTicks(updates.map((trend) => trend.published))}
				/>
			</StatsSection>
			<StatsSection
				id="changes-by-year"
				title="Changes by year"
				lead={trendsNote}>
				<TrendTable
					caption="Changes by year"
					headingLabel="Year"
					detailLabel="Updates"
					rows={props.stats.years.toReversed().map((year) => ({
						key: `year-${year.period}`,
						heading: year.period,
						detail: year.updates,
						tallies: year.tallies,
					}))}
				/>
			</StatsSection>
			<MonthTrends months={props.stats.months} />
			<StatsSection
				id="changes-by-update"
				title="Changes by update"
				lead="Every tracked update, newest first.">
				<TrendTable
					caption="Changes by update"
					headingLabel="Update"
					detailLabel="Published"
					rows={updates.toReversed().map((trend) => ({
						key: `update-${trend.version}`,
						heading: (
							<a
								href={updateHref(trend.version)}
								class="font-mono font-normal hover:text-fg-bright hover:underline">
								{trend.version}
							</a>
						),
						detail: formatDate(trend.published),
						tallies: trend.tallies,
					}))}
				/>
			</StatsSection>
		</>
	)
}
