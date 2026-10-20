function isUpper(code: number): boolean {
	return code >= 65 && code <= 90
}

function isLower(code: number): boolean {
	return code >= 97 && code <= 122
}

function isDigit(code: number): boolean {
	return code >= 48 && code <= 57
}

export function isBoundary(name: string, index: number): boolean {
	if (index === 0) {
		return true
	}
	const previous = name.charCodeAt(index - 1)
	const current = name.charCodeAt(index)
	if (previous === 95) {
		return true
	}
	if (isUpper(current) && !isUpper(previous)) {
		return true
	}
	if (isDigit(current) && !isDigit(previous)) {
		return true
	}
	return isUpper(current) && isUpper(previous) && isLower(name.charCodeAt(index + 1))
}

function subsequence(
	query: string,
	name: string,
	lower: string,
	preferBoundaries: boolean,
): number[] | undefined {
	const positions: number[] = []
	let from = 0
	for (const char of query) {
		let found = -1
		if (preferBoundaries) {
			for (
				let index = lower.indexOf(char, from);
				index !== -1;
				index = lower.indexOf(char, index + 1)
			) {
				if (isBoundary(name, index)) {
					found = index
					break
				}
			}
		}
		if (found === -1) {
			found = lower.indexOf(char, from)
		}
		if (found === -1) {
			return undefined
		}
		positions.push(found)
		from = found + 1
	}
	return positions
}

export function fuzzyPositions(
	query: string,
	name: string,
	lower: string,
): number[] | undefined {
	return (
		subsequence(query, name, lower, true) ?? subsequence(query, name, lower, false)
	)
}

const minimumFuzzySpan = 16
const fuzzySpanPerCharacter = 2

function isTooSpread(query: string, positions: number[]): boolean {
	const span = (positions.at(-1) ?? 0) - (positions[0] ?? 0) + 1
	return span > Math.max(minimumFuzzySpan, query.length * fuzzySpanPerCharacter)
}

function fuzzyScore(query: string, name: string, lower: string): number {
	const positions = fuzzyPositions(query, name, lower)
	if (positions === undefined || isTooSpread(query, positions)) {
		return 0
	}
	let score = 300
	let previous = -1
	for (const position of positions) {
		if (isBoundary(name, position)) {
			score += 12
		}
		if (previous !== -1 && position !== previous + 1) {
			score -= Math.min(position - previous, 10) * 3
		}
		previous = position
	}
	score -= Math.min(positions[0] ?? 0, 20) * 2
	score -= Math.min(lower.length - query.length, 40)
	return Math.max(score, 1)
}

export function scoreName(query: string, name: string, lower: string): number {
	if (query.length === 0) {
		return 1
	}
	if (lower === query) {
		return 1000
	}
	if (lower.startsWith(query)) {
		return 900 - Math.min(lower.length - query.length, 100)
	}
	const index = lower.indexOf(query)
	if (index > 0) {
		const base = isBoundary(name, index) ? 700 : 500
		return base - Math.min(index, 50) - Math.min(lower.length - query.length, 50)
	}
	return fuzzyScore(query, name, lower)
}

export function matchPositions(query: string, name: string): number[] {
	const lower = name.toLowerCase()
	if (query.length === 0) {
		return []
	}
	const index = lower.indexOf(query)
	if (index !== -1) {
		return Array.from({ length: query.length }, (_, offset) => index + offset)
	}
	return fuzzyPositions(query, name, lower) ?? []
}
