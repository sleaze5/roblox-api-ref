import {
	groupOwners,
	ownerGroupTitles,
	type OwnerGroupList,
} from "../../api/owner-groups.ts"
import type { OwnerKind, Update } from "../../api/update-types.ts"
import type { IconName } from "../../components/icon-names.ts"

export type OwnerSubsection = OwnerGroupList & {
	id: string
	title: string
	railLabel: string
}

export type OwnerSection = {
	id: string
	title: string
	icon: IconName
	count: number
	groups: OwnerSubsection[]
}

const sectionInfo = {
	Class: { id: "classes", title: "Classes", icon: "cube", noun: "Class" },
	Enum: { id: "enums", title: "Enums", icon: "list-bullets", noun: "Enum" },
} as const

function ownerSection(update: Update, kind: OwnerKind): OwnerSection {
	const info = sectionInfo[kind]
	const groups = groupOwners(update.owners, kind).map((list) => ({
		group: list.group,
		owners: list.owners,
		id: `${info.id}-${list.group}`,
		title: ownerGroupTitles[list.group],
		railLabel: `${info.noun} ${ownerGroupTitles[list.group].toLowerCase()}`,
	}))
	return {
		id: info.id,
		title: info.title,
		icon: info.icon,
		count: groups.reduce((sum, group) => sum + group.owners.length, 0),
		groups,
	}
}

export function ownerSections(update: Update): OwnerSection[] {
	return (["Class", "Enum"] as const)
		.map((kind) => ownerSection(update, kind))
		.filter((section) => section.count > 0)
}
