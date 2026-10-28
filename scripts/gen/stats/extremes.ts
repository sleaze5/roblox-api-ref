import type { StatRecord, StatRef } from "../../../src/api/stats-types.ts"

export type Measured = {
	ref: StatRef
	value: number
}

export type RecordInfo = Pick<StatRecord, "id" | "label" | "hint">

export function highest(info: RecordInfo, measured: Measured[]): StatRecord {
	return extreme(info, measured, (value, best) => value > best)
}

export function lowest(info: RecordInfo, measured: Measured[]): StatRecord {
	return extreme(info, measured, (value, best) => value < best)
}

function extreme(
	info: RecordInfo,
	measured: Measured[],
	beats: (value: number, best: number) => boolean,
): StatRecord {
	let value: number | undefined
	let items: StatRef[] = []
	for (const entry of measured) {
		if (value === undefined || beats(entry.value, value)) {
			value = entry.value
			items = [entry.ref]
		} else if (entry.value === value) {
			items.push(entry.ref)
		}
	}
	return { ...info, value: value ?? 0, items }
}
