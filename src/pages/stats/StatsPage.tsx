import type { Api } from "../../api/types.ts"
import { comebacksLead, removedNote, statsLead } from "../../api/stats-text.ts"
import type { Stats } from "../../api/stats-types.ts"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { CountTable } from "./CountTable.tsx"
import { RecordTable } from "./RecordTable.tsx"
import { StatsSection } from "./StatsSection.tsx"
import { StatsTrends, trendSections } from "./StatsTrends.tsx"
import { ComebackTable } from "./sections/ComebackTable.tsx"
import { ExtraSections, extraSections } from "./sections/ExtraSections.tsx"

export function StatsPage(props: { api: Api; stats: Stats }) {
	const stats = props.stats
	return (
		<Shell
			section="stats"
			release={props.api}
			rail={
				<PageRail
					links={[
						{ id: "counts", label: "Counts" },
						...stats.records.map((group) => ({
							id: group.id,
							label: group.title,
						})),
						...extraSections,
						...trendSections,
					]}
				/>
			}>
			<article class="flex flex-col gap-10">
				<PageHeader crumbs={rootCrumbs} icon="chart-bar" title="Statistics">
					<PageLead>{statsLead(props.api)}</PageLead>
					<FactList>
						<Fact
							label="Version"
							hint="The Roblox Studio version that the current numbers describe.">
							<span class="font-mono">{props.api.version}</span>
						</Fact>
						<Fact
							label="Tracked since"
							hint="The oldest version in the update history.">
							<span class="font-mono">{stats.historyStart}</span>
						</Fact>
						<Fact label="Updates" hint="Updates in the tracked history.">
							{stats.updates.length}
						</Fact>
					</FactList>
				</PageHeader>
				<StatsSection
					id="counts"
					title="Counts"
					lead={removedNote(stats.historyStart)}>
					<CountTable rows={stats.counts} historyStart={stats.historyStart} />
				</StatsSection>
				{stats.records.map((group) => (
					<StatsSection
						id={group.id}
						title={group.title}
						lead={
							group.id === "comeback-records" ? comebacksLead : undefined
						}>
						<RecordTable records={group.records} />
						{group.id === "comeback-records" ? (
							<ComebackTable batches={stats.comebacks} />
						) : null}
					</StatsSection>
				))}
				<ExtraSections api={props.api} stats={stats} />
				<StatsTrends stats={stats} />
			</article>
		</Shell>
	)
}
