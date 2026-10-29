import { describe, expect, test } from "vite-plus/test"
import {
	setRustVersion,
	setToolchainChannel,
	stableVersion,
	toolchainChannel,
} from "./rust-toolchain.ts"

describe("Rust toolchain", () => {
	test("reads the stable version from the channel manifest", () => {
		const manifest =
			'manifest-version = "2"\n\n[pkg.cargo]\nversion = "0.98.0 (abc 2026-11-01)"\n\n[pkg.rust]\nversion = "1.98.1 (def 2026-11-20)"\n'
		expect(stableVersion(manifest)).toBe("1.98.1")
		expect(stableVersion("[pkg.rust]\n")).toBeUndefined()
	})

	test("rewrites the pinned channel and the minimum Rust version", () => {
		const toml = '[toolchain]\nchannel = "1.97.0"\nprofile = "minimal"\n'
		expect(toolchainChannel(toml)).toBe("1.97.0")
		expect(setToolchainChannel(toml, "1.98.1")).toContain('channel = "1.98.1"')
		expect(
			setRustVersion('edition = "2024"\nrust-version = "1.97"\n', "1.98.1"),
		).toBe('edition = "2024"\nrust-version = "1.98"\n')
	})
})
