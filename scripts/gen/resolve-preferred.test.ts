import { describe, expect, test } from "vite-plus/test"
import type { Class, Event, Member, Method } from "../../src/api/types.ts"
import { resolvePreferredMembers } from "./resolve-preferred.ts"

function event(name: string, preferred?: string): Event {
	return {
		kind: "Event",
		name,
		parameters: [],
		tags: [],
		security: "None",
		threadSafety: "Unsafe",
		capabilities: { read: [], write: [] },
		...(preferred === undefined ? {} : { preferred }),
	}
}

function owner(name: string, members: Member[], superclass?: string): Class {
	return {
		name,
		members,
		memoryCategory: "Instances",
		tags: [],
		subclasses: [],
		...(superclass === undefined ? {} : { superclass }),
	}
}

describe("preferred member targets", () => {
	test("uses the replacement's kind and declaring class, including inherited members", () => {
		const replacement: Method = {
			...event("Bind"),
			kind: "Method",
			returns: { category: "Primitive", name: "null", optional: false },
			simulationAccess: false,
		}
		const oldParent = event("OldParent", "Bind")
		const oldChild = event("OldChild", "Bind")
		resolvePreferredMembers({
			Parent: owner("Parent", [replacement, oldParent]),
			Child: owner("Child", [oldChild], "Parent"),
		})
		expect(oldParent.preferredTarget).toEqual({ owner: "Parent", kind: "Method" })
		expect(oldChild.preferredTarget).toEqual({ owner: "Parent", kind: "Method" })
	})

	test("prefers local overrides and leaves missing or unrelated replacements unlinked", () => {
		const local = event("Old", "New")
		const missing = event("Missing", "Gone")
		const unrelated = event("Unrelated", "Elsewhere")
		resolvePreferredMembers({
			Parent: owner("Parent", [event("New")]),
			Child: owner("Child", [event("New"), local, missing, unrelated], "Parent"),
			Other: owner("Other", [event("Elsewhere")]),
		})
		expect(local.preferredTarget).toEqual({ owner: "Child", kind: "Event" })
		expect(missing.preferred).toBe("Gone")
		expect(missing.preferredTarget).toBeUndefined()
		expect(unrelated.preferredTarget).toBeUndefined()
	})
})
