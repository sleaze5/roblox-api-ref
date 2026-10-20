import type { JSX } from "@solidjs/web"

export function PageLead(props: { children: JSX.Element }) {
	return <p class="max-w-prose text-base text-fg-muted">{props.children}</p>
}
