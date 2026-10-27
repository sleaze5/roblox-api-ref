import type { ChangeEntry } from "./update-types.ts"

export function isApiChange(entry: ChangeEntry): boolean {
	return (
		entry.status !== "Changed" ||
		entry.details.length > 0 ||
		JSON.stringify(entry.before) !== JSON.stringify(entry.after)
	)
}
