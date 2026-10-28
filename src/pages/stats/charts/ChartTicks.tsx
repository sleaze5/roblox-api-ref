import type { ChartTick } from "./chart-ticks.ts"
import { percent } from "./chart-scale.ts"

export function ChartTicks(props: { ticks: ChartTick[]; count: number }) {
	return (
		<div aria-hidden="true" class="text-xs text-fg-faint">
			{props.ticks.map((tick) => (
				<span
					class="absolute top-0 border-l border-line pl-1"
					style={{ left: percent(tick.index / props.count) }}>
					{tick.label}
				</span>
			))}
		</div>
	)
}
