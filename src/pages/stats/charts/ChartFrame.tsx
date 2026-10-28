import type { JSX } from "@solidjs/web"

export function ChartFrame(props: {
	title: string
	summary: string
	legend?: JSX.Element
	children: JSX.Element
}) {
	return (
		<figure class="flex min-w-0 flex-col gap-3">
			<figcaption class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
				<span class="font-medium text-fg">{props.title}</span>
				<span class="text-fg-muted">{props.summary}</span>
			</figcaption>
			{props.legend}
			{props.children}
		</figure>
	)
}

export function ChartRow(props: { children: JSX.Element }) {
	return (
		<div class="flex gap-2">
			<div class="w-8 shrink-0" />
			<div class="relative h-4 min-w-0 flex-1">{props.children}</div>
		</div>
	)
}
