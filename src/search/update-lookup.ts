import type { SearchUpdate } from "./index-format.ts"

const hashPattern = /^[0-9a-f]{6,16}$/
const versionPattern = /^\d+\.\d+(\.\d*)*$/

export function matchUpdates(
	updates: SearchUpdate[],
	query: string,
	limit: number,
): SearchUpdate[] {
	const text = query
		.trim()
		.toLowerCase()
		.replace(/^version-/, "")
	if (hashPattern.test(text)) {
		return updates.filter((update) => update.hash.startsWith(text)).slice(0, limit)
	}
	if (versionPattern.test(text)) {
		return updates
			.filter((update) => update.version.startsWith(text))
			.slice(0, limit)
	}
	return []
}
