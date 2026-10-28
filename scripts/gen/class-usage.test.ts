import { describe, expect, test } from "vite-plus/test"
import { loadApi } from "../../src/server/load-api.ts"
import type { ApiType } from "../../src/api/types.ts"

const api = loadApi()

function names(type: ApiType): string[] {
	return [type.name, ...(type.members ?? []).flatMap(names)]
}

describe("class usage", () => {
	test("lists the members that use a class as their type", () => {
		const usages = api.classes["BasePart"]?.usedBy ?? []
		expect(usages.length).toBeGreaterThan(0)
		for (const usage of usages) {
			const member = api.classes[usage.className]?.members.find(
				(item) => item.name === usage.memberName,
			)
			const types =
				member === undefined
					? []
					: member.kind === "Property"
						? [member.type]
						: [
								...member.parameters.map((parameter) => parameter.type),
								...(member.kind === "Event" ? [] : [member.returns]),
							]
			expect(types.flatMap(names)).toContain("BasePart")
		}
	})

	test("leaves classes that no member uses with no uses", () => {
		const unused = Object.values(api.classes).filter(
			(item) => item.usedBy.length === 0,
		)
		expect(unused.length).toBeGreaterThan(0)
		expect(api.classes["Instance"]?.usedBy.length).toBeGreaterThan(100)
	})
})
