const steps = [1, 2, 2.5, 5, 10]

export function niceCeil(value: number): number {
	if (value <= 0) {
		return 1
	}
	const magnitude = 10 ** Math.floor(Math.log10(value))
	const step = steps.find((candidate) => candidate * magnitude >= value) ?? 10
	return step * magnitude
}

export function axisCap(values: number[]): number {
	const [first = 0, second = 0] = values.toSorted((a, b) => b - a)
	return niceCeil(first > second * 2 && second > 0 ? second : first)
}

export function percent(value: number): string {
	return `${Math.round(value * 1000) / 10}%`
}
