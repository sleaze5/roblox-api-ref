import { readEntries, type ZipEntry } from "./central-directory.ts"
import { readEntryData } from "./extract-entry.ts"
import type { RangeReader } from "./range-reader.ts"
import { windowReader } from "./window-reader.ts"

const maxGap = 64 * 1024

type Span = {
	start: number
	end: number
	entries: ZipEntry[]
}

function spansOf(entries: ZipEntry[], selected: Set<ZipEntry>, size: number): Span[] {
	const ordered = entries.toSorted(
		(a, b) => a.localHeaderOffset - b.localHeaderOffset,
	)
	const spans: Span[] = []
	ordered.forEach((entry, index) => {
		if (!selected.has(entry)) {
			return
		}
		const start = entry.localHeaderOffset
		const end = ordered[index + 1]?.localHeaderOffset ?? size
		const last = spans.at(-1)
		if (last !== undefined && start - last.end <= maxGap) {
			last.end = end
			last.entries.push(entry)
		} else {
			spans.push({ start, end, entries: [entry] })
		}
	})
	return spans
}

export async function extractEntries(
	reader: RangeReader,
	select: (name: string) => boolean,
): Promise<Map<string, Uint8Array>> {
	const entries = await readEntries(reader)
	const selected = new Set(entries.filter((entry) => select(entry.name)))
	const spans = spansOf(entries, selected, reader.size)
	const extracted = await Promise.all(
		spans.map(async (span) => {
			const window = await windowReader(reader, span.start, span.end)
			return Promise.all(
				span.entries.map(
					async (entry) =>
						[entry.name, await readEntryData(window, entry)] as const,
				),
			)
		}),
	)
	return new Map(extracted.flat())
}
