import { join } from "node:path"

export const rootDir = join(import.meta.dir, "..")
export const dataDir = join(rootDir, "data")
export const versionsFile = join(rootDir, "roblox-versions.json")
export const diffsDir = join(rootDir, "diffs")
export const generatedDir = join(rootDir, "src", "generated")
