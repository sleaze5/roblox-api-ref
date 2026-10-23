import { facetDescription } from "../../api/facets/facet-info.ts"
import { useKindTitles, useSectionId, useSections } from "../../api/facets/facet-use.ts"
import type { Api, Facet } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { facetIcons } from "../../components/index/facet-icons.ts"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { indexGroupCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { FacetUseSections } from "../../components/index/FacetUseSections.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { FacetFacts } from "./FacetFacts.tsx"

export function FacetPage(props: { api: Api; facet: Facet }) {
	const sections = useSections(props.facet.uses)
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={
				<IndexSidebar
					api={props.api}
					current={{ group: props.facet.kind, name: props.facet.name }}
				/>
			}
			rail={
				<PageRail
					links={sections.map((section) => ({
						id: useSectionId(section.kind),
						label: useKindTitles[section.kind],
						count: section.uses.length,
					}))}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={indexGroupCrumbs(props.facet.kind)}
					icon={facetIcons[props.facet.kind]}
					title={props.facet.name}
					code>
					<PageLead>{facetDescription(props.facet)}</PageLead>
					<FacetFacts facet={props.facet} />
				</PageHeader>
				<ListFilter
					label="Filter uses"
					target="facet-uses"
					param="q"
					placement="page"
				/>
				<FacetUseSections id="facet-uses" sections={sections} />
			</article>
		</Shell>
	)
}
