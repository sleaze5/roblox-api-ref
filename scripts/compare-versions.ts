export function compareVersions(a: string, b: string): number {
	const left = a.split(".").map(Number)
	const right = b.split(".").map(Number)
	for (let index = 0; index < Math.max(left.length, right.length); index++) {
		const difference = (left[index] ?? 0) - (right[index] ?? 0)
		if (difference !== 0) {
			return difference
		}
	}
	return 0
}
