import type { DisplayTag, TagTone } from "../api/tags.ts"

const toneClasses: Record<TagTone, string | undefined> = {
	neutral: undefined,
	warning: "tag-warning",
	danger: "tag-danger",
}

export function TagList(props: { tags: DisplayTag[]; class?: string }) {
	return props.tags.length === 0 ? null : (
		<ul class={["tag-list", props.class]}>
			{props.tags.map((tag) => (
				<li class={["tag-chip", toneClasses[tag.tone]]}>{tag.label}</li>
			))}
		</ul>
	)
}
