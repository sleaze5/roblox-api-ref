import { shownVersion, uploadName } from "../../api/release.ts"
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

function BuildFacts(props: { update: Update }) {
	const shown = shownVersion(props.update)
	return (
		<>
			<Fact label="Hash" hint="The upload that Roblox serves for this version.">
				<span class="font-mono">{uploadName(shown.hash)}</span>
			</Fact>
			{props.update.player === undefined ? null : (
				<Fact
					label="API build"
					hint="The Roblox build whose API dump this update reads. Roblox Player uploads do not include the API dump.">
					<VersionText
						release={{
							version: props.update.version,
							hash: props.update.hash,
						}}
					/>
				</Fact>
			)}
		</>
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
			<BuildFacts update={props.update} />
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
