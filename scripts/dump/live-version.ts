type ClientVersionResponse = {
	version: string
}

export async function fetchLiveVersion(
	binaryType: string,
	channel: string,
): Promise<string> {
	const response = await fetch(
		`https://clientsettings.roblox.com/v2/client-version/${binaryType}/channel/${channel}`,
	)
	if (!response.ok) {
		throw new Error(
			`Failed to fetch the latest ${channel} ${binaryType} version: HTTP ${response.status}`,
		)
	}
	return ((await response.json()) as ClientVersionResponse).version
}
