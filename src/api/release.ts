export type ClientVersion = {
	version: string
	hash: string
}

export type Release = ClientVersion & {
	player?: ClientVersion
}

export function shownVersion(release: Release): ClientVersion {
	return release.player ?? { version: release.version, hash: release.hash }
}

export function uploadName(hash: string): string {
	return `version-${hash}`
}

export function productName(release: Release): string {
	return release.player === undefined ? "Roblox" : "Roblox Player"
}

export function releaseName(release: Release): string {
	return `${productName(release)} ${shownVersion(release).version}`
}

export function releaseText(release: Release): string {
	return `${releaseName(release)} (${uploadName(shownVersion(release).hash)})`
}
