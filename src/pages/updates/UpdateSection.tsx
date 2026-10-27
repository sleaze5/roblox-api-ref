import { updateHref } from "../../api/routes.ts"
import { formatDate, updateAnchor } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
import { Icon } from "../../components/Icon.tsx"
import { ownerSections } from "./owner-sections.ts"
import { OwnerSubsectionBlock } from "./OwnerSubsectionBlock.tsx"
import { UpdateFacts } from "./UpdateFacts.tsx"

export function UpdateSection(props: { api: Api; updates: Update[]; update: Update }) {
	const id = updateAnchor(props.update.version)
	return (
		<section
			id={id}
			data-list-section=""
			aria-labelledby={`${id}-title`}
			class="flex flex-col gap-4">
			<div class="flex flex-col gap-2">
				<h2
					id={`${id}-title`}
					class="flex flex-wrap items-center gap-x-2 text-lg font-semibold">
					<Icon name="git-diff" size={20} class="text-fg-muted" />
					<a
						href={updateHref(props.update.version)}
						class="font-mono hover:text-fg-bright hover:underline">
						{props.update.version}
					</a>
					<span class="text-sm font-normal text-fg-faint">
						{formatDate(props.update.published)}
					</span>
				</h2>
				<UpdateFacts update={props.update} updates={props.updates} />
			</div>
			{props.update.owners.length === 0 ? (
				<p class="text-fg-muted">
					The API did not change between these versions.
				</p>
			) : (
				<div class="flex flex-col gap-8">
					{ownerSections(props.update).flatMap((section) =>
						section.groups.map((group) => (
							<OwnerSubsectionBlock
								api={props.api}
								group={group}
								title={group.railLabel}
								idPrefix={`${props.update.version}-`}
							/>
						)),
					)}
				</div>
			)}
		</section>
	)
}
