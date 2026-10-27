import { describe, expect, test } from "vite-plus/test"
import { hashId } from "./hash-id.ts"

describe("hashId", () => {
	test("decodes the id in a URL fragment", () => {
		expect(hashId("#Touched")).toBe("Touched")
		expect(hashId("#Part%20Name")).toBe("Part Name")
		expect(hashId("")).toBe("")
	})

	test("matches nothing when the fragment cannot be decoded", () => {
		expect(hashId("#%")).toBe("")
		expect(hashId("#%E0%A4%A")).toBe("")
	})
})
