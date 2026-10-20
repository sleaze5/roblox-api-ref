export type FlatTreeItem = {
	name: string
	href: string
	deprecated: boolean
	label?: string
}

const iconClasses = {
	enum: "tree-enum-icons",
	type: "tree-type-icons",
} as const

export function FlatTree(props: {
	id: string
	items: FlatTreeItem[]
	icon: keyof typeof iconClasses
	current?: string | undefined
}) {
	return (
		<ul id={props.id} data-tree="" class={["class-tree", iconClasses[props.icon]]}>
			{props.items.map((item) => (
				<li>
					<a
						href={item.href}
						aria-current={props.current === item.name ? "page" : undefined}
						data-deprecated={item.deprecated ? "" : undefined}>
						<span>{item.label ?? item.name}</span>
					</a>
				</li>
			))}
		</ul>
	)
}
