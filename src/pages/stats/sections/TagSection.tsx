import { facetHref } from "../../../api/routes.ts"
import type { Api, FacetKind } from "../../../api/types.ts"
import type { TagTrend } from "../../../api/stats-types.ts"
import { DataTable } from "../DataTable.tsx"
import { StatsSection } from "../StatsSection.tsx"
import { UpdateCell } from "../UpdateCell.tsx"

function Name(props: { api: Api; kind: FacetKind; name: string }) {
	return Object.hasOwn(props.api.facets[props.kind], props.name) ? (
		<a
			href={facetHref(props.kind, props.name)}
			class="font-mono font-normal text-fg hover:text-fg-bright hover:underline">
			{props.name}
		</a>
	) : (
		<span class="font-mono font-normal text-fg-muted">{props.name}</span>
	)
}

export function TagSection(props: {
	api: Api
	id: string
	title: string
	lead: string
	kind: FacetKind
	label: string
	rows: TagTrend[]
}) {
	return (
		<StatsSection id={props.id} title={props.title} lead={props.lead}>
			<DataTable
				caption={props.title}
				columns={[
					{ label: props.label },
					{ label: "Current", hint: "Items that have it now.", number: true },
					{
						label: "Gained",
						hint: "Times an update added an item with it or gave it to an item.",
						number: true,
					},
					{
						label: "Lost",
						hint: "Times an update removed an item with it or took it from an item.",
						number: true,
					},
					{
						label: "First gained",
						hint: "The first tracked update that gave it to an item.",
					},
				]}
				rows={props.rows.map((row) => ({
					key: `${props.id}-${row.name}`,
					cells: [
						<Name api={props.api} kind={props.kind} name={row.name} />,
						row.current,
						row.gained,
						row.lost,
						<UpdateCell update={row.first} />,
					],
				}))}
			/>
		</StatsSection>
	)
}
