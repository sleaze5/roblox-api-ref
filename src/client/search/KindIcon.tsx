import type { EntryKind } from "../../search/index-format.ts"

export function KindIcon(props: { kind: EntryKind; class?: string }) {
	return (
		<span
			aria-hidden="true"
			data-kind={props.kind}
			class={["kind-icon", props.class]}
		/>
	)
}
