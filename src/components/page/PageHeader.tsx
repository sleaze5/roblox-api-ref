import type { JSX } from "@solidjs/web"
import { CreatorHubLink } from "../CreatorHubLink.tsx"
import type { IconName } from "../icon-names.ts"
import { Icon } from "../Icon.tsx"
import { Breadcrumbs, type Crumb } from "./Breadcrumbs.tsx"

export function PageHeader(props: {
	crumbs: Crumb[]
	icon: IconName
	title: JSX.Element
	code?: boolean
	docs?: { href: string; name: string } | undefined
	actions?: JSX.Element
	children?: JSX.Element
}) {
	return (
		<header class="flex flex-col gap-4">
			<div class="flex flex-col gap-2">
				<Breadcrumbs crumbs={props.crumbs} />
				<div class="flex items-center gap-3">
					<Icon name={props.icon} size={28} class="text-fg-muted" />
					<h1
						class={[
							"min-w-0 flex-1 text-2xl font-semibold text-fg-bright",
							props.code ? "font-mono break-all" : "break-words",
						]}>
						{props.title}
					</h1>
					{props.actions}
					{props.docs === undefined ? null : (
						<CreatorHubLink href={props.docs.href} name={props.docs.name} />
					)}
				</div>
			</div>
			{props.children}
		</header>
	)
}
