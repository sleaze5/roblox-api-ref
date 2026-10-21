import { describe, expect, test } from "vite-plus/test"
import { parseSpec } from "./package-spec.ts"
import { safeUpgrade } from "./semver.ts"

describe("safe upgrades", () => {
	test("picks the newest stable version in the same major version", () => {
		expect(
			safeUpgrade("4.9.1", [
				"4.9.0",
				"4.9.2",
				"4.10.0",
				"5.0.0",
				"4.11.0-beta.1",
			]),
		).toBe("4.10.0")
	})

	test("stays in the same minor version below 1.0.0", () => {
		expect(safeUpgrade("0.18.1", ["0.18.3", "0.19.0", "1.0.0"])).toBe("0.18.3")
	})

	test("leaves prerelease versions and up-to-date versions alone", () => {
		expect(safeUpgrade("2.0.0-rc.15", ["2.0.0-rc.16", "2.0.0"])).toBeUndefined()
		expect(safeUpgrade("1.3.0", ["1.2.9", "1.3.0", "2.0.0"])).toBeUndefined()
	})

	test("reads npm aliases and writes the new version back into them", () => {
		const spec = parseSpec("vite", "npm:@voidzero-dev/vite-plus-core@1.3.0")
		expect(spec.name).toBe("@voidzero-dev/vite-plus-core")
		expect(spec.version).toBe("1.3.0")
		expect(spec.format("1.4.0")).toBe("npm:@voidzero-dev/vite-plus-core@1.4.0")
		expect(parseSpec("terser", "5.51.2").format("5.52.0")).toBe("5.52.0")
	})
})
