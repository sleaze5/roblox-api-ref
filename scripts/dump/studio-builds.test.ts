import { describe, expect, test } from "vite-plus/test"
import { buildBefore, latestBuildFor, parseStudioBuilds } from "./studio-builds.ts"

const builds = parseStudioBuilds({
	"0.743.0.7431133": "version-4f087c32742644ab",
	"0.744.0.7441077": "version-7aaa6838d36c4448",
	"0.740.19.7400931": "version-c792f79abddd41bd",
	"0.742.0.7421053": "version-9b554450a0fc4e65",
	"0.745.0.7451000": "not-a-version",
})

describe("Studio builds", () => {
	test("sorts builds by version number and skips unknown upload ids", () => {
		expect(builds.map((build) => build.version)).toEqual([
			"0.740.19.7400931",
			"0.742.0.7421053",
			"0.743.0.7431133",
			"0.744.0.7441077",
		])
	})

	test("picks the newest Studio build that is not newer than the player", () => {
		expect(latestBuildFor(builds, "0.744.0.7441080")).toEqual({
			version: "0.744.0.7441077",
			hash: "7aaa6838d36c4448",
		})
		expect(latestBuildFor(builds, "0.743.5.7431200")?.version).toBe(
			"0.743.0.7431133",
		)
		expect(latestBuildFor(builds, "0.700.0.7000000")).toBeUndefined()
	})

	test("finds the build before another build", () => {
		const [, , previous, latest] = builds
		expect(latest && buildBefore(builds, latest)).toEqual(previous)
	})
})
