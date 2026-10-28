import { classDocsHref } from "../../api/docs-links.ts"
import { facetHref } from "../../api/routes.ts"
import { plainTags } from "../../api/tags.ts"
import type { Class } from "../../api/types.ts"
import { DocText } from "../../components/doc/DocText.tsx"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { HistoryFacts } from "../../components/history/HistoryFacts.tsx"
import { HistoryList } from "../../components/history/HistoryList.tsx"
import { ownerAnchor } from "../../api/update-text.ts"
import { classCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { TagList } from "../../components/TagList.tsx"
import { datedTags } from "../../components/history/deprecation.ts"

export function ClassHeader(props: {
	item: Class
	ancestors: Class[]
	memberCount: number
	inheritedCount: number
	siblingCount: number
	historyStart: string | undefined
}) {
	const path = [...props.ancestors].reverse().map((ancestor) => ancestor.name)
	return (
		<PageHeader
			crumbs={classCrumbs(path)}
			icon={
				props.item.icon === undefined ? "cube" : { classIcon: props.item.icon }
			}
			title={props.item.name}
			docs={{ href: classDocsHref(props.item.name), name: props.item.name }}
			code>
			<DocText
				doc={props.item.description}
				class="max-w-prose text-base text-fg"
			/>
			<TagList tags={datedTags(plainTags(props.item.tags), props.item.history)} />
			<FactList>
				<Fact
					label="Memory category"
					hint="The memory category that the API dump assigns to this class.">
					<a
						href={facetHref("memoryCategory", props.item.memoryCategory)}
						class="font-mono text-fg underline decoration-line-strong hover:decoration-fg">
						{props.item.memoryCategory}
					</a>
				</Fact>
				<Fact
					label="Own members"
					hint="Members that this class defines itself.">
					{props.memberCount}
				</Fact>
				<Fact
					label="Inherited members"
					hint="Members that this class inherits from its superclasses.">
					{props.inheritedCount}
				</Fact>
				<Fact
					label="Subclasses"
					hint="Classes that inherit directly from this class.">
					{props.item.subclasses.length}
				</Fact>
				<Fact
					label="Siblings"
					hint={
						props.item.superclass === undefined
							? "Classes without a superclass, including this class."
							: "Classes with the same superclass, including this class."
					}>
					{props.siblingCount}
				</Fact>
				<Fact
					label="Uses"
					hint="Properties, parameters, and return values that use this class as their type.">
					{props.item.usedBy.length}
				</Fact>
			</FactList>
			<FactList>
				<HistoryFacts
					history={props.item.history}
					kind="Class"
					name={props.item.name}
					start={props.historyStart}
				/>
			</FactList>
			<HistoryList
				history={props.item.history}
				fragment={ownerAnchor({ kind: "Class", name: props.item.name })}
			/>
		</PageHeader>
	)
}
