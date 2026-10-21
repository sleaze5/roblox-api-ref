import type { IconName } from "../../components/icon-names.ts"
import type { OwnerChanges, OwnerKind, Update } from "../../api/update-types.ts"

export type OwnerSection = {
	id: string
	title: string
	icon: IconName
	owners: OwnerChanges[]
}

function ownerSection(update: Update, kind: OwnerKind): OwnerSection {
	const owners = update.owners.filter((owner) => owner.kind === kind)
	return kind === "Class"
		? { id: "classes", title: "Classes", icon: "cube", owners }
		: { id: "enums", title: "Enums", icon: "list-bullets", owners }
}

export function ownerSections(update: Update): OwnerSection[] {
	return (["Class", "Enum"] as const)
		.map((kind) => ownerSection(update, kind))
		.filter((section) => section.owners.length > 0)
}
