import {
	dataTypeCategories,
	dataTypeCategoryTitles,
	sortedDataTypes,
} from "../../api/data-type-info.ts"
import { typeHref } from "../../api/routes.ts"
import type { Api, DataType } from "../../api/types.ts"
import { IndexRow } from "./IndexRow.tsx"
import { indexGridClass, IndexSection } from "./IndexSection.tsx"

function TypeRows(props: { types: DataType[] }) {
	return (
		<ul class={indexGridClass}>
			{props.types.map((item) => (
				<IndexRow
					name={item.name}
					href={typeHref(item.name)}
					count={item.usedBy.length}
					type
				/>
			))}
		</ul>
	)
}

export function DataTypeGroups(props: { api: Api; standalone?: boolean }) {
	const types = sortedDataTypes(props.api.dataTypes)
	return dataTypeCategories.map((category) => {
		const members = types.filter((item) => item.category === category)
		return props.standalone ? (
			<IndexSection
				id={category}
				title={dataTypeCategoryTitles[category]}
				icon="brackets-curly"
				count={members.length}>
				<TypeRows types={members} />
			</IndexSection>
		) : (
			<section
				id={category}
				data-list-section=""
				aria-labelledby={`${category}-title`}
				class="flex flex-col gap-1">
				<h3
					id={`${category}-title`}
					class="px-2 text-xs font-medium text-fg-muted">
					{dataTypeCategoryTitles[category]}
				</h3>
				<TypeRows types={members} />
			</section>
		)
	})
}
