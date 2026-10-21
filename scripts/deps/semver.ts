export type Version = {
	major: number
	minor: number
	patch: number
	prerelease: string | undefined
}

export function parseVersion(text: string): Version | undefined {
	const match =
		/^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?$/.exec(text)
	if (match === null) {
		return undefined
	}
	const [, major, minor, patch, prerelease] = match
	return {
		major: Number(major),
		minor: Number(minor),
		patch: Number(patch),
		prerelease,
	}
}

function compareStable(a: Version, b: Version): number {
	return a.major - b.major || a.minor - b.minor || a.patch - b.patch
}

function isCompatible(current: Version, candidate: Version): boolean {
	if (candidate.major !== current.major) {
		return false
	}
	return current.major > 0 || candidate.minor === current.minor
}

export function safeUpgrade(current: string, candidates: string[]): string | undefined {
	const from = parseVersion(current)
	if (from === undefined || from.prerelease !== undefined) {
		return undefined
	}
	let best: { text: string; version: Version } | undefined
	for (const text of candidates) {
		const version = parseVersion(text)
		if (
			version === undefined ||
			version.prerelease !== undefined ||
			!isCompatible(from, version) ||
			compareStable(version, from) <= 0
		) {
			continue
		}
		if (best === undefined || compareStable(version, best.version) > 0) {
			best = { text, version }
		}
	}
	return best?.text
}
