import { memberKindTitles, membersOfKind } from "../../api/members.ts"
import type { Class, MemberKind } from "../../api/types.ts"
import { Icon } from "../Icon.tsx"
import { kindIcons } from "./kind-icons.ts"
import { MemberGroup } from "./MemberGroup.tsx"

export function sectionId(kind: MemberKind): string {
	return memberKindTitles[kind].toLowerCase()
}

export function MemberSection(props: {
	kind: MemberKind
	item: Class
	ancestors: Class[]
}) {
	const groups = [...props.ancestors]
		.reverse()
		.map((owner) => ({
			owner: owner.name,
			own: false,
			members: membersOfKind(owner.members, props.kind),
		}))
		.concat({
			owner: props.item.name,
			own: true,
			members: membersOfKind(props.item.members, props.kind),
		})
		.filter((group) => group.members.length > 0)
	if (groups.length === 0) {
		return null
	}
	const id = sectionId(props.kind)
	const title = memberKindTitles[props.kind]
	const total = groups.reduce((sum, group) => sum + group.members.length, 0)
	return (
		<section
			id={id}
			data-section={props.kind}
			aria-labelledby={`${id}-title`}
			class="flex flex-col gap-3">
			<h2
				id={`${id}-title`}
				class="flex items-center gap-2 text-lg font-semibold">
				<Icon name={kindIcons[props.kind]} size={20} class="text-fg-muted" />
				{title}
				<span data-count="" class="text-sm font-normal text-fg-faint">
					{total}
				</span>
			</h2>
			{groups.map((group) => (
				<MemberGroup
					members={group.members}
					owner={group.owner}
					own={group.own}
					sectionId={id}
				/>
			))}
			<p data-empty="" hidden class="text-fg-muted">
				No {title.toLowerCase()} match the filters.
			</p>
		</section>
	)
}
