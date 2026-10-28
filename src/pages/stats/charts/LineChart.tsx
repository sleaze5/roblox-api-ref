import { ChartFrame, ChartRow } from "./ChartFrame.tsx"
import { ChartTicks } from "./ChartTicks.tsx"
import type { ChartTick } from "./chart-ticks.ts"
import { niceCeil } from "./chart-scale.ts"

export type LinePoint = {
	value: number
	hint: string
}

const plotHeight = 100

export function LineChart(props: {
	title: string
	summary: string
	points: LinePoint[]
	ticks: ChartTick[]
}) {
	const values = props.points.map((point) => point.value)
	const max = Math.max(...values)
	const min = Math.min(...values)
	const step = niceCeil((max - min) / 4)
	const top = Math.ceil(max / step) * step
	const bottom = Math.floor(min / step) * step
	const y = (value: number) =>
		plotHeight - ((value - bottom) / (top - bottom || 1)) * plotHeight
	const count = props.points.length
	const path = props.points
		.map((point, index) => `${index + 0.5},${y(point.value)}`)
		.join(" ")
	return (
		<ChartFrame title={props.title} summary={props.summary}>
			<div class="flex flex-col">
				<div class="flex gap-2">
					<div
						aria-hidden="true"
						class="relative h-32 w-8 shrink-0 text-right text-xs text-fg-faint">
						<span class="absolute top-0 right-0 -translate-y-1/2">
							{top}
						</span>
						<span class="absolute right-0 bottom-0 translate-y-1/2">
							{bottom}
						</span>
					</div>
					<svg
						role="img"
						aria-label={`${props.title}. ${props.summary}`}
						viewBox={`0 0 ${count} ${plotHeight}`}
						preserveAspectRatio="none"
						class="chart-plot h-32 min-w-0 flex-1">
						<line
							class="stroke-line-subtle"
							x1={0}
							x2={count}
							y1={0}
							y2={0}
							vector-effect="non-scaling-stroke"
						/>
						<line
							class="stroke-line"
							x1={0}
							x2={count}
							y1={plotHeight}
							y2={plotHeight}
							vector-effect="non-scaling-stroke"
						/>
						{props.points.map((point, index) => (
							<rect
								data-hit=""
								data-tooltip={point.hint}
								x={index}
								y={0}
								width={1}
								height={plotHeight}
							/>
						))}
						<polyline
							class="pointer-events-none fill-none stroke-fg-muted stroke-2"
							points={path}
							vector-effect="non-scaling-stroke"
							stroke-linejoin="round"
						/>
					</svg>
				</div>
				<div class="h-2" />
				<ChartRow>
					<ChartTicks ticks={props.ticks} count={count} />
				</ChartRow>
			</div>
		</ChartFrame>
	)
}
