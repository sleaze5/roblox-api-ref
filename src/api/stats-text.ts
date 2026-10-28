import { formatMonth } from "./update-text.ts"
import { releaseName } from "./release.ts"
import {
	trendKinds,
	type CountKind,
	type StatRef,
	type Tallies,
} from "./stats-types.ts"

export const countLabels: Record<CountKind, string> = {
	Class: "Classes",
	Property: "Properties",
	Method: "Methods",
	Event: "Events",
	Callback: "Callbacks",
	Enum: "Enums",
	EnumItem: "Enum items",
	DataType: "Data types",
}

const refNouns: Record<StatRef["kind"], string> = {
	...countLabels,
	Parameter: "Parameters",
	Update: "Updates",
}

export function refNoun(kind: StatRef["kind"]): string {
	return refNouns[kind].toLowerCase()
}

export function statsLead(release: { version: string; hash: string }): string {
	return `Numbers about the Roblox Engine API in ${releaseName(release)}, and how it changed over the updates that this site tracks.`
}

export function removedNote(historyStart: string): string {
	return `Removed counts what an update removed since Roblox Studio ${historyStart} and no later update added back. The members and items of a removed class or enum are not counted on their own. A data type counts as removed when a removed or changed member used it and no current member does.`
}

export const trendsNote =
	"Each update compares one Roblox Studio version with the one before it. The members and items of an added class or enum count as added. Changed counts the classes, members, enums, and enum items whose signature, tags, or other details changed."

export function monthLabel(period: string): string {
	return formatMonth(`2000-${period}-15T00:00:00Z`)
}

export function changeTotal(tallies: Tallies): number {
	return trendKinds.reduce(
		(sum, kind) =>
			sum + tallies[kind].added + tallies[kind].removed + tallies[kind].changed,
		0,
	)
}

export const monthsNote =
	"Every update, grouped by the month Roblox published it, across all tracked years. Changes counts the classes, members, enums, and enum items that the updates added, removed, or changed."

export function deprecatedTotal(tallies: Tallies): number {
	return (
		tallies.Class.deprecated +
		tallies.Member.deprecated +
		tallies.Enum.deprecated +
		tallies.EnumItem.deprecated
	)
}
