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

type Prerelease = {
	tag: string
	number: number
}

function parsePrerelease(text: string | undefined): Prerelease | undefined {
	const match = /^([A-Za-z-]+)\.(\d+)$/.exec(text ?? "")
	if (match === null) {
		return undefined
	}
	const [, tag = "", number] = match
	return { tag, number: Number(number) }
}

function prereleaseUpgrade(from: Version, candidates: string[]): string | undefined {
	const line = parsePrerelease(from.prerelease)
	if (line === undefined) {
		return undefined
	}
	let best: { text: string; number: number } | undefined
	for (const text of candidates) {
		const version = parseVersion(text)
		if (version === undefined || compareStable(version, from) !== 0) {
			continue
		}
		if (version.prerelease === undefined) {
			return text
		}
		const candidate = parsePrerelease(version.prerelease)
		if (
			candidate?.tag === line.tag &&
			candidate.number > (best?.number ?? line.number)
		) {
			best = { text, number: candidate.number }
		}
	}
	return best?.text
}

function stableUpgrade(from: Version, candidates: string[]): string | undefined {
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

export function safeUpgrade(current: string, candidates: string[]): string | undefined {
	const from = parseVersion(current)
	if (from === undefined) {
		return undefined
	}
	return from.prerelease === undefined
		? stableUpgrade(from, candidates)
		: prereleaseUpgrade(from, candidates)
}
