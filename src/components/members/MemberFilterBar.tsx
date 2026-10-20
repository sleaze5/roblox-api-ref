import { memberKindTitles } from "../../api/members.ts"
import type { FilterTagGroup } from "../../api/member-filter-tags.ts"
import type { MemberKind } from "../../api/types.ts"
import { FilterField } from "../filters/FilterField.tsx"
import { FilterMenu } from "./FilterMenu.tsx"

function Toggle(props: { name: string; label: string }) {
	return (
		<label class="chip">
			<input type="checkbox" name={props.name} checked class="sr-only" />
			{props.label}
		</label>
	)
}

export function MemberFilterBar(props: {
	kinds: MemberKind[]
	tags: FilterTagGroup[]
}) {
	return (
		<form
			data-member-filter=""
			role="search"
			aria-label="Filter members"
			class="z-10 -mx-3 flex flex-col gap-2 border-b border-line bg-canvas px-3 py-2 md:sticky md:top-header">
			<div class="flex flex-wrap items-center gap-2">
				<FilterField
					label="Filter members by name"
					placeholder="Filter members"
					count
					class="min-w-48 flex-1"
				/>
				<FilterMenu groups={props.tags} />
			</div>
			<div class="flex flex-wrap items-center gap-2">
				<fieldset class="flex flex-wrap gap-1">
					<legend class="sr-only">Member kinds</legend>
					{props.kinds.map((kind) => (
						<Toggle
							name={`kind-${kind.toLowerCase()}`}
							label={memberKindTitles[kind]}
						/>
					))}
				</fieldset>
				<button
					type="button"
					data-collapse-all="collapse"
					class="ml-auto button">
					Collapse all
				</button>
			</div>
		</form>
	)
}
