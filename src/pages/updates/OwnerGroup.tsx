import { Dynamic } from "@solidjs/web"
import { ownerAnchor } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import type { ChangeStatus, OwnerChanges } from "../../api/update-types.ts"
import { ChangeDetails } from "./ChangeDetails.tsx"
import { ChangeEntryRow } from "./ChangeEntryRow.tsx"
import { ChangeMark } from "./ChangeMark.tsx"
import { DescriptionChanges } from "./DescriptionChanges.tsx"
import { OwnerName } from "./OwnerName.tsx"

const kindNames = { Class: "class", Enum: "enum" } as const

function statusLabel(owner: OwnerChanges): string | undefined {
	const labels: Partial<Record<ChangeStatus, string>> = {
		Added: `Added ${kindNames[owner.kind]}`,
		Removed: `Removed ${kindNames[owner.kind]}`,
	}
	return owner.status === undefined ? undefined : labels[owner.status]
}

export function OwnerGroup(props: {
	api: Api
	owner: OwnerChanges
	idPrefix?: string
	heading?: "h3" | "h4"
}) {
	const id = `${props.idPrefix ?? ""}${ownerAnchor(props.owner)}`
	const label = statusLabel(props.owner)
	const status =
		props.owner.status ??
		(props.owner.descriptions.length > 0 ? "Changed" : undefined)
	return (
		<section
			id={id}
			data-change-group=""
			data-name={props.owner.name}
			aria-labelledby={`${id}-title`}
			class="flex flex-col gap-2">
			<Dynamic
				component={props.heading ?? "h3"}
				id={`${id}-title`}
				data-change={status === undefined ? undefined : ""}
				class="change-line">
				<ChangeMark status={status} />
				<span class="flex min-w-0 items-center gap-2">
					<span
						aria-hidden="true"
						data-kind={props.owner.kind}
						class="kind-icon text-fg-muted"
					/>
					<span class="sr-only">{label ?? kindNames[props.owner.kind]}</span>
					<OwnerName api={props.api} owner={props.owner} />
					{label === undefined ? null : (
						<span aria-hidden="true" class="shrink-0 text-xs text-fg-faint">
							{label}
						</span>
					)}
				</span>
			</Dynamic>
			<ChangeDetails api={props.api} details={props.owner.details} />
			<DescriptionChanges details={props.owner.descriptions} />
			{props.owner.entries.length === 0 ? null : (
				<ul class="flex flex-col gap-2">
					{props.owner.entries.map((entry) => (
						<ChangeEntryRow api={props.api} entry={entry} />
					))}
				</ul>
			)}
		</section>
	)
}
