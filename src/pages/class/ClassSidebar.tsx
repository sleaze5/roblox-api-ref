import type { Api, Class } from "../../api/types.ts"
import { ClassTree } from "../../components/tree/ClassTree.tsx"
import { SectionSidebar } from "../../layout/SectionSidebar.tsx"

export function ClassSidebar(props: { api: Api; item?: Class; ancestors: Class[] }) {
	const open = new Set(
		[props.item, ...props.ancestors].flatMap((item) =>
			item === undefined ? [] : [item.name],
		),
	)
	return (
		<SectionSidebar label="Filter classes" target="sidebar-tree">
			<ClassTree
				api={props.api}
				id="sidebar-tree"
				current={props.item?.name}
				open={open}
				counts={false}
				sticky
			/>
		</SectionSidebar>
	)
}
