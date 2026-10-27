import type { HistoryEntry, HistoryKind, MemberCounts } from "../../api/item-history.ts"

const kindLabels: Record<HistoryKind, string> = {
	added: "Added",
	removed: "Removed",
	changed: "Changed",
	described: "Description changed",
}

function memberText(members: MemberCounts): string {
	const parts = (["added", "removed", "changed"] as const)
		.filter((kind) => members[kind] > 0)
		.map((kind) => `${members[kind]} ${kind}`)
	const total = members.added + members.removed + members.changed
	return `${total === 1 ? "Member" : "Members"}: ${parts.join(", ")}`
}

export function entryText(entry: HistoryEntry): string {
	return [
		...entry.kinds.map((kind) => kindLabels[kind]),
		...(entry.members === undefined ? [] : [memberText(entry.members)]),
	].join(", ")
}
