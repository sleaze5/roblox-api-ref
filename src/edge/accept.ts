export type MediaType = "text/html" | "text/markdown"

type MediaRange = {
	type: string
	q: number
	specificity: number
	position: number
}

function quality(parameters: string[]): number {
	const value = parameters
		.map((parameter) => parameter.split("=").map((part) => part.trim()))
		.find(([name]) => name?.toLowerCase() === "q")?.[1]
	const q = Number(value ?? 1)
	return Number.isNaN(q) ? 1 : Math.min(Math.max(q, 0), 1)
}

function specificity(type: string): number {
	if (type === "*/*") {
		return 0
	}
	return type.endsWith("/*") ? 1 : 2
}

function parseAccept(header: string): MediaRange[] {
	return header.split(",").flatMap((entry, position) => {
		const [rawType = "", ...parameters] = entry.split(";")
		const type = rawType.trim().toLowerCase()
		return type === ""
			? []
			: [
					{
						type,
						q: quality(parameters),
						specificity: specificity(type),
						position,
					},
				]
	})
}

function covers(range: MediaRange, type: MediaType): boolean {
	return (
		range.type === "*/*" ||
		range.type === type ||
		(range.type.endsWith("/*") && type.startsWith(range.type.slice(0, -1)))
	)
}

function bestRange(ranges: MediaRange[], type: MediaType): MediaRange | undefined {
	return ranges
		.filter((range) => covers(range, type))
		.sort((a, b) => b.specificity - a.specificity || a.position - b.position)[0]
}

function unrejectedType(
	ranges: MediaRange[],
	types: readonly MediaType[],
): MediaType | undefined {
	return types.find((type) => bestRange(ranges, type) === undefined)
}

export function preferredType(
	header: string | null,
	types: readonly MediaType[],
): MediaType | undefined {
	const ranges = parseAccept(header ?? "")
	if (ranges.length === 0) {
		return types[0]
	}
	if (ranges.every((range) => range.q === 0)) {
		return unrejectedType(ranges, types)
	}
	const candidates = types.flatMap((type) => {
		const range = bestRange(ranges, type)
		return range === undefined || range.q === 0 ? [] : [{ type, range }]
	})
	return candidates.sort(
		(a, b) => b.range.q - a.range.q || a.range.position - b.range.position,
	)[0]?.type
}
