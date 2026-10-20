import type { Class, Member } from "../../src/api/types.ts"

function preferredTarget(
	classes: Record<string, Class>,
	owner: Class,
	name: string,
): Member["preferredTarget"] {
	let current: Class | undefined = owner
	while (current !== undefined) {
		const member = current.members.find((candidate) => candidate.name === name)
		if (member !== undefined) {
			return { owner: current.name, kind: member.kind }
		}
		current =
			current.superclass === undefined ? undefined : classes[current.superclass]
	}
	return undefined
}

export function resolvePreferredMembers(classes: Record<string, Class>): void {
	for (const owner of Object.values(classes)) {
		for (const member of owner.members) {
			if (member.preferred === undefined) {
				continue
			}
			const target = preferredTarget(classes, owner, member.preferred)
			if (target !== undefined) {
				member.preferredTarget = target
			}
		}
	}
}
