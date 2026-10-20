import { memberHref } from "../../api/routes.ts"
import type { Member } from "../../api/types.ts"

export function Preferred(props: { member: Member; class?: string }) {
	return props.member.preferred === undefined ? null : (
		<p class={["text-fg-muted", props.class]}>
			Use{" "}
			{props.member.preferredTarget === undefined ? (
				<span class="font-mono text-fg">{props.member.preferred}</span>
			) : (
				<a
					href={memberHref(
						props.member.preferredTarget.owner,
						props.member.preferredTarget.kind,
						props.member.preferred,
					)}
					class="font-mono text-fg underline decoration-line-strong hover:decoration-fg">
					{props.member.preferred}
				</a>
			)}{" "}
			instead.
		</p>
	)
}
