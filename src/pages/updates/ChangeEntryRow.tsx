import type { SignatureParts } from "../../api/signature.ts"
import { addedTags } from "../../api/added-tags.ts"
import type { Api, EnumItem, Member } from "../../api/types.ts"
import type { ChangeEntry, ChangeStatus } from "../../api/update-types.ts"
import { Signature } from "../../components/members/Signature.tsx"
import { TagList } from "../../components/TagList.tsx"
import { ChangeDetails } from "./ChangeDetails.tsx"
import { ChangeMark } from "./ChangeMark.tsx"
import { changeParts, partsText } from "./change-signature.ts"

type Line = {
	status: ChangeStatus
	label: string
	parts: SignatureParts
}

function changeLines(api: Api, entry: ChangeEntry): Line[] {
	const side = (value: Member | EnumItem | undefined, linked: boolean) =>
		value === undefined ? undefined : changeParts(api, entry.owner, value, linked)
	const before = side(entry.before, false)
	const after = side(entry.after, true)
	if (before !== undefined && after !== undefined) {
		return partsText(before) === partsText(after)
			? [{ status: "Changed", label: "Changed:", parts: after }]
			: [
					{ status: "Removed", label: "Before:", parts: before },
					{ status: "Added", label: "After:", parts: after },
				]
	}
	if (after !== undefined) {
		return [{ status: "Added", label: "Added:", parts: after }]
	}
	return before === undefined
		? []
		: [{ status: "Removed", label: "Removed:", parts: before }]
}

export function ChangeEntryRow(props: { api: Api; entry: ChangeEntry }) {
	const tags = addedTags(props.entry)
	return (
		<li
			data-change=""
			data-name={`${props.entry.owner}.${props.entry.name}`}
			class="flex flex-col gap-1">
			{changeLines(props.api, props.entry).map((line) => (
				<div class="change-line">
					<ChangeMark status={line.status} label={line.label} />
					<Signature parts={line.parts} />
				</div>
			))}
			{tags.length === 0 ? null : <TagList tags={tags} class="change-indent" />}
			<ChangeDetails api={props.api} details={props.entry.details} />
		</li>
	)
}
