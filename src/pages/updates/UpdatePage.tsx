import { shownVersion } from "../../api/release.ts"
import { ownerAnchor, updateSummary } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import type { Update } from "../../api/update-types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { updatesCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"
import { IndexSection } from "../index/IndexSection.tsx"
import { OwnerGroup } from "./OwnerGroup.tsx"
import { ownerSections } from "./owner-sections.ts"
import { UpdateFacts } from "./UpdateFacts.tsx"
import { UpdateSidebar } from "./UpdateSidebar.tsx"

export function UpdatePage(props: { api: Api; updates: Update[]; item: Update }) {
	const sections = ownerSections(props.item)
	return (
		<Shell
			section="updates"
			release={props.api}
			sidebar={
				<UpdateSidebar updates={props.updates} current={props.item.version} />
			}
			rail={
				<PageRail
					links={sections.map((section) => ({
						id: section.id,
						label: section.title,
						count: section.owners.length,
						entries: section.owners.map((owner) => ({
							id: ownerAnchor(owner),
							label: owner.name,
						})),
					}))}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader
					crumbs={updatesCrumbs}
					icon="git-diff"
					title={shownVersion(props.item).version}
					code>
					<PageLead>{updateSummary(props.item)}</PageLead>
					<UpdateFacts
						update={props.item}
						updates={props.updates}
						published
					/>
				</PageHeader>
				{sections.length === 0 ? (
					<p class="text-fg-muted">
						The API did not change between these versions.
					</p>
				) : (
					<>
						<ListFilter
							label="Filter changes"
							target="update-changes"
							param="q"
							placement="page"
						/>
						<div
							id="update-changes"
							data-changes=""
							class="flex flex-col gap-8">
							{sections.map((section) => (
								<IndexSection
									id={section.id}
									title={section.title}
									icon={section.icon}
									count={section.owners.length}>
									<div class="flex flex-col gap-6">
										{section.owners.map((owner) => (
											<OwnerGroup api={props.api} owner={owner} />
										))}
									</div>
								</IndexSection>
							))}
						</div>
					</>
				)}
			</article>
		</Shell>
	)
}
