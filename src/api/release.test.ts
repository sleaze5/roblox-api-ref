import { describe, expect, test } from "vite-plus/test"
import { releaseName, releaseText, shownVersion } from "./release.ts"

const build = { version: "0.744.0.7441077", hash: "7aaa6838d36c4448" }
const player = { version: "0.744.0.7441080", hash: "9ac8595712b447a7" }

describe("release labels", () => {
	test("show the Roblox Player version and hash when they are known", () => {
		const release = { ...build, player }
		expect(shownVersion(release)).toEqual(player)
		expect(releaseText(release)).toBe(
			"Roblox Player 0.744.0.7441080 (version-9ac8595712b447a7)",
		)
	})

	test("fall back to the API build without naming a binary", () => {
		expect(shownVersion(build)).toEqual(build)
		expect(releaseName(build)).toBe("Roblox 0.744.0.7441077")
		expect(releaseText(build)).toBe(
			"Roblox 0.744.0.7441077 (version-7aaa6838d36c4448)",
		)
	})
})
