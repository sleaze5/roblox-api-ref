import type {
	HistoryChanges,
	HistoryEntry,
	MemberCounts,
} from "../../api/item-history.ts"

export type HistoryNoun = "member" | "item"

function plural(count: number, noun: string): string {
	return `${count} ${noun}${count === 1 ? "" : "s"}`
}

function memberParts(
	members: MemberCounts,
	deprecated: number,
	noun: HistoryNoun,
): string[] {
	const counts = { ...members, deprecated }
	return (["added", "removed", "changed", "deprecated"] as const)
		.filter((kind) => counts[kind] > 0)
		.map((kind, index) =>
			index === 0
				? `${plural(counts[kind], noun)} ${kind}`
				: `${counts[kind]} ${kind}`,
		)
}

function changeParts(changes: HistoryChanges): string[] {
	return [
		...(changes.deprecation === "set" ? ["deprecated"] : []),
		...(changes.deprecation === "clear" ? ["no longer deprecated"] : []),
		...changes.fields.map((field) => `${field} updated`),
		...(changes.tagsAdded > 0 ? [`${plural(changes.tagsAdded, "tag")} added`] : []),
		...(changes.tagsRemoved > 0
			? [`${plural(changes.tagsRemoved, "tag")} removed`]
			: []),
	]
}

function statusParts(entry: HistoryEntry): string[] {
	return [
		...(entry.kinds.includes("added") ? ["added"] : []),
		...(entry.kinds.includes("removed") ? ["removed"] : []),
		...(entry.kinds.includes("changed") && entry.changes === undefined
			? ["changed"]
			: []),
	]
}

export function entryText(entry: HistoryEntry, noun: HistoryNoun = "member"): string {
	const text = [
		...statusParts(entry),
		...(entry.members === undefined
			? []
			: memberParts(entry.members, entry.changes?.deprecated ?? 0, noun)),
		...(entry.changes === undefined ? [] : changeParts(entry.changes)),
		...(entry.kinds.includes("described") ? ["description updated"] : []),
	].join(", ")
	return text.charAt(0).toUpperCase() + text.slice(1)
}
