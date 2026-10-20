import { token } from "../../api/code-token.ts"
import { enumItemAnchor } from "../../api/routes.ts"
import { plainTags } from "../../api/tags.ts"
import type { Enum, EnumItem } from "../../api/types.ts"
import { CodeTokens } from "../../components/code/CodeTokens.tsx"
import { DocText } from "../../components/doc/DocText.tsx"
import { CopyLink } from "../../components/members/MemberLink.tsx"
import { TagList } from "../../components/TagList.tsx"

export function EnumItemRow(props: { owner: Enum; item: EnumItem }) {
	const anchor = enumItemAnchor(props.item.name)
	return (
		<li id={anchor} data-name={props.item.name} class="member-card">
			<div class="member-head">
				<code class="member-code">
					<CodeTokens
						tokens={[
							token("Enum.", "punctuation"),
							token(props.owner.name, "type"),
							token(".", "punctuation"),
							token(props.item.name, "constant"),
							token(" = ", "punctuation"),
							token(String(props.item.value), "constant"),
						]}
					/>
				</code>
				<div class="member-actions">
					<CopyLink name={props.item.name} href={`#${anchor}`} />
				</div>
			</div>
			<DocText doc={props.item.description} class="member-description" />
			<TagList tags={plainTags(props.item.tags)} />
			{props.item.legacyNames.length === 0 ? null : (
				<p class="text-fg-muted">
					Legacy names:{" "}
					<code class="text-fg">{props.item.legacyNames.join(", ")}</code>
				</p>
			)}
		</li>
	)
}
