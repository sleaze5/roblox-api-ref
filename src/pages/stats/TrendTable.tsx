import type { JSX } from "@solidjs/web"
import { deprecatedTotal } from "../../api/stats-text.ts"
import type { Tallies } from "../../api/stats-types.ts"
import { TallyCell } from "./TallyCell.tsx"

export type TrendRow = {
	key: string
	heading: JSX.Element
	detail: JSX.Element
	tallies: Tallies
}

export function TrendTable(props: {
	caption: string
	headingLabel: string
	detailLabel: string
	rows: TrendRow[]
}) {
	return (
		<div class="stat-table">
			<table>
				<caption class="sr-only">{props.caption}</caption>
				<thead>
					<tr>
						<th scope="col">{props.headingLabel}</th>
						<th scope="col">{props.detailLabel}</th>
						<th scope="col">Classes</th>
						<th scope="col">Members</th>
						<th scope="col">Enums</th>
						<th scope="col">Enum items</th>
						<th scope="col" data-number="">
							Deprecated
						</th>
					</tr>
				</thead>
				<tbody>
					{props.rows.map((row) => {
						const deprecated = deprecatedTotal(row.tallies)
						return (
							<tr id={row.key}>
								<th scope="row">{row.heading}</th>
								<td>{row.detail}</td>
								<TallyCell tally={row.tallies.Class} />
								<TallyCell tally={row.tallies.Member} changes />
								<TallyCell tally={row.tallies.Enum} />
								<TallyCell tally={row.tallies.EnumItem} changes />
								<td
									data-number=""
									class={
										deprecated === 0 ? "text-fg-faint" : undefined
									}>
									{deprecated}
								</td>
							</tr>
						)
					})}
				</tbody>
			</table>
		</div>
	)
}
