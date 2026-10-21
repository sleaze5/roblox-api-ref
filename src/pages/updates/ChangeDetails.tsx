import { plainTags } from "../../api/tags.ts"
import type { Api } from "../../api/types.ts"
import type { ChangeDetail } from "../../api/update-types.ts"
import { TagList } from "../../components/TagList.tsx"
import { DetailValueText } from "./DetailValueText.tsx"

function ValueChange(props: {
	api: Api
	detail: Extract<ChangeDetail, { kind: "value" }>
}) {
	const before = props.detail.before
	const after = props.detail.after
	return (
		<dd class="flex min-w-0 flex-wrap items-center gap-x-2">
			{before === undefined ? null : (
				<del class="text-fg-faint">
					<DetailValueText api={props.api} value={before} />
				</del>
			)}
			{before === undefined || after === undefined ? null : (
				<span class="text-fg-faint">
					<span aria-hidden="true">→</span>
					<span class="sr-only">changed to</span>
				</span>
			)}
			{after === undefined ? null : (
				<ins class="text-fg no-underline">
					<DetailValueText api={props.api} value={after} />
				</ins>
			)}
		</dd>
	)
}

export function ChangeDetails(props: { api: Api; details: ChangeDetail[] }) {
	return props.details.length === 0 ? null : (
		<dl class="flex flex-col gap-1 change-indent">
			{props.details.map((detail) => (
				<div class="flex flex-wrap items-center gap-x-3 gap-y-1">
					<dt class="text-fg-faint">{detail.label}</dt>
					{detail.kind === "tags" ? (
						<dd>
							<TagList tags={plainTags(detail.tags)} />
						</dd>
					) : (
						<ValueChange api={props.api} detail={detail} />
					)}
				</div>
			))}
		</dl>
	)
}
