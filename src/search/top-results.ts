export class TopResults<T> {
	private readonly items: T[] = []
	private readonly limit: number
	private readonly compare: (a: T, b: T) => number

	constructor(limit: number, compare: (a: T, b: T) => number) {
		this.limit = limit
		this.compare = compare
	}

	private accepts(item: T): boolean {
		const last = this.items.at(-1)
		return (
			this.limit > 0 &&
			(this.items.length < this.limit ||
				(last !== undefined && this.compare(item, last) < 0))
		)
	}

	add(item: T): void {
		if (!this.accepts(item)) {
			return
		}
		let low = 0
		let high = this.items.length
		while (low < high) {
			const middle = (low + high) >>> 1
			if (this.compare(item, this.items[middle] as T) < 0) {
				high = middle
			} else {
				low = middle + 1
			}
		}
		this.items.splice(low, 0, item)
		if (this.items.length > this.limit) {
			this.items.pop()
		}
	}

	toArray(): T[] {
		return this.items
	}
}
