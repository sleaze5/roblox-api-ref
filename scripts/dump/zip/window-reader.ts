import type { RangeReader } from "./range-reader.ts"

export async function windowReader(
	reader: RangeReader,
	start: number,
	end: number,
): Promise<RangeReader> {
	const bytes = await reader.read(start, end)
	return {
		size: reader.size,
		read: async (from, to) =>
			from >= start && to <= end
				? bytes.subarray(from - start, to - start)
				: reader.read(from, to),
	}
}
