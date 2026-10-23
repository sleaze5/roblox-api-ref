import { classHref, enumHref, enumItemHref, memberHref } from "../routes.ts"
import type { FacetUse } from "../types.ts"

export type UseKind = FacetUse["kind"]

export const useKinds: readonly UseKind[] = [
	"Class",
	"Enum",
	"EnumItem",
	"Property",
	"Method",
	"Event",
	"Callback",
]

export const useKindTitles: Record<UseKind, string> = {
	Class: "Classes",
	Enum: "Enums",
	EnumItem: "Enum items",
	Property: "Properties",
	Method: "Methods",
	Event: "Events",
	Callback: "Callbacks",
}

export type UseSection = {
	kind: UseKind
	uses: FacetUse[]
}

export function useSections(uses: FacetUse[]): UseSection[] {
	return useKinds
		.map((kind) => ({ kind, uses: uses.filter((use) => use.kind === kind) }))
		.filter((section) => section.uses.length > 0)
}

export function useSectionId(kind: UseKind): string {
	return useKindTitles[kind].toLowerCase().replaceAll(" ", "-")
}

export type UseParts = {
	prefix: string
	owner?: string
	separator: string
	name: string
}

export function useParts(use: FacetUse): UseParts {
	switch (use.kind) {
		case "Class":
			return { prefix: "", separator: "", name: use.name }
		case "Enum":
			return { prefix: "Enum.", separator: "", name: use.name }
		case "EnumItem":
			return { prefix: "Enum.", owner: use.owner, separator: ".", name: use.name }
		default:
			return {
				prefix: "",
				owner: use.owner,
				separator: use.kind === "Method" ? ":" : ".",
				name: use.name,
			}
	}
}

export function useText(use: FacetUse): string {
	const parts = useParts(use)
	return `${parts.prefix}${parts.owner ?? ""}${parts.separator}${parts.name}`
}

export function useHref(use: FacetUse): string {
	switch (use.kind) {
		case "Class":
			return classHref(use.name)
		case "Enum":
			return enumHref(use.name)
		case "EnumItem":
			return enumItemHref(use.owner, use.name)
		default:
			return memberHref(use.owner, use.kind, use.name)
	}
}
