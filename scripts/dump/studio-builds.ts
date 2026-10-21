import { compareVersions } from "../compare-versions.ts"

const versionHistoryUrl =
	"https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/roblox/version-history.json"

export type StudioBuild = {
	version: string
	hash: string
}

export function parseStudioBuilds(history: Record<string, string>): StudioBuild[] {
	return Object.entries(history)
		.flatMap(([version, upload]) => {
			const hash = /^version-([0-9a-f]{16})$/.exec(upload)?.[1]
			return hash === undefined ? [] : [{ version, hash }]
		})
		.sort((a, b) => compareVersions(a.version, b.version))
}

export async function fetchStudioBuilds(): Promise<StudioBuild[]> {
	const response = await fetch(versionHistoryUrl)
	if (!response.ok) {
		throw new Error(`Failed to fetch ${versionHistoryUrl}: HTTP ${response.status}`)
	}
	return parseStudioBuilds((await response.json()) as Record<string, string>)
}

export function latestBuildFor(
	builds: StudioBuild[],
	version: string,
): StudioBuild | undefined {
	return builds.findLast((build) => compareVersions(build.version, version) <= 0)
}

export function buildBefore(
	builds: StudioBuild[],
	build: StudioBuild,
): StudioBuild | undefined {
	return builds.findLast((entry) => compareVersions(entry.version, build.version) < 0)
}
