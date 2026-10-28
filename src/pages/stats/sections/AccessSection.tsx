import { accessLead } from "../../../api/stats-text.ts"
import type { Stats } from "../../../api/stats-types.ts"
import { DataTable } from "../DataTable.tsx"
import { StatsSection } from "../StatsSection.tsx"

export function AccessSection(props: { access: Stats["access"] }) {
	return (
		<StatsSection id="access" title="Access" lead={accessLead(props.access.now)}>
			<DataTable
				caption="Access by year"
				columns={[
					{ label: "Year" },
					{
						label: "Added open",
						hint: "Members added that game scripts can use.",
						number: true,
					},
					{
						label: "Added restricted",
						hint: "Members added that need plugin, Roblox, or another security level.",
						number: true,
					},
					{
						label: "Opened",
						hint: "Members whose security changed so that game scripts can use them.",
						number: true,
					},
					{
						label: "Restricted",
						hint: "Members whose security changed so that game scripts can no longer use them.",
						number: true,
					},
					{
						label: "Made safe",
						hint: "Members whose thread safety changed from Unsafe to ReadSafe or Safe.",
						number: true,
					},
					{
						label: "Made unsafe",
						hint: "Members whose thread safety changed to Unsafe.",
						number: true,
					},
				]}
				rows={props.access.years.toReversed().map((year) => ({
					key: `access-${year.period}`,
					cells: [
						year.period,
						year.addedOpen,
						year.addedRestricted,
						year.opened,
						year.restricted,
						year.madeSafe,
						year.madeUnsafe,
					],
				}))}
			/>
		</StatsSection>
	)
}
