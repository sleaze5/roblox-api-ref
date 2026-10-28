import type { JSX } from "@solidjs/web"

export function StatsSection(props: {
	id: string
	title: string
	lead?: string | undefined
	children: JSX.Element
}) {
	return (
		<section
			id={props.id}
			aria-labelledby={`${props.id}-title`}
			class="flex flex-col gap-3">
			<h2 id={`${props.id}-title`} class="text-lg font-semibold">
				{props.title}
			</h2>
			{props.lead === undefined ? null : (
				<p class="max-w-prose text-fg-muted">{props.lead}</p>
			)}
			{props.children}
		</section>
	)
}
