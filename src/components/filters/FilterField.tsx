import { Icon } from "../Icon.tsx"

export function FilterField(props: {
	label: string
	placeholder: string
	count?: boolean
	class?: string
}) {
	return (
		<div class={["relative flex items-center", props.class]}>
			<label class="flex min-w-0 flex-1 items-center">
				<span class="sr-only">{props.label}</span>
				<Icon
					name="magnifying-glass"
					size={14}
					class="pointer-events-none absolute left-3 text-fg-muted"
				/>
				<input
					type="search"
					name="text"
					placeholder={props.placeholder}
					autocomplete="off"
					spellcheck={false}
					class={[
						"field field-icon",
						props.count ? "field-count" : undefined,
					]}
				/>
			</label>
			{props.count ? (
				<output
					data-filter-summary=""
					aria-live="polite"
					class="pointer-events-none absolute right-3 text-xs text-fg-faint"
				/>
			) : null}
		</div>
	)
}
