import type { CodeToken } from "../../../api/code-token.ts"
import type { Doc } from "../../../api/doc-types.ts"
import { CodeTokens } from "../../code/CodeTokens.tsx"
import { DocText } from "../../doc/DocText.tsx"

export type DetailRow = {
	code: CodeToken[]
	description?: Doc | undefined
}

export function DetailTable(props: {
	caption: string
	heading: string
	rows: DetailRow[]
}) {
	const described = props.rows.some((row) => row.description !== undefined)
	return props.rows.length === 0 ? null : (
		<div class="detail-table">
			<table>
				<caption>{props.caption}</caption>
				{described ? (
					<colgroup>
						<col data-code="" />
						<col />
					</colgroup>
				) : null}
				<thead>
					<tr>
						<th scope="col">{props.heading}</th>
						{described ? <th scope="col">Description</th> : null}
					</tr>
				</thead>
				<tbody>
					{props.rows.map((row) => (
						<tr>
							<td data-code="">
								<code>
									<CodeTokens tokens={row.code} />
								</code>
							</td>
							{described ? (
								<td>
									<DocText doc={row.description} />
								</td>
							) : null}
						</tr>
					))}
				</tbody>
			</table>
		</div>
	)
}
