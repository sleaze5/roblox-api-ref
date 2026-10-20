import { dataTypeCategoryTitles } from "../../api/data-type-info.ts"
import { dataTypeDocsHref } from "../../api/docs-links.ts"
import { unlinked } from "../../api/code-token.ts"
import { typeTokens } from "../../api/luau-type.ts"
import type { Api, DataType } from "../../api/types.ts"
import { CodeTokens } from "../../components/code/CodeTokens.tsx"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { indexGroupCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import {
	roleSections,
	usageCount,
	UsageSections,
} from "../../components/usage/UsageSections.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"

export function TypePage(props: { api: Api; item: DataType }) {
	const docs = dataTypeDocsHref(props.item.name)
	const luau = typeTokens({
		category: props.item.category,
		name: props.item.name,
		optional: false,
	})
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={
				<IndexSidebar
					api={props.api}
					current={{ group: "type", name: props.item.name }}
				/>
			}
			rail={
				<PageRail
					links={roleSections
						.map((section) => ({
							id: section.id,
							label: section.title,
							count: usageCount(props.item.usedBy, section.role),
						}))
						.filter((link) => link.count > 0)}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={indexGroupCrumbs("type")}
					icon="brackets-curly"
					title={props.item.name}
					docs={
						docs === undefined
							? undefined
							: { href: docs, name: props.item.name }
					}
					code>
					<FactList>
						<Fact
							label="Kind"
							hint="The category that the API dump gives this type.">
							{dataTypeCategoryTitles[props.item.category]}
						</Fact>
						<Fact
							label="Luau type"
							hint="How this type is written in Luau code.">
							<code class="text-fg">
								<CodeTokens tokens={unlinked(luau)} />
							</code>
						</Fact>
						<Fact
							label="Uses"
							hint="Properties, parameters, and return values that use this type.">
							{props.item.usedBy.length}
						</Fact>
					</FactList>
					{docs === undefined ? (
						<p class="text-fg-muted">
							Creator Hub has no known page for this type.
						</p>
					) : null}
				</PageHeader>
				<ListFilter
					label="Filter uses"
					target="type-uses"
					param="q"
					placement="page"
				/>
				<div id="type-uses" class="flex flex-col gap-8">
					<UsageSections usages={props.item.usedBy} name={props.item.name} />
				</div>
			</article>
		</Shell>
	)
}
