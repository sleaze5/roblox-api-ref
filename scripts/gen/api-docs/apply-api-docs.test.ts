import { describe, expect, test } from "vite-plus/test"
import { applyApiDocs } from "./apply-api-docs.ts"
import { testNames } from "./test-names.ts"

describe("applyApiDocs", () => {
	test("describes classes, members, enums, and enum items from their documentation", () => {
		const names = testNames()
		applyApiDocs(names, {
			"@roblox/globaltype/BasePart": {
				documentation: "Base of <code>Part</code>.",
			},
			"@roblox/globaltype/BasePart.Anchored": { documentation: "Stays still." },
			"@roblox/globaltype/Part": { documentation: "  " },
			"@roblox/global/Enum.Material": { documentation: "Materials." },
			"@roblox/enum/Material.Plastic": { documentation: "Plastic." },
		})
		expect(names.classes["BasePart"]?.description).toEqual([
			"Base of ",
			{ tag: "code", children: ["Part"], link: { kind: "class", name: "Part" } },
			".",
		])
		expect(names.classes["BasePart"]?.members[0]?.description).toEqual([
			"Stays still.",
		])
		expect(names.classes["Part"]).not.toHaveProperty("description")
		expect(names.classes["Instance"]).not.toHaveProperty("description")
		expect(names.enums["Material"]?.description).toEqual(["Materials."])
		expect(names.enums["Material"]?.items[0]?.description).toEqual(["Plastic."])
	})

	test("describes parameters by name and the return value of methods", () => {
		const names = testNames()
		applyApiDocs(names, {
			"@roblox/globaltype/Instance.FindFirstChild": {
				documentation: "Finds a child.",
				params: [
					{
						name: "self",
						documentation:
							"@roblox/globaltype/Instance.FindFirstChild/param/0",
					},
					{
						name: "name",
						documentation:
							"@roblox/globaltype/Instance.FindFirstChild/param/1",
					},
					{
						name: "deep",
						documentation:
							"@roblox/globaltype/Instance.FindFirstChild/param/2",
					},
				],
				returns: ["@roblox/globaltype/Instance.FindFirstChild/return/0"],
			},
			"@roblox/globaltype/Instance.FindFirstChild/param/0": {
				documentation: "An object.",
			},
			"@roblox/globaltype/Instance.FindFirstChild/param/1": {
				documentation: "The <code>Name</code> to find.",
			},
			"@roblox/globaltype/Instance.FindFirstChild/param/2": {
				documentation: "Searches deeply.",
			},
			"@roblox/globaltype/Instance.FindFirstChild/return/0": {
				documentation: "The child.",
			},
		})
		const method = names.classes["Instance"]?.members[0]
		if (method?.kind !== "Method") {
			throw new Error("expected a method")
		}
		expect(method.parameters[0]?.description).toEqual([
			"The ",
			{ tag: "code", children: ["Name"] },
			" to find.",
		])
		expect(method.parameters[1]).not.toHaveProperty("description")
		expect(method.returnDescription).toEqual(["The child."])
	})
})
