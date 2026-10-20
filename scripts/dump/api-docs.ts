import type { RobloxVersion } from "../roblox-version.ts"
import { apiDocsUrl } from "./cdn-urls.ts"
import { saveFile } from "./save-file.ts"
import { extractEntry } from "./zip/extract-entry.ts"
import { httpRangeReader } from "./zip/range-reader.ts"

export async function downloadApiDocs(
	version: RobloxVersion,
	path: string,
): Promise<void> {
	const reader = await httpRangeReader(apiDocsUrl(version))
	await saveFile(path, await extractEntry(reader, "en-us.json"))
}
