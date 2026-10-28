import { describe, expect, test } from "vite-plus/test"
import type { Class, Property } from "../../../src/api/types.ts"
import { collectFacets } from "./collect-facets.ts"

function property(name: string, category: string): Property {
	return {
		kind: "Property",
		name,
		category,
		type: { category: "Primitive", name: "bool", optional: false },
		tags: [],
		security: "None",
		writeSecurity: "None",
		threadSafety: "Safe",
		capabilities: { read: [], write: [] },
		canLoad: true,
		canSave: true,
		simulationAccess: false,
	}
}

describe("property category facets", () => {
	test("indexes defining properties without duplicating inherited uses or including other members", () => {
		const parent: Class = {
			name: "Parent",
			memoryCategory: "Instances",
			tags: [],
			subclasses: ["Child"],
			usedBy: [],
			members: [
				property("Enabled", "Behavior"),
				property("Visible", "Appearance"),
				{
					kind: "Event",
					name: "Changed",
					parameters: [],
					tags: [],
					security: "None",
					threadSafety: "Unsafe",
					capabilities: { read: [], write: [] },
				},
			],
		}
		const child: Class = {
			name: "Child",
			superclass: "Parent",
			memoryCategory: "Instances",
			tags: [],
			subclasses: [],
			usedBy: [],
			members: [property("Active", "Behavior"), property("Unnamed", "")],
		}
		const facets = collectFacets([parent, child], []).propertyCategory
		expect(Object.keys(facets)).toEqual(["Appearance", "Behavior"])
		expect(facets["Behavior"]?.uses).toEqual([
			{ kind: "Property", owner: "Child", name: "Active" },
			{ kind: "Property", owner: "Parent", name: "Enabled" },
		])
		expect(facets["Appearance"]?.uses).toEqual([
			{ kind: "Property", owner: "Parent", name: "Visible" },
		])
	})
})
