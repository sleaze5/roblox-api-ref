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

	test("leaves up-to-date versions alone", () => {
		expect(safeUpgrade("1.3.0", ["1.2.9", "1.3.0", "2.0.0"])).toBeUndefined()
	})

	test("moves a prerelease along its own line", () => {
		expect(
			safeUpgrade("2.0.0-rc.15", [
				"2.0.0-rc.9",
				"2.0.0-rc.16",
				"2.0.0-beta.20",
				"2.1.0-rc.1",
				"3.0.0",
			]),
		).toBe("2.0.0-rc.16")
		expect(safeUpgrade("3.0.0-next.50", ["3.0.0-next.51", "3.0.0-next.49"])).toBe(
			"3.0.0-next.51",
		)
		expect(
			safeUpgrade("2.0.0-rc.16", ["2.0.0-rc.16", "2.0.0-rc.2"]),
		).toBeUndefined()
	})

	test("moves a prerelease to the stable release of the same version", () => {
		expect(safeUpgrade("2.0.0-rc.15", ["2.0.0-rc.16", "2.0.0", "2.0.1"])).toBe(
			"2.0.0",
		)
	})

	test("reads npm aliases and writes the new version back into them", () => {
		const spec = parseSpec("vite", "npm:@voidzero-dev/vite-plus-core@1.3.0")
		expect(spec.name).toBe("@voidzero-dev/vite-plus-core")
		expect(spec.version).toBe("1.3.0")
		expect(spec.format("1.4.0")).toBe("npm:@voidzero-dev/vite-plus-core@1.4.0")
		expect(parseSpec("terser", "5.51.2").format("5.52.0")).toBe("5.52.0")
	})
})
