import { ChartFrame, ChartRow } from "./ChartFrame.tsx"
import { axisCap, percent } from "./chart-scale.ts"

export type Column = {
	label: string
	value: number
	hint: string
}

const plotHeight = 100

function markedValues(columns: Column[]): Set<number> {
	const values = columns.map((column) => column.value)
	return new Set([Math.max(...values), Math.min(...values)])
}

function ValueLabels(props: { columns: Column[]; scale: number }) {
	const marked = markedValues(props.columns)
	return props.columns.map((column, index) =>
		marked.has(column.value) ? (
			<span
				aria-hidden="true"
				class="absolute -translate-x-1/2 -translate-y-full pb-1 text-xs text-fg-muted"
				style={{
					left: percent((index + 0.5) / props.columns.length),
					top: percent(
						1 -
							Math.min(column.value * props.scale, plotHeight) /
								plotHeight,
					),
				}}>
				{column.value}
			</span>
		) : null,
	)
}

export function ColumnChart(props: {
	title: string
	summary: string
	columns: Column[]
}) {
	const cap = axisCap(props.columns.map((column) => column.value))
	const scale = plotHeight / cap
	const count = props.columns.length
	return (
		<ChartFrame title={props.title} summary={props.summary}>
			<div class="flex flex-col">
				<div class="h-4" />
				<div class="flex gap-2">
					<div
						aria-hidden="true"
						class="relative h-32 w-8 shrink-0 text-right text-xs text-fg-faint">
						<span class="absolute top-0 right-0 -translate-y-1/2">
							{cap}
						</span>
						<span class="absolute right-0 bottom-0 translate-y-1/2">0</span>
					</div>
					<div class="relative h-32 min-w-0 flex-1">
						<svg
							role="img"
							aria-label={`${props.title}. ${props.summary}`}
							viewBox={`0 0 ${count} ${plotHeight}`}
							preserveAspectRatio="none"
							class="chart-plot h-32">
							<line
								class="stroke-line-strong"
								x1={0}
								x2={count}
								y1={plotHeight}
								y2={plotHeight}
								vector-effect="non-scaling-stroke"
							/>
							{props.columns.map((column, index) => {
								const height = Math.min(
									column.value * scale,
									plotHeight,
								)
								return (
									<g data-tooltip={column.hint}>
										<rect
											data-hit=""
											x={index}
											y={0}
											width={1}
											height={plotHeight}
										/>
										<rect
											class="fill-fg-muted"
											x={index + 0.25}
											y={plotHeight - height}
											width={0.5}
											height={height}
										/>
									</g>
								)
							})}
						</svg>
						<ValueLabels columns={props.columns} scale={scale} />
					</div>
				</div>
				<div class="h-1" />
				<ChartRow>
					<div aria-hidden="true" class="flex text-xs text-fg-faint">
						{props.columns.map((column) => (
							<span class="min-w-0 flex-1 text-center">
								{column.label}
							</span>
						))}
					</div>
				</ChartRow>
			</div>
		</ChartFrame>
	)
}
