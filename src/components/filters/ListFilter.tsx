import { FilterField } from "./FilterField.tsx"

const placements = {
	page: "sticky top-header z-10 -mx-3 border-b border-line bg-canvas px-3 py-2",
	sidebar: "sticky -top-3 z-20 -mx-3 -mt-3 bg-surface p-3",
} as const

export function ListFilter(props: {
	label: string
	target: string
	param: string
	placement: keyof typeof placements
}) {
	return (
		<form
			data-list-filter={props.target}
			data-param={props.param}
			role="search"
			aria-label={props.label}
			class={placements[props.placement]}>
			<FilterField
				label={props.label}
				placeholder={props.label}
				count={props.placement === "page"}
			/>
		</form>
	)
}
