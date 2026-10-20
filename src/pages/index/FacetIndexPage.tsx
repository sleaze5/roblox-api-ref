import {
	facetIndexDescriptions,
	facetTitles,
	sortedFacets,
} from "../../api/facets/facet-info.ts"
import { facetHref } from "../../api/routes.ts"
import type { Api, FacetKind } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { facetIcons } from "../../components/index/facet-icons.ts"
import { indexCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { IndexRow } from "./IndexRow.tsx"
import { indexGridClass } from "./IndexSection.tsx"

export function FacetIndexPage(props: { api: Api; kind: FacetKind }) {
	const facets = sortedFacets(props.api, props.kind)
	const filterLabel = `Filter ${facetTitles[props.kind].toLowerCase()}`
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={<IndexSidebar api={props.api} current={{ group: props.kind }} />}
			rail={
				<PageRail
					links={[
						{
							id: "facet-list",
							label: facetTitles[props.kind],
							count: facets.length,
						},
					]}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={indexCrumbs}
					icon={facetIcons[props.kind]}
					title={facetTitles[props.kind]}>
					<PageLead>{facetIndexDescriptions[props.kind]}</PageLead>
				</PageHeader>
				<ListFilter
					label={filterLabel}
					target="facet-list"
					param="q"
					placement="page"
				/>
				<ul id="facet-list" class={indexGridClass}>
					{facets.map((facet) => (
						<IndexRow
							name={facet.name}
							href={facetHref(props.kind, facet.name)}
							count={facet.uses.length}
						/>
					))}
				</ul>
			</article>
		</Shell>
	)
}
