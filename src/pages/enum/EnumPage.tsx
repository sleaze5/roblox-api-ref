import { enumDocsHref } from "../../api/docs-links.ts"
import { plainTags } from "../../api/tags.ts"
import type { Api, Enum } from "../../api/types.ts"
import { DocText } from "../../components/doc/DocText.tsx"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { HistoryFacts } from "../../components/history/HistoryFacts.tsx"
import { HistoryList } from "../../components/history/HistoryList.tsx"
import { ownerAnchor } from "../../api/update-text.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { Icon } from "../../components/Icon.tsx"
import { enumCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { TagList } from "../../components/TagList.tsx"
import {
	roleSections,
	usageCount,
	UsageSections,
} from "../../components/usage/UsageSections.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { EnumItemRow } from "./EnumItemRow.tsx"
import { EnumSidebar } from "./EnumSidebar.tsx"

export function EnumPage(props: { api: Api; item: Enum }) {
	const usageLinks = roleSections
		.map((section) => ({
			id: section.id,
			label: section.title,
			count: usageCount(props.item.usedBy, section.role),
		}))
		.filter((link) => link.count > 0)
	return (
		<Shell
			section="enums"
			release={props.api}
			sidebar={<EnumSidebar api={props.api} current={props.item.name} />}
			rail={
				<PageRail
					links={[
						{ id: "items", label: "Items", count: props.item.items.length },
						...usageLinks,
					]}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={enumCrumbs}
					icon="list-bullets"
					title={
						<>
							<span class="text-fg-muted">Enum.</span>
							{props.item.name}
						</>
					}
					docs={{
						href: enumDocsHref(props.item.name),
						name: `Enum.${props.item.name}`,
					}}
					code>
					<DocText
						doc={props.item.description}
						class="max-w-prose text-base text-fg"
					/>
					<TagList tags={plainTags(props.item.tags)} />
					<FactList>
						<Fact label="Items" hint="Enum items that this enum defines.">
							{props.item.items.length}
						</Fact>
						<Fact
							label="Uses"
							hint="Properties, parameters, and return values that use this enum.">
							{props.item.usedBy.length}
						</Fact>
					</FactList>
					<FactList>
						<HistoryFacts
							history={props.item.history}
							kind="Enum"
							name={props.item.name}
							start={props.api.historyStart}
						/>
					</FactList>
					<HistoryList
						history={props.item.history}
						fragment={ownerAnchor({ kind: "Enum", name: props.item.name })}
					/>
				</PageHeader>
				<ListFilter
					label="Filter items and uses"
					target="enum-content"
					param="q"
					placement="page"
				/>
				<div id="enum-content" class="flex flex-col gap-8">
					<section
						id="items"
						data-list-section=""
						aria-labelledby="items-title"
						class="flex flex-col gap-3">
						<h2
							id="items-title"
							class="flex items-center gap-2 text-lg font-semibold">
							<Icon name="hash" size={20} class="text-fg-muted" />
							Items
							<span class="text-sm font-normal text-fg-faint">
								{props.item.items.length}
							</span>
						</h2>
						<ul class="flex flex-col gap-4">
							{props.item.items.map((item) => (
								<EnumItemRow owner={props.item} item={item} />
							))}
						</ul>
					</section>
					<UsageSections
						usages={props.item.usedBy}
						name={`Enum.${props.item.name}`}
					/>
				</div>
			</article>
		</Shell>
	)
}
