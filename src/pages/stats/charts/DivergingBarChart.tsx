import { ChartFrame, ChartRow } from "./ChartFrame.tsx"
import { ChartTicks } from "./ChartTicks.tsx"
import type { ChartTick } from "./chart-ticks.ts"
import { axisCap, percent } from "./chart-scale.ts"

export type BarColumn = {
	up: number
	down: number
	hint: string
}

const plotHeight = 100

function Overflow(props: { value: number; index: number; count: number; up: boolean }) {
	return (
		<span
			aria-hidden="true"
			class={[
				"absolute -translate-x-1/2 font-mono text-xs text-fg-muted",
				props.up ? "bottom-0" : "top-0",
			]}
			style={{ left: percent((props.index + 0.5) / props.count) }}>
			{props.value}
		</span>
	)
}

function Bars(props: { columns: BarColumn[]; zero: number; scale: number }) {
	return props.columns.map((column, index) => {
		const up = Math.min(column.up * props.scale, props.zero)
		const down = Math.min(column.down * props.scale, plotHeight - props.zero)
		return (
			<g data-tooltip={column.hint}>
				<rect data-hit="" x={index} y={0} width={1} height={plotHeight} />
				{up > 0 ? (
					<rect
						class="fill-success"
						x={index + 0.2}
						y={props.zero - up}
						width={0.6}
						height={up}
					/>
				) : null}
				{down > 0 ? (
					<rect
						class="fill-danger"
						x={index + 0.2}
						y={props.zero}
						width={0.6}
						height={down}
					/>
				) : null}
			</g>
		)
	})
}

function Legend(props: { upLabel: string; downLabel: string }) {
	return (
		<ul class="flex flex-wrap gap-x-4 text-fg-muted">
			<li class="flex items-center gap-2">
				<span aria-hidden="true" class="size-2 rounded-xs bg-success" />
				{props.upLabel}
			</li>
			<li class="flex items-center gap-2">
				<span aria-hidden="true" class="size-2 rounded-xs bg-danger" />
				{props.downLabel}
			</li>
		</ul>
	)
}

export function DivergingBarChart(props: {
	title: string
	summary: string
	upLabel: string
	downLabel: string
	columns: BarColumn[]
	ticks: ChartTick[]
}) {
	const upCap = axisCap(props.columns.map((column) => column.up))
	const downCap = axisCap(props.columns.map((column) => column.down))
	const scale = plotHeight / (upCap + downCap)
	const zero = upCap * scale
	const count = props.columns.length
	return (
		<ChartFrame
			title={props.title}
			summary={props.summary}
			legend={<Legend upLabel={props.upLabel} downLabel={props.downLabel} />}>
			<div class="flex flex-col">
				<ChartRow>
					{props.columns.map((column, index) =>
						column.up * scale > zero ? (
							<Overflow
								value={column.up}
								index={index}
								count={count}
								up
							/>
						) : null,
					)}
				</ChartRow>
				<div class="flex gap-2">
					<div
						aria-hidden="true"
						class="relative h-40 w-8 shrink-0 text-right text-xs text-fg-faint">
						<span class="absolute top-0 right-0 -translate-y-1/2">
							{upCap}
						</span>
						<span
							class="absolute right-0 -translate-y-1/2"
							style={{ top: percent(zero / plotHeight) }}>
							0
						</span>
						<span class="absolute right-0 bottom-0 translate-y-1/2">
							{downCap}
						</span>
					</div>
					<svg
						role="img"
						aria-label={`${props.title}. ${props.summary}`}
						viewBox={`0 0 ${count} ${plotHeight}`}
						preserveAspectRatio="none"
						class="chart-plot h-40 min-w-0 flex-1">
						<line
							class="stroke-line-strong"
							x1={0}
							x2={count}
							y1={zero}
							y2={zero}
							vector-effect="non-scaling-stroke"
						/>
						<Bars columns={props.columns} zero={zero} scale={scale} />
					</svg>
				</div>
				<ChartRow>
					{props.columns.map((column, index) =>
						column.down * scale > plotHeight - zero ? (
							<Overflow
								value={column.down}
								index={index}
								count={count}
								up={false}
							/>
						) : null,
					)}
				</ChartRow>
				<ChartRow>
					<ChartTicks ticks={props.ticks} count={count} />
				</ChartRow>
			</div>
		</ChartFrame>
	)
}
