import { readVersion, writeVersion, type RobloxVersion } from "../roblox-version.ts"
import { isPublished } from "./is-published.ts"

const channel = "LIVE"

type ClientVersionResponse = {
	version: string
	clientVersionUpload: string
}

const response = await fetch(
	`https://clientsettings.roblox.com/v2/client-version/WindowsStudio64/channel/${channel}`,
)
if (!response.ok) {
	throw new Error(
		`Failed to fetch the latest ${channel} version: HTTP ${response.status}`,
	)
}

const { version, clientVersionUpload } =
	(await response.json()) as ClientVersionResponse
const hash = /^version-([0-9a-f]{16})$/.exec(clientVersionUpload)?.[1]
if (hash === undefined) {
	throw new Error(`Unexpected version upload id: ${clientVersionUpload}`)
}

const latest: RobloxVersion = { channel, version, hash }
const current = await readVersion()

if (current?.hash === latest.hash && current.channel === latest.channel) {
	console.log(`Already on ${channel} ${version} (${hash}).`)
} else if (!(await isPublished(latest))) {
	console.log(
		`${channel} ${version} (${hash}) is out, but its API files are not on the CDN yet. Keeping ${current?.version ?? "(none)"}.`,
	)
} else {
	await writeVersion(latest)
	console.log(
		`Updated ${channel} ${current?.version ?? "(none)"} -> ${version} (${hash}).`,
	)
	console.log("Run `vp run dump` to download the API dump.")
}
