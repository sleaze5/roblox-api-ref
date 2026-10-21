import { shownVersion } from "../../api/release.ts"
import { updateHref } from "../../api/routes.ts"
import type { Update } from "../../api/update-types.ts"
import { FlatTree } from "../../components/tree/FlatTree.tsx"
import { SectionSidebar } from "../../layout/SectionSidebar.tsx"

export function UpdateSidebar(props: { updates: Update[]; current?: string }) {
	const items = props.updates.map((update) => ({
		name: update.version,
		href: updateHref(update.version),
		label: shownVersion(update).version,
		deprecated: false,
	}))
	return (
		<SectionSidebar label="Filter updates" target="sidebar-tree">
			<FlatTree
				id="sidebar-tree"
				items={items}
				icon="update"
				current={props.current}
			/>
		</SectionSidebar>
	)
}
