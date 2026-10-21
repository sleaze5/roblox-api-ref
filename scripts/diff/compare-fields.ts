import type { FieldChange } from "./diff-types.ts"

const unorderedFields = new Set(["Tags", "LegacyNames"])

function canonical(value: unknown): unknown {
	if (Array.isArray(value)) {
		return value.map(canonical)
	}
	if (typeof value === "object" && value !== null) {
		return Object.fromEntries(
			Object.entries(value)
				.sort(([a], [b]) => a.localeCompare(b))
				.map(([key, entry]) => [key, canonical(entry)]),
		)
	}
	return value
}

function fieldKey(field: string, value: unknown): string {
	const normalized = canonical(value)
	if (unorderedFields.has(field) && Array.isArray(normalized)) {
		return JSON.stringify(normalized.map((entry) => JSON.stringify(entry)).sort())
	}
	return JSON.stringify(normalized ?? null)
}

export function compareFields(
	before: Record<string, unknown>,
	after: Record<string, unknown>,
	ignored: readonly string[],
): FieldChange[] {
	const fields = new Set([...Object.keys(before), ...Object.keys(after)])
	const changes: FieldChange[] = []
	for (const field of fields) {
		if (ignored.includes(field)) {
			continue
		}
		const previous = before[field] ?? null
		const next = after[field] ?? null
		if (fieldKey(field, previous) !== fieldKey(field, next)) {
			changes.push({ field, before: previous, after: next })
		}
	}
	return changes
}
