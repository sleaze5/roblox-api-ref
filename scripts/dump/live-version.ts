import type { ClientVersion } from "../../src/api/release.ts"

type ClientVersionResponse = {
	version: string
	clientVersionUpload: string
}

export async function fetchLiveVersion(
	binaryType: string,
	channel: string,
): Promise<ClientVersion> {
	const response = await fetch(
		`https://clientsettings.roblox.com/v2/client-version/${binaryType}/channel/${channel}`,
	)
	if (!response.ok) {
		throw new Error(
			`Failed to fetch the latest ${channel} ${binaryType} version: HTTP ${response.status}`,
		)
	}
	const { version, clientVersionUpload } =
		(await response.json()) as ClientVersionResponse
	const hash = /^version-([0-9a-f]{16})$/.exec(clientVersionUpload)?.[1]
	if (hash === undefined) {
		throw new Error(`Unexpected version upload id: ${clientVersionUpload}`)
	}
	return { version, hash }
}
