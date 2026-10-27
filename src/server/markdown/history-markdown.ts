import type { HistoryEvent, ItemHistory } from "../../api/item-history.ts"
import { formatDate } from "../../api/update-text.ts"
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
