import type { OwnerChanges, OwnerKind } from "./update-types.ts"

export const ownerGroupKinds = ["additions", "removals", "changes"] as const

export type OwnerGroupKind = (typeof ownerGroupKinds)[number]

export const ownerGroupTitles: Record<OwnerGroupKind, string> = {
	additions: "Additions",
	removals: "Removals",
	changes: "Changes",
}

export type OwnerGroupList = {
	group: OwnerGroupKind
	owners: OwnerChanges[]
}

function ownerGroupKind(owner: OwnerChanges): OwnerGroupKind {
	if (owner.status === "Added") {
		return "additions"
	}
	return owner.status === "Removed" ? "removals" : "changes"
}

export function groupOwners(owners: OwnerChanges[], kind: OwnerKind): OwnerGroupList[] {
	const ofKind = owners.filter((owner) => owner.kind === kind)
	return ownerGroupKinds
		.map((group) => ({
			group,
			owners: ofKind.filter((owner) => ownerGroupKind(owner) === group),
		}))
		.filter((list) => list.owners.length > 0)
}
