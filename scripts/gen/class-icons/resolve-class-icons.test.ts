import { describe, expect, test } from "vite-plus/test"
import type { Class } from "../../../src/api/types.ts"
import { resolveClassIcons } from "./resolve-class-icons.ts"

function owner(name: string, superclass?: string): Class {
	return {
		name,
		members: [],
		memoryCategory: "Instances",
		tags: [],
		subclasses: [],
		...(superclass === undefined ? {} : { superclass }),
	}
}

const classes = Object.fromEntries(
	[
		owner("Instance"),
		owner("PVInstance", "Instance"),
		owner("BasePart", "PVInstance"),
		owner("Part", "BasePart"),
		owner("FormFactorPart", "BasePart"),
		owner("Seat", "Part"),
		owner("Debris", "Instance"),
	].map((item) => [item.name, item]),
)

describe("class icons", () => {
	test("uses the class's own icon, then its nearest superclass's, then the Class icon", () => {
		const icons = resolveClassIcons(classes, new Set(["Part", "Seat", "Class"]))
		expect(Object.fromEntries(icons)).toEqual({
			Instance: "Class",
			PVInstance: "Class",
			BasePart: "Class",
			Part: "Part",
			FormFactorPart: "Class",
			Seat: "Seat",
			Debris: "Class",
		})
	})

	test("leaves a class without an icon when Studio has no Class icon", () => {
		const icons = resolveClassIcons(classes, new Set(["Part"]))
		expect(icons.has("Debris")).toBe(false)
		expect(icons.get("Seat")).toBe("Part")
	})
})
