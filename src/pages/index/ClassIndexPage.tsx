import type { Api } from "../../api/types.ts"
import { ListFilter } from "../../components/filters/ListFilter.tsx"
import { ClassTree } from "../../components/tree/ClassTree.tsx"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { ClassSidebar } from "../class/ClassSidebar.tsx"
import { PageRail } from "../../layout/PageRail.tsx"
import { Shell } from "../../layout/Shell.tsx"

export function ClassIndexPage(props: { api: Api }) {
	return (
		<Shell
			section="classes"
			release={props.api}
			sidebar={<ClassSidebar api={props.api} ancestors={[]} />}
			rail={
				<PageRail
					links={[
						{
							id: "class-tree",
							label: "Classes",
							count: Object.keys(props.api.classes).length,
						},
					]}
				/>
			}>
			<article class="flex flex-col gap-6">
				<PageHeader crumbs={rootCrumbs} icon="tree-structure" title="Classes">
					<PageLead>
						All {Object.keys(props.api.classes).length} classes, arranged by
						inheritance.
					</PageLead>
				</PageHeader>
				<ListFilter
					label="Filter classes"
					target="class-tree"
					param="q"
					placement="page"
				/>
				<ClassTree
					api={props.api}
					id="class-tree"
					open={new Set(Object.keys(props.api.classes))}
					counts
				/>
			</article>
		</Shell>
	)
}
