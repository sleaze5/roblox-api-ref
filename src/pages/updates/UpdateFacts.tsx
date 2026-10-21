import { uploadName } from "../../api/release.ts"
import { updateHref } from "../../api/routes.ts"
import { formatDate } from "../../api/update-text.ts"
import {
	changeStatuses,
	type ChangeStatus,
	type Update,
} from "../../api/update-types.ts"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { VersionText } from "./VersionText.tsx"

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
		<a href={updateHref(props.update.previous.version)} class="hover:underline">
			<VersionText release={props.update.previous} />
		</a>
	) : (
		<VersionText release={props.update.previous} />
	)
}

function HashFact(props: { update: Update }) {
	return (
		<Fact label="Hash" hint="The Studio upload that this version comes from.">
			<span class="font-mono">{uploadName(props.update.hash)}</span>
		</Fact>
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
			<HashFact update={props.update} />
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
			<Fact
				label="Descriptions"
				hint="Descriptions that this version adds, changes, or removes.">
				{props.update.descriptionCount}
			</Fact>
		</FactList>
	)
}
