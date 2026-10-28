import type { RobloxVersion } from "../roblox-version.ts"
import type { StudioIcons } from "../gen/class-icons/studio-icons-types.ts"
import { studioIconsUrl } from "./cdn-urls.ts"
import { saveFile } from "./save-file.ts"
import { extractEntries } from "./zip/extract-entries.ts"
import { httpRangeReader } from "./zip/range-reader.ts"

const iconPattern = /^Shared\\InsertableObjects\\Dark\\Standard\\([^\\@]+)@3x\.png$/

export async function downloadStudioIcons(
	version: RobloxVersion,
	path: string,
): Promise<void> {
	const reader = await httpRangeReader(studioIconsUrl(version))
	const entries = await extractEntries(reader, (name) => iconPattern.test(name))
	const icons: StudioIcons = {}
	for (const [name, bytes] of entries) {
		const className = iconPattern.exec(name)?.[1]
		if (className !== undefined) {
			icons[className] = Buffer.from(bytes).toString("base64")
		}
	}
	if (Object.keys(icons).length === 0) {
		throw new Error(`${studioIconsUrl(version)} has no Studio class icons.`)
	}
	await saveFile(path, JSON.stringify(icons))
}
