import { updateHref } from "../../api/routes.ts"
import { formatDate } from "../../api/update-text.ts"
import {
	changeStatuses,
	type ChangeStatus,
	type Update,
} from "../../api/update-types.ts"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"

const countHints: Record<ChangeStatus, string> = {
	Added: "Classes, members, enums, and enum items that this version adds.",
	Removed: "Classes, members, enums, and enum items that this version removes.",
	Changed:
		"Classes, members, enums, and enum items whose signature, tags, or other details changed.",
}

function PreviousVersion(props: { update: Update; updates: Update[] }) {
	const known = props.updates.some(
		(update) => update.version === props.update.previous.version,
	)
	return known ? (
		<a
			href={updateHref(props.update.previous.version)}
			class="font-mono hover:underline">
			{props.update.previous.version}
		</a>
	) : (
		<span class="font-mono">{props.update.previous.version}</span>
	)
}

export function UpdateFacts(props: {
	update: Update
	updates: Update[]
	published?: boolean
}) {
	return (
		<FactList>
			{props.published ? (
				<Fact
					label="Published"
					hint="When Roblox published the API files of this version.">
					{formatDate(props.update.published)}
				</Fact>
			) : null}
			<Fact
				label="Compared with"
				hint="The previous version that this update compares against.">
				<PreviousVersion update={props.update} updates={props.updates} />
			</Fact>
			{changeStatuses.map((status) => (
				<Fact label={status} hint={countHints[status]}>
					{props.update.counts[status]}
				</Fact>
			))}
		</FactList>
	)
}
