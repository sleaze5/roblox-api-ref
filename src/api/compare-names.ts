const collator = new Intl.Collator("en", { numeric: true })

export function compareNames(a: string, b: string): number {
	return collator.compare(a, b)
}
