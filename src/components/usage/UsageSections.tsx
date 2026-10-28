import { compareNames } from "../../api/compare-names.ts"
import { memberHref } from "../../api/routes.ts"
import type { TypeRole, TypeUsage } from "../../api/types.ts"
import type { IconName } from "../icon-names.ts"
import { Icon } from "../Icon.tsx"
import { kindIcons } from "../members/kind-icons.ts"

export const roleSections: {
	role: TypeRole
	id: string
	title: string
	icon: IconName
}[] = [
	{
		role: "Value",
		id: "used-by-properties",
		title: "Used by properties",
		icon: "sliders-horizontal",
	},
	{
		role: "Parameter",
		id: "used-by-parameters",
		title: "Used by parameters",
		icon: "sign-in",
	},
	{
		role: "Return",
		id: "used-by-returns",
		title: "Used by return values",
		icon: "sign-out",
	},
]

function byMember(a: TypeUsage, b: TypeUsage): number {
	return (
		compareNames(a.className, b.className) ||
		compareNames(a.memberName, b.memberName)
	)
}

function UsageLink(props: { usage: TypeUsage }) {
	const separator = props.usage.kind === "Method" ? ":" : "."
	return (
		<li data-name={`${props.usage.className}${separator}${props.usage.memberName}`}>
			<a
				href={memberHref(
					props.usage.className,
					props.usage.kind,
					props.usage.memberName,
				)}
				class="flex h-7 items-center gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover">
				<Icon
					name={kindIcons[props.usage.kind]}
					size={14}
					class="text-fg-muted"
				/>
				<span class="truncate">
					<span class="text-code-type">{props.usage.className}</span>
					<span>{separator}</span>
					<span class="text-fg-bright">{props.usage.memberName}</span>
				</span>
			</a>
		</li>
	)
}

export function usageCount(usages: TypeUsage[], role: TypeRole): number {
	return usages.filter((usage) => usage.role === role).length
}

export function UsageSections(props: { usages: TypeUsage[]; name: string }) {
	const sections = roleSections
		.map((section) => ({
			id: section.id,
			title: section.title,
			icon: section.icon,
			usages: props.usages
				.filter((usage) => usage.role === section.role)
				.sort(byMember),
		}))
		.filter((section) => section.usages.length > 0)
	if (sections.length === 0) {
		return <p class="text-fg-muted">No class members use {props.name}.</p>
	}
	return (
		<>
			{sections.map((section) => (
				<section
					id={section.id}
					data-list-section=""
					aria-labelledby={`${section.id}-title`}
					class="flex flex-col gap-3">
					<h2
						id={`${section.id}-title`}
						class="flex items-center gap-2 text-lg font-semibold">
						<Icon name={section.icon} size={20} class="text-fg-muted" />
						{section.title}
						<span class="text-sm font-normal text-fg-faint">
							{section.usages.length}
						</span>
					</h2>
					<ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2">
						{section.usages.map((usage) => (
							<UsageLink usage={usage} />
						))}
					</ul>
				</section>
			))}
		</>
	)
}
