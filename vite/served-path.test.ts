import { join } from "node:path"
import { describe, expect, test } from "vite-plus/test"
import { servedPath } from "./served-path.ts"

const outDir = join("/repo", "dist")

describe("servedPath", () => {
	test("serves files and page indexes inside the output directory", () => {
		expect(servedPath(outDir, "/assets/index.js")).toBe(
			join(outDir, "assets", "index.js"),
		)
		expect(servedPath(outDir, "/classes/Part/")).toBe(
			join(outDir, "classes", "Part", "index.html"),
		)
		expect(servedPath(outDir, "/classes/Part%20Name/")).toBe(
			join(outDir, "classes", "Part Name", "index.html"),
		)
	})

	test.each(["/..%2fdist-secrets/token.txt", "/..%2f..%2fetc/passwd", "/..%2f"])(
		"refuses %s, which leaves the output directory",
		(pathname) => {
			expect(servedPath(outDir, pathname)).toBeUndefined()
		},
	)

	test("refuses a path that cannot be decoded", () => {
		expect(servedPath(outDir, "/%")).toBeUndefined()
	})
})
