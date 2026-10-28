import { compareNames } from "../../api/compare-names.ts"
import { isDeprecated } from "../../api/members.ts"
import { classHref } from "../../api/routes.ts"
import type { Api, Class } from "../../api/types.ts"
import { classIconStyle } from "../ClassIcon.tsx"

type TreeOptions = {
	api: Api
	current?: string | undefined
	open: Set<string>
	counts: boolean
}

function childrenOf(api: Api, item: Class): Class[] {
	return item.subclasses.flatMap((name) => {
		const child = api.classes[name]
		return child === undefined ? [] : [child]
	})
}

function NodeLink(props: { item: Class; options: TreeOptions }) {
	return (
		<a
			href={classHref(props.item.name)}
			style={classIconStyle(props.item.icon)}
			aria-current={
				props.options.current === props.item.name ? "page" : undefined
			}
			data-deprecated={isDeprecated(props.item) ? "" : undefined}>
			<span>{props.item.name}</span>
		</a>
	)
}

function ClassNode(props: { item: Class; options: TreeOptions; depth: number }) {
	const children = childrenOf(props.options.api, props.item)
	if (children.length === 0) {
		return (
			<li>
				<NodeLink item={props.item} options={props.options} />
			</li>
		)
	}
	return (
		<li>
			<details open={props.options.open.has(props.item.name)}>
				<summary style={`--depth: ${props.depth}`}>
					<NodeLink item={props.item} options={props.options} />
					{props.options.counts ? <small>{children.length}</small> : null}
				</summary>
				<ul>
					{children.map((child) => (
						<ClassNode
							item={child}
							options={props.options}
							depth={props.depth + 1}
						/>
					))}
				</ul>
			</details>
		</li>
	)
}

export function ClassTree(props: {
	api: Api
	id: string
	current?: string | undefined
	open: Set<string>
	counts: boolean
	sticky?: boolean
}) {
	const roots = Object.values(props.api.classes)
		.filter((item) => item.superclass === undefined)
		.sort((a, b) => compareNames(a.name, b.name))
	const options: TreeOptions = {
		api: props.api,
		open: props.open,
		counts: props.counts,
		current: props.current,
	}
	return (
		<ul
			id={props.id}
			data-tree=""
			data-sticky-tree={props.sticky ? "" : undefined}
			class={[
				"class-tree tree-class-icons",
				props.sticky ? "tree-sticky" : undefined,
			]}>
			{roots.map((item) => (
				<ClassNode item={item} options={options} depth={0} />
			))}
		</ul>
	)
}
