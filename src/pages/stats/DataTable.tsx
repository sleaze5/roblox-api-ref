import type { JSX } from "@solidjs/web"

export type DataColumn = {
	label: string
	hint?: string
	number?: boolean
}

export type DataRow = {
	key: string
	cells: JSX.Element[]
}

function Heading(props: { column: DataColumn }) {
	return (
		<th scope="col" data-number={props.column.number ? "" : undefined}>
			{props.column.hint === undefined ? (
				props.column.label
			) : (
				<span
					data-tooltip={props.column.hint}
					class="cursor-help underline decoration-line-strong decoration-dotted underline-offset-4">
					{props.column.label}
				</span>
			)}
		</th>
	)
}

export function DataTable(props: {
	caption: string
	columns: DataColumn[]
	rows: DataRow[]
}) {
	return (
		<div class="stat-table">
			<table>
				<caption class="sr-only">{props.caption}</caption>
				<thead>
					<tr>
						{props.columns.map((column) => (
							<Heading column={column} />
						))}
					</tr>
				</thead>
				<tbody>
					{props.rows.map((row) => (
						<tr id={row.key}>
							{row.cells.map((cell, index) =>
								index === 0 ? (
									<th scope="row">{cell}</th>
								) : (
									<td
										data-number={
											props.columns[index]?.number
												? ""
												: undefined
										}>
										{cell}
									</td>
								),
							)}
						</tr>
					))}
				</tbody>
			</table>
		</div>
	)
}
