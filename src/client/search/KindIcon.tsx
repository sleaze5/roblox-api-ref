import { classIconStyle } from "../../components/ClassIcon.tsx"
import type { EntryKind } from "../../search/index-format.ts"

export function KindIcon(props: {
	kind: EntryKind
	classIcon?: number | undefined
	class?: string
}) {
	return (
		<span
			aria-hidden="true"
			data-kind={props.kind}
			style={classIconStyle(props.classIcon)}
			class={["kind-icon", props.class]}
		/>
	)
}
