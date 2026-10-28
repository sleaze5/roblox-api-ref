export type ChartTick = {
	index: number
	label: string
}

const minimumGap = 0.06

export function yearTicks(dates: string[]): ChartTick[] {
	const ticks: ChartTick[] = []
	dates.forEach((date, index) => {
		const year = date.slice(0, 4)
		const last = ticks.at(-1)
		if (last?.label === year) {
			return
		}
		if (last !== undefined && (index - last.index) / dates.length < minimumGap) {
			ticks.pop()
		}
		ticks.push({ index, label: year })
	})
	return ticks
}
