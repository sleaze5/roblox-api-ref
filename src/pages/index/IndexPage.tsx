import { facetPaths, facetTitles, sortedFacets } from "../../api/facets/facet-info.ts"
import { facetHref, facetIndexHref, typesHref } from "../../api/routes.ts"
import { facetKinds, type Api } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { facetIcons } from "../../components/index/facet-icons.ts"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { IndexRow } from "./IndexRow.tsx"
import { indexGridClass, IndexSection } from "./IndexSection.tsx"
import { DataTypeGroups } from "./DataTypeGroups.tsx"

function DataTypeSection(props: { api: Api }) {
	return (
		<IndexSection
			id="data-types"
			title="Data types"
			icon="brackets-curly"
			count={Object.keys(props.api.dataTypes).length}
			href={typesHref()}>
			<DataTypeGroups api={props.api} />
		</IndexSection>
	)
}

export function IndexPage(props: { api: Api }) {
	const facetLinks = facetKinds.map((kind) => ({
		id: facetPaths[kind],
		label: facetTitles[kind],
		count: Object.keys(props.api.facets[kind]).length,
	}))
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={<IndexSidebar api={props.api} />}
			rail={
				<PageRail
					links={[
						{
							id: "data-types",
							label: "Data types",
							count: Object.keys(props.api.dataTypes).length,
						},
						...facetLinks,
					]}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader crumbs={rootCrumbs} icon="books" title="Index">
					<PageLead>
						Every data type, tag, property category, security level,
						capability, thread safety level, and memory category found in
						the API dump. The dump does not describe them, so each page
						lists what uses it.
					</PageLead>
				</PageHeader>
				<ListFilter
					label="Filter the index"
					target="index-list"
					param="q"
					placement="page"
				/>
				<div id="index-list" class="flex flex-col gap-8">
					<DataTypeSection api={props.api} />
					{facetKinds.map((kind) => {
						const facets = sortedFacets(props.api, kind)
						return (
							<IndexSection
								id={facetPaths[kind]}
								title={facetTitles[kind]}
								icon={facetIcons[kind]}
								count={facets.length}
								href={facetIndexHref(kind)}>
								<ul class={indexGridClass}>
									{facets.map((facet) => (
										<IndexRow
											name={facet.name}
											href={facetHref(kind, facet.name)}
											count={facet.uses.length}
										/>
									))}
								</ul>
							</IndexSection>
						)
					})}
				</div>
			</article>
		</Shell>
	)
}
