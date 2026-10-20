import type { JSX } from "@solidjs/web"

export function Fact(props: { label: string; hint?: string; children: JSX.Element }) {
	return (
		<div class="flex items-center gap-2">
			<dt
				data-tooltip={props.hint}
				class={[
					"text-fg-faint",
					props.hint !== undefined &&
						"cursor-help underline decoration-line-strong decoration-dotted underline-offset-4",
				]}>
				{props.label}
			</dt>
			<dd class="text-fg-muted">{props.children}</dd>
		</div>
	)
}
