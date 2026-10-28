export function classIconStyle(icon: number | undefined): string | undefined {
	return icon === undefined ? undefined : `--icon: ${icon}`
}

export function ClassIcon(props: { icon: number; class?: string }) {
	return (
		<span
			aria-hidden="true"
			style={classIconStyle(props.icon)}
			class={["shrink-0 class-icon", props.class]}
		/>
	)
}
