import { describe, expect, test } from "vite-plus/test"
import { releaseName, releaseText } from "./release.ts"

describe("release labels", () => {
	test("name the Studio version and its upload hash", () => {
		const release = { version: "0.744.0.7441077", hash: "7aaa6838d36c4448" }
		expect(releaseName(release)).toBe("Roblox Studio 0.744.0.7441077")
		expect(releaseText(release)).toBe(
			"Roblox Studio 0.744.0.7441077 (version-7aaa6838d36c4448)",
		)
	})
})
