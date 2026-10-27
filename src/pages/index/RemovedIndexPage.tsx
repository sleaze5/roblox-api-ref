import type { RemovedOwner } from "../../api/item-history.ts"
import { removedLead } from "../../api/removed-text.ts"
import { updateHref } from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { formatDate, ownerAnchor } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { IndexSidebar } from "../../components/index/IndexSidebar.tsx"
import { indexCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { IndexSection } from "./IndexSection.tsx"

const groups = [
	{ kind: "Class", id: "removed-classes", title: "Classes", icon: "cube" },
	{ kind: "Enum", id: "removed-enums", title: "Enums", icon: "list-bullets" },
] as const

function owners(api: Api, kind: OwnerKind): RemovedOwner[] {
	return kind === "Class" ? (api.removed?.classes ?? []) : (api.removed?.enums ?? [])
}

export function RemovedRow(props: { owner: RemovedOwner; kind: OwnerKind }) {
	return (
		<li
			data-name={props.owner.name}
			class="flex flex-wrap items-baseline gap-x-3 px-2 py-1">
			<code class="text-fg-muted">{props.owner.name}</code>
			<span class="text-fg-faint">
				Removed in{" "}
				<a
					href={`${updateHref(props.owner.removed.version)}#${ownerAnchor({ kind: props.kind, name: props.owner.name })}`}
					class="font-mono text-fg-muted hover:text-fg hover:underline">
					{props.owner.removed.version}
				</a>
			</span>
			<span class="text-xs text-fg-faint">
				{formatDate(props.owner.removed.published)}
			</span>
		</li>
	)
}

export function RemovedIndexPage(props: { api: Api }) {
	return (
		<Shell
			section="index"
			release={props.api}
			sidebar={<IndexSidebar api={props.api} current={{ group: "removed" }} />}
			rail={
				<PageRail
					links={groups.map((group) => ({
						id: group.id,
						label: group.title,
						count: owners(props.api, group.kind).length,
					}))}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={indexCrumbs}
					icon="clock-counter-clockwise"
					title="Removed">
					<PageLead>{removedLead}</PageLead>
				</PageHeader>
				<ListFilter
					label="Filter removed"
					target="removed-list"
					param="q"
					placement="page"
				/>
				<div id="removed-list" class="flex flex-col gap-8">
					{groups.map((group) => (
						<IndexSection
							id={group.id}
							title={group.title}
							icon={group.icon}
							count={owners(props.api, group.kind).length}>
							<ul class="flex flex-col">
								{owners(props.api, group.kind).map((owner) => (
									<RemovedRow owner={owner} kind={group.kind} />
								))}
							</ul>
						</IndexSection>
					))}
				</div>
			</article>
		</Shell>
	)
}
