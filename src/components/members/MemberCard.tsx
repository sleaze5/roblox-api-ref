import { unlinked } from "../../api/code-token.ts"
import { memberAnchor, memberHref } from "../../api/routes.ts"
import { signatureParts } from "../../api/signature.ts"
import { memberTags } from "../../api/tags.ts"
import type { Member } from "../../api/types.ts"
import { DocText } from "../doc/DocText.tsx"
import {
	HistoryAction,
	HistoryLine,
	HoverHistory,
	historyVariant,
	RecentChip,
} from "../history/MemberHistory.tsx"
import { TagList } from "../TagList.tsx"
import { MemberDetails } from "./details/MemberDetails.tsx"
import { memberAttributes } from "./member-attributes.ts"
import { CopyLink, DocsLink } from "./MemberLink.tsx"
import { Preferred } from "./Preferred.tsx"
import { Signature } from "./Signature.tsx"

export function MemberCard(props: { member: Member; owner: string; own: boolean }) {
	const anchor = memberAnchor(props.member.kind, props.member.name)
	const href = props.own
		? `#${anchor}`
		: memberHref(props.owner, props.member.kind, props.member.name)
	const linked = signatureParts(
		props.member,
		props.owner,
		props.own ? undefined : href,
	)
	const parts = props.own ? { ...linked, owner: unlinked(linked.owner) } : linked
	return (
		<li
			{...memberAttributes(props.member)}
			id={props.own ? anchor : undefined}
			class="member-card">
			<div class="member-head">
				<Signature parts={parts} />
				{historyVariant === "hover" ? (
					<HoverHistory
						history={props.member.history}
						kind="Class"
						owner={props.owner}
					/>
				) : null}
				<div class="member-actions">
					{historyVariant === "icon" ? (
						<HistoryAction
							history={props.member.history}
							name={props.member.name}
							kind="Class"
							owner={props.owner}
						/>
					) : null}
					<CopyLink name={props.member.name} href={href} />
					<DocsLink name={props.member.name} owner={props.owner} />
				</div>
			</div>
			<DocText doc={props.member.description} class="member-description" />
			{historyVariant === "chip" ? (
				<div class="flex flex-wrap items-center gap-1">
					<RecentChip history={props.member.history} />
					<TagList tags={memberTags(props.member)} />
				</div>
			) : (
				<TagList tags={memberTags(props.member)} />
			)}
			{historyVariant === "line" ? (
				<HistoryLine
					history={props.member.history}
					kind="Class"
					owner={props.owner}
				/>
			) : null}
			<MemberDetails member={props.member} />
			<Preferred member={props.member} />
		</li>
	)
}
