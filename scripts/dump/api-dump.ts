import type { RobloxVersion } from "../roblox-version.ts"
import { apiDumpUrl } from "./cdn-urls.ts"
import { saveFile } from "./save-file.ts"

export async function downloadApiDump(
	version: RobloxVersion,
	path: string,
): Promise<void> {
	const url = apiDumpUrl(version)
	const response = await fetch(url)
	if (!response.ok) {
		throw new Error(`Failed to download ${url}: HTTP ${response.status}`)
	}
	await saveFile(path, await response.blob())
}
