import { readVersion, writeVersion, type RobloxVersion } from "../roblox-version.ts"
import { publishedDate } from "./published-date.ts"

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

const current = await readVersion()
const previous = current?.version ?? "(none)"

async function update(release: Omit<RobloxVersion, "published">): Promise<void> {
	const label = `${release.channel} ${release.version} (${release.hash})`
	const published = await publishedDate(release)
	if (published === undefined) {
		console.log(
			`${label} is out, but its API files are not on the CDN yet. Keeping ${previous}.`,
		)
		return
	}
	await writeVersion({ ...release, published })
	console.log(
		`Updated ${release.channel} ${previous} -> ${release.version} (${release.hash}).`,
	)
	console.log("Run `vp run dump` to download the API dump.")
}

if (current?.hash === hash && current.channel === channel) {
	console.log(`Already on ${channel} ${version} (${hash}).`)
} else {
	await update({ channel, version, hash })
}
