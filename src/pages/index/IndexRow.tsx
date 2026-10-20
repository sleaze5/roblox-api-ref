export function IndexRow(props: {
	name: string
	href: string
	count: number
	type?: boolean
}) {
	return (
		<li data-name={props.name}>
			<a
				href={props.href}
				class={[
					"flex h-7 items-center justify-between gap-2 rounded-sm px-2 font-mono motion-colors hover:bg-hover",
					props.type ? "text-code-type" : "text-fg",
				]}>
				<span class="truncate">{props.name}</span>
				<span class="font-sans text-xs text-fg-faint">{props.count}</span>
			</a>
		</li>
	)
}
