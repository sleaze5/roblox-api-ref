import { dataTypeCategories, dataTypeCategoryTitles } from "../../api/data-type-info.ts"
import type { Api } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { indexCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { DataTypeGroups } from "./DataTypeGroups.tsx"

export function TypeIndexPage(props: { api: Api }) {
	const types = Object.values(props.api.dataTypes)
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={<IndexSidebar api={props.api} current={{ group: "type" }} />}
			rail={
				<PageRail
					links={dataTypeCategories.map((category) => ({
						id: category,
						label: dataTypeCategoryTitles[category],
						count: types.filter((item) => item.category === category)
							.length,
					}))}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={indexCrumbs}
					icon="brackets-curly"
					title="Data types">
					<PageLead>
						All {types.length} data types: primitive types, group types, and
						Roblox types.
					</PageLead>
				</PageHeader>
				<ListFilter
					label="Filter data types"
					target="type-list"
					param="q"
					placement="page"
				/>
				<div id="type-list" class="flex flex-col gap-8">
					<DataTypeGroups api={props.api} standalone />
				</div>
			</article>
		</Shell>
	)
}
