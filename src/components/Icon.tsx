import sprite from "../generated/icons.svg?url"
import type { IconName } from "./icon-names.ts"

const sizes = {
	14: "size-3.5",
	16: "size-4",
	20: "size-5",
	28: "size-7",
} as const

export function Icon(props: {
	name: IconName
	size?: keyof typeof sizes
	class?: string
}) {
	return (
		<svg
			class={["shrink-0", sizes[props.size ?? 16], props.class]}
			aria-hidden="true">
			<use href={`${sprite}#${props.name}`} />
		</svg>
	)
}

export function SpriteIcon(props: { name: IconName; class?: string }) {
	return (
		<svg class={props.class} aria-hidden="true">
			<use href={`${sprite}#${props.name}`} />
		</svg>
	)
}
