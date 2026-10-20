import type { UseKind } from "../../api/facets/facet-use.ts"
import type { FacetKind } from "../../api/types.ts"
import type { IconName } from "../icon-names.ts"
import { kindIcons } from "../members/kind-icons.ts"

export const facetIcons: Record<FacetKind, IconName> = {
	tag: "tag",
	propertyCategory: "folders",
	security: "shield",
	capability: "puzzle-piece",
	threadSafety: "git-fork",
	memoryCategory: "memory",
}

export const useKindIcons: Record<UseKind, IconName> = {
	Class: "cube",
	Enum: "list-bullets",
	EnumItem: "hash",
	...kindIcons,
}
