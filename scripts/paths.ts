import { join } from "node:path"

export const rootDir = join(import.meta.dir, "..")
export const generatedDir = join(rootDir, "src", "generated")
