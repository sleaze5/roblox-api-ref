import type { ChangeStatus, DescriptionDetail } from "../../api/update-types.ts"
import { DocText } from "../../components/doc/DocText.tsx"

const statusWords: Record<ChangeStatus, string> = {
	Added: "added",
	Removed: "removed",
	Changed: "changed",
}

function DescriptionLabel(props: { detail: DescriptionDetail }) {
	const status = statusWords[props.detail.status]
	switch (props.detail.part) {
		case "description":
			return <>Description {status}</>
		case "return":
			return <>Return value description {status}</>
		case "parameter":
			return (
				<>
					Description of <code class="text-fg">{props.detail.parameter}</code>{" "}
					{status}
				</>
			)
	}
}

export function DescriptionChanges(props: { details: DescriptionDetail[] }) {
	return props.details.length === 0 ? null : (
		<div class="flex flex-col gap-3 change-indent">
			{props.details.map((detail) => (
				<div class="flex flex-col gap-1.5">
					<p class="text-fg-faint">
						<DescriptionLabel detail={detail} />
					</p>
					<DocText doc={detail.after} class="member-description" />
					{detail.before === undefined ? null : (
						<details class="change-previous">
							<summary>Previous description</summary>
							<DocText doc={detail.before} class="member-description" />
						</details>
					)}
				</div>
			))}
		</div>
	)
}
