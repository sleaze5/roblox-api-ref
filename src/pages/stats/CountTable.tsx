import { countLabels } from "../../api/stats-text.ts"
import type { CountKind, CountRow } from "../../api/stats-types.ts"
import type { IconName } from "../../components/icon-names.ts"
import { Icon } from "../../components/Icon.tsx"

const countIcons: Record<CountKind, IconName> = {
	Class: "cube",
	Property: "sliders-horizontal",
	Method: "function",
	Event: "lightning",
	Callback: "arrow-u-up-left",
	Enum: "list-bullets",
	EnumItem: "hash",
	DataType: "brackets-curly",
}

function Header(props: { label: string; hint: string }) {
	return (
		<th scope="col" data-number="">
			<span
				data-tooltip={props.hint}
				class="cursor-help underline decoration-line-strong decoration-dotted underline-offset-4">
				{props.label}
			</span>
		</th>
	)
}

export function CountTable(props: { rows: CountRow[]; historyStart: string }) {
	const current = props.rows.reduce((sum, row) => sum + row.current, 0)
	const removed = props.rows.reduce((sum, row) => sum + row.removed, 0)
	return (
		<div class="stat-table">
			<table>
				<thead>
					<tr>
						<th scope="col">Kind</th>
						<Header label="Current" hint="Items in the current version." />
						<Header
							label="Removed"
							hint={`Items removed since ${props.historyStart} and not added back.`}
						/>
						<Header
							label="Total"
							hint="Current and removed items together."
						/>
					</tr>
				</thead>
				<tbody>
					{props.rows.map((row) => (
						<tr>
							<th scope="row">
								<span class="flex items-center gap-2">
									<Icon
										name={countIcons[row.kind]}
										size={14}
										class="text-fg-muted"
									/>
									{countLabels[row.kind]}
								</span>
							</th>
							<td data-number="">{row.current}</td>
							<td data-number="">{row.removed}</td>
							<td data-number="">{row.current + row.removed}</td>
						</tr>
					))}
				</tbody>
				<tfoot>
					<tr>
						<th scope="row">All</th>
						<td data-number="">{current}</td>
						<td data-number="">{removed}</td>
						<td data-number="">{current + removed}</td>
					</tr>
				</tfoot>
			</table>
		</div>
	)
}
