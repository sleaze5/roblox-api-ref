import { join } from "node:path"
import { dataDir } from "../paths.ts"
import {
	apiDocsFileName,
	dumpFileName,
	studioIconsFileName,
	type RobloxVersion,
} from "../roblox-version.ts"
import { downloadApiDocs } from "./api-docs.ts"
import { downloadApiDump } from "./api-dump.ts"
import { downloadStudioIcons } from "./studio-icons.ts"

export type DataFile = {
	prefix: string
	fileName: (version: RobloxVersion) => string
	download: (version: RobloxVersion, path: string) => Promise<void>
}

export const apiDumpFile: DataFile = {
	prefix: "Full-API-Dump-",
	fileName: dumpFileName,
	download: downloadApiDump,
}

export const apiDocsFile: DataFile = {
	prefix: "Api-Docs-",
	fileName: apiDocsFileName,
	download: downloadApiDocs,
}

export const studioIconsFile: DataFile = {
	prefix: "Studio-Icons-",
	fileName: studioIconsFileName,
	download: downloadStudioIcons,
}

export async function ensureFile(
	file: DataFile,
	version: RobloxVersion,
): Promise<string> {
	const fileName = file.fileName(version)
	const path = join(dataDir, fileName)
	if (await Bun.file(path).exists()) {
		console.log(`${fileName} is already downloaded.`)
		return path
	}
	await file.download(version, path)
	console.log(`Downloaded ${fileName}.`)
	return path
}
