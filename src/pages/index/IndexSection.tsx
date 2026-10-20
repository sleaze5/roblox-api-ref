import type { JSX } from "@solidjs/web"
import type { IconName } from "../../components/icon-names.ts"
import { Icon } from "../../components/Icon.tsx"

export const indexGridClass = "grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3"

export function IndexSection(props: {
	id: string
	title: string
	icon: IconName
	count: number
	href?: string
	children: JSX.Element
}) {
	return (
		<section
			id={props.id}
			data-list-section=""
			aria-labelledby={`${props.id}-title`}
			class="flex flex-col gap-3">
			<h2
				id={`${props.id}-title`}
				class="flex items-center gap-2 text-lg font-semibold">
				<Icon name={props.icon} size={20} class="text-fg-muted" />
				{props.href === undefined ? (
					props.title
				) : (
					<a href={props.href} class="hover:text-fg-bright hover:underline">
						{props.title}
					</a>
				)}
				<span class="text-sm font-normal text-fg-faint">{props.count}</span>
			</h2>
			{props.children}
		</section>
	)
}
