import { updateHref } from "../../api/routes.ts"
import { changeTotal, updateAnchor, updatesLead } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { PageRail, type RailLink } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { IndexRow } from "../index/IndexRow.tsx"
import { IndexSection, indexGridClass } from "../index/IndexSection.tsx"
import { UpdateSection } from "./UpdateSection.tsx"
import { UpdateSidebar } from "./UpdateSidebar.tsx"

const recentCount = 10

function railLinks(recent: Update[], earlier: Update[]): RailLink[] {
	const links: RailLink[] = recent.map((update) => ({
		id: updateAnchor(update.version),
		label: update.version,
		count: changeTotal(update),
		countHint: "Changes in this update.",
	}))
	return earlier.length === 0
		? links
		: [
				...links,
				{
					id: "earlier-updates",
					label: "Earlier updates",
					count: earlier.length,
				},
			]
}

function UpdateList(props: { api: Api; updates: Update[] }) {
	const recent = props.updates.slice(0, recentCount)
	const earlier = props.updates.slice(recentCount)
	return (
		<>
			<ListFilter
				label="Filter changes"
				target="update-changes"
				param="q"
				placement="page"
			/>
			<div id="update-changes" data-changes="" class="flex flex-col gap-12">
				{recent.map((update) => (
					<UpdateSection
						api={props.api}
						updates={props.updates}
						update={update}
					/>
				))}
			</div>
			{earlier.length === 0 ? null : (
				<IndexSection
					id="earlier-updates"
					title="Earlier updates"
					icon="git-diff"
					count={earlier.length}>
					<ul class={indexGridClass}>
						{earlier.map((update) => (
							<IndexRow
								name={update.version}
								href={updateHref(update.version)}
								count={changeTotal(update)}
							/>
						))}
					</ul>
				</IndexSection>
			)}
		</>
	)
}

export function UpdatesPage(props: { api: Api; updates: Update[] }) {
	const recent = props.updates.slice(0, recentCount)
	const earlier = props.updates.slice(recentCount)
	return (
		<Shell
			section="updates"
			release={props.api}
			sidebar={<UpdateSidebar updates={props.updates} />}
			rail={<PageRail links={railLinks(recent, earlier)} />}>
			<article class="flex flex-col gap-6">
				<PageHeader crumbs={rootCrumbs} icon="git-diff" title="Updates">
					<PageLead>{updatesLead}</PageLead>
				</PageHeader>
				{props.updates.length === 0 ? (
					<p class="text-fg-muted">
						There are no updates yet. An update appears here when the site
						moves to a new Studio version.
					</p>
				) : (
					<UpdateList api={props.api} updates={props.updates} />
				)}
			</article>
		</Shell>
	)
}
