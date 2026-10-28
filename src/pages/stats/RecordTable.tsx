import { statRefTokens } from "../../api/stat-ref.ts"
import { refNoun } from "../../api/stats-text.ts"
import type { StatRecord, StatRef } from "../../api/stats-types.ts"
import { CodeTokens } from "../../components/code/CodeTokens.tsx"

const shownItems = 8

function ItemList(props: { items: StatRef[] }) {
	return (
		<ul class="flex flex-wrap gap-x-4 gap-y-1">
			{props.items.map((item) => (
				<li class="min-w-0 break-all">
					<code>
						<CodeTokens tokens={statRefTokens(item)} />
					</code>
				</li>
			))}
		</ul>
	)
}

function RecordItems(props: { items: StatRef[] }) {
	const [first] = props.items
	if (first === undefined) {
		return null
	}
	return props.items.length <= shownItems ? (
		<ItemList items={props.items} />
	) : (
		<details class="change-previous">
			<summary>
				{props.items.length} {refNoun(first.kind)}
			</summary>
			<ItemList items={props.items} />
		</details>
	)
}

export function RecordTable(props: { records: StatRecord[] }) {
	return (
		<div class="stat-table">
			<table data-records="">
				<colgroup>
					<col data-label="" />
					<col data-value="" />
					<col />
				</colgroup>
				<thead>
					<tr>
						<th scope="col">Statistic</th>
						<th scope="col" data-number="">
							Value
						</th>
						<th scope="col">Items</th>
					</tr>
				</thead>
				<tbody>
					{props.records.map((record) => (
						<tr id={record.id}>
							<th scope="row" data-wrap="">
								<span
									data-tooltip={record.hint}
									class="cursor-help font-normal underline decoration-line-strong decoration-dotted underline-offset-4">
									{record.label}
								</span>
							</th>
							<td data-number="">{record.value}</td>
							<td data-wrap="">
								<RecordItems items={record.items} />
							</td>
						</tr>
					))}
				</tbody>
			</table>
		</div>
	)
}
