import { compareNames } from "../../api/compare-names.ts"
import { enumHref } from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { EnumSidebar } from "../enum/EnumSidebar.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"

export function EnumIndexPage(props: { api: Api }) {
	const enums = Object.values(props.api.enums).sort((a, b) =>
		compareNames(a.name, b.name),
	)
	return (
		<Shell
			section="enums"
			release={props.api}
			sidebar={<EnumSidebar api={props.api} />}
			rail={
				<PageRail
					links={[{ id: "enum-list", label: "Enums", count: enums.length }]}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader crumbs={rootCrumbs} icon="list-bullets" title="Enums">
					<PageLead>
						All {enums.length} enums, in alphabetical order.
					</PageLead>
				</PageHeader>
				<ListFilter
					label="Filter enums"
					target="enum-list"
					param="q"
					placement="page"
				/>
				<ul
					id="enum-list"
					class="grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3">
					{enums.map((item) => (
						<li data-name={item.name}>
							<a
								href={enumHref(item.name)}
								aria-label={
									item.tags.includes("Deprecated")
										? `${item.name} (deprecated)`
										: undefined
								}
								class={[
									"flex h-7 items-center justify-between gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover",
									item.tags.includes("Deprecated")
										? "text-code-type-muted hover:text-code-type focus-visible:text-code-type"
										: "text-code-type",
								]}>
								<span class="truncate">{item.name}</span>
								<span class="font-sans text-xs text-fg-faint">
									{item.items.length}
								</span>
							</a>
						</li>
					))}
				</ul>
			</article>
		</Shell>
	)
}
