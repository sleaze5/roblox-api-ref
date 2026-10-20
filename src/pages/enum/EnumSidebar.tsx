import { compareNames } from "../../api/compare-names.ts"
import { enumHref } from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import { FlatTree } from "../../components/tree/FlatTree.tsx"
import { SectionSidebar } from "../../layout/SectionSidebar.tsx"

export function EnumSidebar(props: { api: Api; current?: string }) {
	const items = Object.values(props.api.enums)
		.sort((a, b) => compareNames(a.name, b.name))
		.map((item) => ({
			name: item.name,
			href: enumHref(item.name),
			deprecated: item.tags.includes("Deprecated"),
		}))
	return (
		<SectionSidebar label="Filter enums" target="sidebar-tree">
			<FlatTree
				id="sidebar-tree"
				items={items}
				icon="enum"
				current={props.current}
			/>
		</SectionSidebar>
	)
}
