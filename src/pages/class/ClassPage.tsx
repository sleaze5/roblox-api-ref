import {
	ancestorsOf,
	memberKinds,
	memberKindTitles,
	membersInPageOrder,
} from "../../api/members.ts"
import { filterTagGroups } from "../../api/member-filter-tags.ts"
import { memberAnchor } from "../../api/routes.ts"
import type { Api, Class, MemberKind } from "../../api/types.ts"
import { MemberFilterBar } from "../../components/members/MemberFilterBar.tsx"
import { MemberSection, sectionId } from "../../components/members/MemberSection.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { ClassHeader } from "./ClassHeader.tsx"
import { ClassSidebar } from "./ClassSidebar.tsx"

function countOfKind(classes: Class[], kind: MemberKind): number {
	return classes.reduce(
		(total, item) =>
			total + item.members.filter((member) => member.kind === kind).length,
		0,
	)
}

function siblingCount(api: Api, item: Class): number {
	const superclass = item.superclass
	return superclass === undefined
		? Object.values(api.classes).filter((other) => other.superclass === undefined)
				.length
		: (api.classes[superclass]?.subclasses.length ?? 1)
}

export function ClassPage(props: { api: Api; item: Class }) {
	const ancestors = ancestorsOf(props.api, props.item)
	const lineage = [props.item, ...ancestors]
	const kinds = memberKinds.filter((kind) => countOfKind(lineage, kind) > 0)
	return (
		<Shell
			section="classes"
			release={props.api}
			sidebar={
				<ClassSidebar api={props.api} item={props.item} ancestors={ancestors} />
			}
			rail={
				<PageRail
					links={kinds.map((kind) => ({
						id: sectionId(kind),
						label: memberKindTitles[kind],
						count: countOfKind([props.item], kind),
						countHint: `${memberKindTitles[kind]} that this class defines itself.`,
						entries: membersInPageOrder(props.item.members, kind).map(
							(member) => ({
								id: memberAnchor(kind, member.name),
								label: member.name,
							}),
						),
					}))}
				/>
			}>
			<article class="flex flex-col gap-6">
				<ClassHeader
					item={props.item}
					ancestors={ancestors}
					memberCount={props.item.members.length}
					siblingCount={siblingCount(props.api, props.item)}
					inheritedCount={ancestors.reduce(
						(sum, ancestor) => sum + ancestor.members.length,
						0,
					)}
				/>
				{kinds.length === 0 ? (
					<p class="text-fg-muted">This class has no members.</p>
				) : (
					<>
						<MemberFilterBar
							kinds={kinds}
							tags={filterTagGroups(
								lineage.flatMap((item) => item.members),
							)}
						/>
						{kinds.map((kind) => (
							<MemberSection
								kind={kind}
								item={props.item}
								ancestors={ancestors}
							/>
						))}
					</>
				)}
			</article>
		</Shell>
	)
}
