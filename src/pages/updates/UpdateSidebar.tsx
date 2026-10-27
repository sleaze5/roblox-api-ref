import { uploadName } from "../../api/release.ts"
import { updateHref } from "../../api/routes.ts"
import { formatShortDate } from "../../api/update-text.ts"
import type { Update } from "../../api/update-types.ts"
import { SectionSidebar } from "../../layout/SectionSidebar.tsx"
import { updateCalendar, type UpdateMonth } from "./update-calendar.ts"

function monthHolds(month: UpdateMonth, version: string): boolean {
	return month.updates.some((update) => update.version === version)
}

function UpdateLeaf(props: { update: Update; current: string | undefined }) {
	const upload = uploadName(props.update.hash)
	return (
		<li data-name={props.update.version} data-aliases={upload}>
			<a
				href={updateHref(props.update.version)}
				aria-current={
					props.current === props.update.version ? "page" : undefined
				}
				data-tooltip={upload}>
				<span>{props.update.version}</span>
				<small>{formatShortDate(props.update.published)}</small>
			</a>
		</li>
	)
}

function MonthNode(props: {
	month: UpdateMonth
	open: boolean
	current: string | undefined
}) {
	return (
		<details open={props.open}>
			<summary style="--depth: 1">
				<span>{props.month.title}</span>
				<small>{props.month.updates.length}</small>
			</summary>
			<ul>
				{props.month.updates.map((update) => (
					<UpdateLeaf update={update} current={props.current} />
				))}
			</ul>
		</details>
	)
}

export function UpdateSidebar(props: { updates: Update[]; current?: string }) {
	const years = updateCalendar(props.updates)
	const current = props.current
	const openMonth =
		current === undefined
			? years[0]?.months[0]
			: years
					.flatMap((year) => year.months)
					.find((month) => monthHolds(month, current))
	return (
		<SectionSidebar label="Filter updates" target="sidebar-tree">
			<ul
				id="sidebar-tree"
				data-tree=""
				data-sticky-tree=""
				class="tree-sticky class-tree tree-update-icons">
				{years.map((year) => (
					<li>
						<details
							open={
								openMonth !== undefined &&
								year.months.includes(openMonth)
							}>
							<summary style="--depth: 0">
								<span>{year.year}</span>
								<small>{year.count}</small>
							</summary>
							<ul>
								{year.months.map((month) => (
									<li>
										<MonthNode
											month={month}
											open={month === openMonth}
											current={current}
										/>
									</li>
								))}
							</ul>
						</details>
					</li>
				))}
			</ul>
		</SectionSidebar>
	)
}
