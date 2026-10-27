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
				<li
					data-tooltip={tag.hint}
					class={["tag-chip gap-1.5", toneClasses[tag.tone]]}>
					{tag.label}
					{tag.note === undefined ? null : (
						<span class="text-fg-faint">{tag.note}</span>
					)}
				</li>
			))}
		</ul>
	)
}
