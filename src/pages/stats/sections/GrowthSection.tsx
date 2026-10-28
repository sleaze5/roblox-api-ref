import { classHref } from "../../../api/routes.ts"
import { growthLead } from "../../../api/stats-text.ts"
import type { GrowthPeriod } from "../../../api/stats-types.ts"
import type { Api } from "../../../api/types.ts"
import { DataTable } from "../DataTable.tsx"
import { StatsSection } from "../StatsSection.tsx"

function ClassName(props: { api: Api; name: string }) {
	return Object.hasOwn(props.api.classes, props.name) ? (
		<a href={classHref(props.name)} class="text-code-type hover:underline">
			{props.name}
		</a>
	) : (
		<span class="text-fg-muted">{props.name}</span>
	)
}

export function GrowthSection(props: { api: Api; growth: GrowthPeriod[] }) {
	return (
		<StatsSection id="growth" title="Growth by class" lead={growthLead}>
			<DataTable
				caption="Growth by class"
				columns={[{ label: "Year" }, { label: "Classes that grew the most" }]}
				rows={props.growth.toReversed().map((year) => ({
					key: `growth-${year.period}`,
					cells: [
						year.period,
						<ul class="flex flex-wrap gap-x-4 gap-y-1 whitespace-normal">
							{year.classes.map((item) => (
								<li>
									<code>
										<ClassName api={props.api} name={item.name} />
									</code>{" "}
									<span class="text-success">+</span>
									{item.added}
								</li>
							))}
						</ul>,
					],
				}))}
			/>
		</StatsSection>
	)
}
