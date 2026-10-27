import type { Api } from "../../api/types.ts"
import { OwnerGroup } from "./OwnerGroup.tsx"
import type { OwnerSubsection } from "./owner-sections.ts"

export function OwnerSubsectionBlock(props: { api: Api; group: OwnerSubsection }) {
	return (
		<section
			id={props.group.id}
			data-list-section=""
			aria-labelledby={`${props.group.id}-title`}
			class="flex flex-col gap-4">
			<h3
				id={`${props.group.id}-title`}
				class="flex items-center gap-2 border-b border-line-subtle pb-2 text-base font-semibold">
				{props.group.title}
				<span class="text-sm font-normal text-fg-faint">
					{props.group.owners.length}
				</span>
			</h3>
			<div class="flex flex-col gap-6">
				{props.group.owners.map((owner) => (
					<OwnerGroup api={props.api} owner={owner} heading="h4" />
				))}
			</div>
		</section>
	)
}
