export type Release = {
	version: string
	hash: string
}

export function uploadName(hash: string): string {
	return `version-${hash}`
}

export function releaseName(release: Release): string {
	return `Roblox Studio ${release.version}`
}

export function releaseText(release: Release): string {
	return `${releaseName(release)} (${uploadName(release.hash)})`
}
