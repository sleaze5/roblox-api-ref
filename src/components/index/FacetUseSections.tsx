import type { Api } from "../../api/types.ts"
import {
	useKindTitles,
	useSectionId,
	type UseSection,
} from "../../api/facets/facet-use.ts"
import { Icon } from "../Icon.tsx"
import { useKindIcons } from "./facet-icons.ts"
import { FacetUseRow } from "./FacetUseRow.tsx"

export function FacetUseSections(props: {
	api: Api
	id: string
	sections: UseSection[]
}) {
	return (
		<div id={props.id} class="flex flex-col gap-8">
			{props.sections.map((section) => {
				const id = useSectionId(section.kind)
				return (
					<section
						id={id}
						data-list-section=""
						aria-labelledby={`${id}-title`}
						class="flex flex-col gap-3">
						<h2
							id={`${id}-title`}
							class="flex items-center gap-2 text-lg font-semibold">
							<Icon
								name={useKindIcons[section.kind]}
								size={20}
								class="text-fg-muted"
							/>
							{useKindTitles[section.kind]}
							<span class="text-sm font-normal text-fg-faint">
								{section.uses.length}
							</span>
						</h2>
						<ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2">
							{section.uses.map((use) => (
								<FacetUseRow api={props.api} use={use} />
							))}
						</ul>
					</section>
				)
			})}
		</div>
	)
}
