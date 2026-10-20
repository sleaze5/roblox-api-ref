import { join } from "node:path"

export const rootDir = join(import.meta.dir, "..")
export const dataDir = join(rootDir, "data")
export const versionFile = join(rootDir, "roblox-version.json")
export const generatedDir = join(rootDir, "src", "generated")
