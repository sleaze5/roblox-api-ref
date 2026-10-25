import { describe, expect, test } from "vite-plus/test"
import { skippedPackages } from "./dependabot.ts"

describe("Dependabot coverage", () => {
	test("skips only the Bun packages that Dependabot ignores entirely", () => {
		const skipped = skippedPackages(`
updates:
    - package-ecosystem: github-actions
      ignore:
          - dependency-name: actions/checkout
    - package-ecosystem: bun
      ignore:
          - dependency-name: "*"
            update-types:
                - version-update:semver-major
          - dependency-name: terser
            versions:
                - 6.x
          - dependency-name: vite
`)
		expect([...skipped]).toEqual(["vite"])
	})

	test("skips nothing without a Bun entry", () => {
		expect(skippedPackages("updates: []").size).toBe(0)
	})

	test("leaves the Vite+ packages to the custom updater", async () => {
		const yaml = await Bun.file(
			new URL("../../.github/dependabot.yml", import.meta.url),
		).text()
		expect([...skippedPackages(yaml)].toSorted()).toEqual(["vite", "vite-plus"])
	})
})
