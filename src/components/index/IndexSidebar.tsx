import {
	dataTypeCategories,
	dataTypeCategoryTitles,
	sortedDataTypes,
} from "../../api/data-type-info.ts"
import { facetTitles, sortedFacets } from "../../api/facets/facet-info.ts"
import {
	facetHref,
	facetIndexHref,
	removedHref,
	typeHref,
	typesHref,
	updateHref,
} from "../../api/routes.ts"
import type { RemovedOwner } from "../../api/item-history.ts"
import { ownerAnchor } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { facetKinds, type Api, type IndexGroup } from "../../api/types.ts"
import { SectionSidebar } from "../../layout/SectionSidebar.tsx"

type SidebarGroup = IndexGroup | "removed"

export type IndexCurrent = {
	group: SidebarGroup
	name?: string
}

type Leaf = {
	name: string
	href: string
}

type Branch = {
	title: string
	href?: string
	leaves: Leaf[]
	branches: Branch[]
}

type Group = Branch & {
	key: SidebarGroup
}

function dataTypeGroup(api: Api): Group {
	const types = sortedDataTypes(api.dataTypes)
	return {
		key: "type",
		title: "Data types",
		href: typesHref(),
		leaves: [],
		branches: dataTypeCategories.map((category) => ({
			title: dataTypeCategoryTitles[category],
			leaves: types
				.filter((item) => item.category === category)
				.map((item) => ({ name: item.name, href: typeHref(item.name) })),
			branches: [],
		})),
	}
}

function removedGroup(api: Api): Group {
	const leaves = (kind: OwnerKind, owners: RemovedOwner[]) =>
		owners.map((owner) => ({
			name: owner.name,
			href: `${updateHref(owner.removed.version)}#${ownerAnchor({ kind, name: owner.name })}`,
		}))
	return {
		key: "removed",
		title: "Removed",
		href: removedHref(),
		leaves: [],
		branches: [
			{
				title: "Classes",
				leaves: leaves("Class", api.removed?.classes ?? []),
				branches: [],
			},
			{
				title: "Enums",
				leaves: leaves("Enum", api.removed?.enums ?? []),
				branches: [],
			},
		],
	}
}

function groupsOf(api: Api): Group[] {
	return [
		dataTypeGroup(api),
		...facetKinds.map((kind) => ({
			key: kind,
			title: facetTitles[kind],
			href: facetIndexHref(kind),
			leaves: sortedFacets(api, kind).map((facet) => ({
				name: facet.name,
				href: facetHref(kind, facet.name),
			})),
			branches: [],
		})),
		removedGroup(api),
	]
}

function holds(branch: Branch, name: string): boolean {
	return (
		branch.leaves.some((leaf) => leaf.name === name) ||
		branch.branches.some((child) => holds(child, name))
	)
}

function BranchNode(props: {
	branch: Branch
	depth: number
	current: string | undefined
	active: boolean
}) {
	const open =
		props.active &&
		(props.current === undefined || holds(props.branch, props.current))
	return (
		<details open={open}>
			<summary style={`--depth: ${props.depth}`}>
				{props.branch.href === undefined ? (
					<span>{props.branch.title}</span>
				) : (
					<a
						href={props.branch.href}
						aria-current={
							props.active && props.current === undefined
								? "page"
								: undefined
						}>
						<span>{props.branch.title}</span>
					</a>
				)}
			</summary>
			<ul>
				{props.branch.branches.map((child) => (
					<li>
						<BranchNode
							branch={child}
							depth={props.depth + 1}
							current={props.current}
							active={props.active}
						/>
					</li>
				))}
				{props.branch.leaves.map((leaf) => (
					<li>
						<a
							href={leaf.href}
							aria-current={
								props.current === leaf.name ? "page" : undefined
							}>
							<span>{leaf.name}</span>
						</a>
					</li>
				))}
			</ul>
		</details>
	)
}

export function IndexSidebar(props: { api: Api; current?: IndexCurrent }) {
	return (
		<SectionSidebar label="Filter the index" target="sidebar-tree">
			<ul
				id="sidebar-tree"
				data-tree=""
				data-sticky-tree=""
				class="tree-sticky class-tree tree-index-icons">
				{groupsOf(props.api).map((group) => (
					<li data-group={group.key}>
						<BranchNode
							branch={group}
							depth={0}
							current={
								props.current?.group === group.key
									? props.current.name
									: undefined
							}
							active={props.current?.group === group.key}
						/>
					</li>
				))}
			</ul>
		</SectionSidebar>
	)
}
