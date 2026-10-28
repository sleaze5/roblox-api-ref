import type { StatRef } from "./stats-types.ts"

export function memberSeparator(kind: string): string {
	return kind === "Method" ? ":" : "."
}

export function statRefLabel(ref: StatRef): string {
	switch (ref.kind) {
		case "Class":
		case "DataType":
			return ref.name
		case "Enum":
			return `Enum.${ref.name}`
		case "EnumItem":
			return `Enum.${ref.owner}.${ref.name}`
		case "Parameter":
			return `${ref.owner}${memberSeparator(ref.memberKind)}${ref.member}(${ref.name})`
		case "Update":
			return ref.version
		case "Removed":
			return ref.label
		default:
			return `${ref.owner}${memberSeparator(ref.kind)}${ref.name}`
	}
}
