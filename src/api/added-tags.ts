import { memberTags, plainTags, type DisplayTag } from "./tags.ts"
import type { ChangeEntry } from "./update-types.ts"

export function addedTags(entry: ChangeEntry): DisplayTag[] {
	if (entry.status !== "Added" || entry.after === undefined) {
		return []
	}
	return entry.kind === "Member"
		? memberTags(entry.after)
		: plainTags(entry.after.tags)
}
