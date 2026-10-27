import { signatureText } from "../../api/change-kind.ts"
import type { HistoryEvent, ItemHistory, Removed } from "../../api/item-history.ts"
import { updateHref } from "../../api/routes.ts"
import type { EnumItem, Member } from "../../api/types.ts"
import { changeAnchor, formatDate } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { absoluteUrl } from "../../site/origin.ts"
import { inlineCode, link } from "./escape.ts"
import { entryText } from "../../components/history/history-text.ts"

type Fact = [string, string | undefined]

function eventText(event: HistoryEvent | undefined): string | undefined {
	return event === undefined
		? undefined
		: `${event.version} (${formatDate(event.published)})`
}

export function historyFacts(history: ItemHistory | undefined, full: boolean): Fact[] {
	if (history === undefined) {
		return []
	}
	return [
		[
			"Added",
			eventText(history.added) ??
				`Not recorded (${history.addedBefore ?? "oldest tracked version"} or earlier)`,
		],
		...(full ? ([["Last changed", eventText(history.changed)]] as Fact[]) : []),
		["Deprecated", eventText(history.deprecated)],
		[
			"History",
			history.entries
				.map((entry) => `${entry.version}: ${entryText(entry)}`)
				.join("; "),
		],
	]
}

export function removedMarkdown(
	title: string,
	kind: OwnerKind,
	owner: string,
	removed: Removed<Member | EnumItem>[] | undefined,
): string | undefined {
	if (removed === undefined || removed.length === 0) {
		return undefined
	}
	const lines = removed.map(({ value, removed: event }) => {
		const url = absoluteUrl(
			`${updateHref(event.version)}#${changeAnchor(kind, owner, value.name)}`,
		)
		return `- ${inlineCode(signatureText(owner, value))}: removed in ${link(event.version, url)} (${formatDate(event.published)})`
	})
	return `## ${title}\n\n${lines.join("\n")}`
}
