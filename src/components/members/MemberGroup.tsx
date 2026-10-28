import { facetLabels } from "../../api/facets/facet-info.ts"
import { classHref, facetHref } from "../../api/routes.ts"
import { propertyGroups } from "../../api/members.ts"
import type { Member } from "../../api/types.ts"
import { SpriteIcon } from "../Icon.tsx"
import { MemberCard } from "./MemberCard.tsx"

function Cards(props: { members: Member[]; owner: string; own: boolean }) {
	return (
		<ul class="flex flex-col gap-4">
			{props.members.map((member) => (
				<MemberCard member={member} owner={props.owner} own={props.own} />
			))}
		</ul>
	)
}

function GroupHeader(props: {
	owner: string
	own: boolean
	count: number
	id: string
}) {
	const label = props.own ? "Defined by" : "Inherited from"
	return (
		<div class="group-head">
			<button
				type="button"
				data-collapse-toggle={props.owner}
				aria-expanded={props.own ? "true" : "false"}
				aria-controls={props.id}
				aria-label={`${label} ${props.owner}`}
				class="absolute inset-0 rounded-sm"
			/>
			<SpriteIcon name="caret-right" class="icon-14 rotate-90 motion-caret" />
			<span>{label}</span>
			{props.own ? (
				<span class="font-mono text-fg">{props.owner}</span>
			) : (
				<a
					href={classHref(props.owner)}
					class="relative font-mono text-code-type hover:underline">
					{props.owner}
				</a>
			)}
			<span data-count="" class="text-fg-faint">
				{props.count}
			</span>
		</div>
	)
}

export function MemberGroup(props: {
	members: Member[]
	owner: string
	own: boolean
	sectionId: string
}) {
	const id = `${props.sectionId}-${props.owner}`
	const categorized = props.members[0]?.kind === "Property"
	return (
		<div
			data-group=""
			data-owner={props.owner}
			data-inherited-group={props.own ? undefined : ""}
			data-collapsed={props.own ? undefined : ""}
			class="flex flex-col gap-2">
			<GroupHeader
				owner={props.owner}
				own={props.own}
				count={props.members.length}
				id={id}
			/>
			<div id={id} data-group-body="" class="flex flex-col gap-4">
				{categorized ? (
					propertyGroups(props.members).map((category) => (
						<div data-subgroup="" class="flex flex-col gap-2">
							<h3 class="flex items-baseline gap-1.5 px-3 text-xs font-medium text-fg-muted">
								<span class="text-fg-faint">
									{facetLabels.propertyCategory}
								</span>
								<a
									href={facetHref(
										"propertyCategory",
										category.category,
									)}
									class="hover:text-fg hover:underline">
									{category.category}
								</a>
							</h3>
							<Cards
								members={category.members}
								owner={props.owner}
								own={props.own}
							/>
						</div>
					))
				) : (
					<Cards
						members={props.members}
						owner={props.owner}
						own={props.own}
					/>
				)}
			</div>
		</div>
	)
}
