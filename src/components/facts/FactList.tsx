import type { JSX } from "@solidjs/web"

export function FactList(props: { children: JSX.Element }) {
	return <dl class="flex flex-wrap gap-x-6 gap-y-1">{props.children}</dl>
}
