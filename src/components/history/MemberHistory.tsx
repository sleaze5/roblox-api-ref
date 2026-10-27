import type { HistoryEvent, ItemHistory } from "../../api/item-history.ts"
import { formatDate } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { historyHref } from "./history-href.ts"

export type HistoryVariant = "chip" | "hover" | "line" | "icon"

export const historyVariant: HistoryVariant = "icon"

const chipLabels = { added: "New", changed: "Changed" } as const

function EventLink(props: {
	label: string
	event: HistoryEvent
	kind: OwnerKind
	owner: string
}) {
	return (
		<span class="flex items-baseline gap-1.5">
			<span>{props.label}</span>
			<a
				href={historyHref(props.event, props.kind, props.owner)}
				data-tooltip={formatDate(props.event.published)}
				class="font-mono text-fg-muted hover:text-fg hover:underline">
				{props.event.version}
			</a>
		</span>
	)
}

function Events(props: { history: ItemHistory; kind: OwnerKind; owner: string }) {
	return (
		<>
			{props.history.added === undefined ? null : (
				<EventLink
					label="Added"
					event={props.history.added}
					kind={props.kind}
					owner={props.owner}
				/>
			)}
			{props.history.changed === undefined ? null : (
				<EventLink
					label="Changed"
					event={props.history.changed}
					kind={props.kind}
					owner={props.owner}
				/>
			)}
		</>
	)
}

function eventText(label: string, event: HistoryEvent | undefined): string[] {
	return event === undefined
		? []
		: [`${label} in ${event.version} on ${formatDate(event.published)}.`]
}

export function HistoryAction(props: {
	history: ItemHistory | undefined
	name: string
	kind: OwnerKind
	owner: string
}) {
	const history = props.history
	const latest = history?.changed ?? history?.added
	return history === undefined || latest === undefined ? null : (
		<a
			href={historyHref(latest, props.kind, props.owner)}
			data-tooltip={[
				...eventText("Added", history.added),
				...eventText("Last changed", history.changed),
				...eventText("Description changed", history.described),
			].join(" ")}
			aria-label={`Open the last update that changed ${props.name}`}
			class="history-link"
		/>
	)
}

export function RecentChip(props: { history: ItemHistory | undefined }) {
	const recent = props.history?.recent
	const event = recent === "added" ? props.history?.added : props.history?.changed
	return recent === undefined || event === undefined ? null : (
		<span
			data-tooltip={`${recent === "added" ? "Added" : "Changed"} in ${event.version} on ${formatDate(event.published)}.`}
			class="tag-chip">
			{chipLabels[recent]}
		</span>
	)
}

export function HoverHistory(props: {
	history: ItemHistory | undefined
	kind: OwnerKind
	owner: string
}) {
	return props.history === undefined ||
		(props.history.added === undefined &&
			props.history.changed === undefined) ? null : (
		<span class="member-actions items-center gap-3 text-xs text-fg-faint">
			<Events history={props.history} kind={props.kind} owner={props.owner} />
		</span>
	)
}

export function HistoryLine(props: {
	history: ItemHistory | undefined
	kind: OwnerKind
	owner: string
}) {
	return props.history === undefined ||
		(props.history.added === undefined &&
			props.history.changed === undefined) ? null : (
		<p class="flex flex-wrap gap-x-4 text-xs text-fg-faint">
			<Events history={props.history} kind={props.kind} owner={props.owner} />
		</p>
	)
}
