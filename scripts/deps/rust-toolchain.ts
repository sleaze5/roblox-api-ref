const channelUrl = "https://static.rust-lang.org/dist/channel-rust-stable.toml"

/** The version of the stable Rust release in a channel manifest. */
export function stableVersion(manifest: string): string | undefined {
	return /^\[pkg\.rust\]\nversion = "(\d+\.\d+\.\d+) /m.exec(manifest)?.[1]
}

export async function fetchStableRust(): Promise<string> {
	const response = await fetch(channelUrl)
	if (!response.ok) {
		throw new Error(`Failed to fetch ${channelUrl}: HTTP ${response.status}`)
	}
	const version = stableVersion(await response.text())
	if (version === undefined) {
		throw new Error(`${channelUrl} does not name the stable Rust version.`)
	}
	return version
}

/** The toolchain pinned in rust-toolchain.toml. */
export function toolchainChannel(toml: string): string | undefined {
	return /^channel = "([^"]+)"$/m.exec(toml)?.[1]
}

export function setToolchainChannel(toml: string, version: string): string {
	return toml.replace(/^channel = "[^"]+"$/m, `channel = "${version}"`)
}

/** Keeps the workspace's minimum Rust version at the toolchain's minor version. */
export function setRustVersion(cargoToml: string, version: string): string {
	const minor = version.split(".").slice(0, 2).join(".")
	return cargoToml.replace(/^rust-version = "[^"]+"$/m, `rust-version = "${minor}"`)
}
