import type { Class } from "../../../src/api/types.ts"

export const fallbackIcon = "Class"

function ownOrInherited(
	classes: Record<string, Class>,
	item: Class,
	icons: Set<string>,
): string | undefined {
	for (let current: Class | undefined = item; current;) {
		if (icons.has(current.name)) {
			return current.name
		}
		current =
			current.superclass === undefined ? undefined : classes[current.superclass]
	}
	return icons.has(fallbackIcon) ? fallbackIcon : undefined
}

export function resolveClassIcons(
	classes: Record<string, Class>,
	icons: Set<string>,
): Map<string, string> {
	const resolved = new Map<string, string>()
	for (const item of Object.values(classes)) {
		const icon = ownOrInherited(classes, item, icons)
		if (icon !== undefined) {
			resolved.set(item.name, icon)
		}
	}
	return resolved
}
