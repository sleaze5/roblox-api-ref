import { isApiChange } from "../../api/api-change.ts"
import { updateHref } from "../../api/routes.ts"
import { formatDate, ownerAnchor, updateAnchor } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import type { ChangeStatus, OwnerChanges, Update } from "../../api/update-types.ts"
import { ChangeCounts, type ChangeCountValues } from "../../components/ChangeCounts.tsx"
import { classIconStyle } from "../../components/ClassIcon.tsx"
import { Icon } from "../../components/Icon.tsx"
import { ownerSections, type OwnerSubsection } from "./owner-sections.ts"
import { UpdateFacts } from "./UpdateFacts.tsx"

const countKeys: Record<ChangeStatus, keyof ChangeCountValues> = {
	Added: "added",
	Removed: "removed",
	Changed: "changed",
}

function entryCounts(owner: OwnerChanges): ChangeCountValues {
	const counts = { added: 0, removed: 0, changed: 0 }
	for (const entry of owner.entries) {
		if (isApiChange(entry)) {
			counts[countKeys[entry.status]]++
		}
	}
	return counts
}

function OwnerNote(props: { owner: OwnerChanges; counts: ChangeCountValues }) {
	if (props.counts.added + props.counts.removed + props.counts.changed > 0) {
		return <ChangeCounts counts={props.counts} />
	}
	if (props.owner.details.length > 0) {
		return <span>Details changed</span>
	}
	return props.owner.status === undefined ? <span>Description changed</span> : null
}

function OwnerRow(props: { api: Api; update: Update; owner: OwnerChanges }) {
	const icon =
		props.owner.kind === "Class"
			? props.api.classes[props.owner.name]?.icon
			: undefined
	return (
		<li data-name={props.owner.name}>
			<a
				href={`${updateHref(props.update.version)}#${ownerAnchor(props.owner)}`}
				class="flex h-7 items-center justify-between gap-3 rounded-sm px-2 motion-colors hover:bg-hover">
				<span class="flex min-w-0 items-center gap-2">
					<span
						aria-hidden="true"
						data-kind={props.owner.kind}
						style={classIconStyle(icon)}
						class="kind-icon text-fg-muted"
					/>
					<code class="truncate text-code-type">{props.owner.name}</code>
				</span>
				<span class="shrink-0 text-xs text-fg-faint">
					<OwnerNote owner={props.owner} counts={entryCounts(props.owner)} />
				</span>
			</a>
		</li>
	)
}

function GroupSummary(props: { api: Api; update: Update; group: OwnerSubsection }) {
	const id = `${props.update.version}-${props.group.id}`
	return (
		<section
			id={id}
			data-list-section=""
			aria-labelledby={`${id}-title`}
			class="flex flex-col gap-2">
			<h3
				id={`${id}-title`}
				class="flex items-center gap-2 border-b border-line-subtle pb-2 text-base font-semibold">
				{props.group.railLabel}
				<span class="text-sm font-normal text-fg-faint">
					{props.group.owners.length}
				</span>
			</h3>
			<ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2">
				{props.group.owners.map((owner) => (
					<OwnerRow api={props.api} update={props.update} owner={owner} />
				))}
			</ul>
		</section>
	)
}

export function UpdateSummary(props: { api: Api; updates: Update[]; update: Update }) {
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
				<div class="flex flex-col gap-6">
					{ownerSections(props.update).flatMap((section) =>
						section.groups.map((group) => (
							<GroupSummary
								api={props.api}
								update={props.update}
								group={group}
							/>
						)),
					)}
				</div>
			)}
		</section>
	)
}
