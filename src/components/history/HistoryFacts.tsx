import type { HistoryEvent, ItemHistory } from "../../api/item-history.ts"
import { formatDate } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { Fact } from "../facts/Fact.tsx"
import { historyHref } from "./history-href.ts"

const nouns: Record<OwnerKind, string> = { Class: "class", Enum: "enum" }

function EventValue(props: { event: HistoryEvent; kind: OwnerKind; name: string }) {
	return (
		<span class="flex flex-wrap items-baseline gap-x-2">
			<a
				href={historyHref(props.event, props.kind, props.name)}
				class="font-mono text-fg underline decoration-line-strong hover:decoration-fg">
				{props.event.version}
			</a>
			<span class="text-fg-faint">{formatDate(props.event.published)}</span>
		</span>
	)
}

export function HistoryFacts(props: {
	history: ItemHistory | undefined
	kind: OwnerKind
	name: string
	start: string | undefined
}) {
	const noun = nouns[props.kind]
	const added = props.history?.added
	const changed = props.history?.changed
	return (
		<>
			<Fact
				label="Added"
				hint={
					added === undefined
						? `Not recorded: this ${noun} already existed in ${props.start ?? "the oldest version"}, the oldest version that this site tracks.`
						: `The update that added this ${noun}.`
				}>
				{added === undefined ? (
					"Not recorded"
				) : (
					<EventValue event={added} kind={props.kind} name={props.name} />
				)}
			</Fact>
			{changed === undefined ? null : (
				<Fact
					label="Last changed"
					hint={`The last update that changed this ${noun} or its members, not counting descriptions.`}>
					<EventValue event={changed} kind={props.kind} name={props.name} />
				</Fact>
			)}
		</>
	)
}
