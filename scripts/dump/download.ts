import { readdir, rm } from "node:fs/promises"
import { join } from "node:path"
import { dataDir } from "../paths.ts"
import {
	apiDocsFileName,
	dumpFileName,
	readVersion,
	type RobloxVersion,
} from "../roblox-version.ts"
import { downloadApiDocs } from "./api-docs.ts"
import { downloadApiDump } from "./api-dump.ts"

type DataFile = {
	prefix: string
	fileName: (version: RobloxVersion) => string
	download: (version: RobloxVersion, path: string) => Promise<void>
}

const dataFiles: DataFile[] = [
	{ prefix: "Full-API-Dump-", fileName: dumpFileName, download: downloadApiDump },
	{
		prefix: "Api-Docs-",
		fileName: apiDocsFileName,
		download: downloadApiDocs,
	},
]

const version = await readVersion()
if (version === undefined) {
	throw new Error("roblox-version.json is missing. Run `vp run dump:update` first.")
}

async function ensureFile(file: DataFile, current: RobloxVersion): Promise<void> {
	const fileName = file.fileName(current)
	const path = join(dataDir, fileName)
	if (await Bun.file(path).exists()) {
		console.log(`${fileName} is already downloaded.`)
		return
	}
	await file.download(current, path)
	console.log(`Downloaded ${fileName}.`)
}

await Promise.all(dataFiles.map((file) => ensureFile(file, version)))

const current = new Set(dataFiles.map((file) => file.fileName(version)))
const staleFiles = (await readdir(dataDir)).filter(
	(name) =>
		dataFiles.some((file) => name.startsWith(file.prefix)) && !current.has(name),
)
await Promise.all(staleFiles.map((name) => rm(join(dataDir, name))))
for (const name of staleFiles) {
	console.log(`Deleted ${name}.`)
}
