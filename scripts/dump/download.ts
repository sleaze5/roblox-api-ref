import { readdir, rm } from "node:fs/promises"
import { join } from "node:path"
import { dataDir } from "../paths.ts"
import { readCurrentVersion } from "../roblox-version.ts"
import { apiDocsFile, apiDumpFile, ensureFile } from "./data-files.ts"

const dataFiles = [apiDumpFile, apiDocsFile]
const version = await readCurrentVersion()

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
