import { readFileSync } from "node:fs"
import { join } from "node:path"
import type { Update } from "../api/update-types.ts"

let cached: Update[] | undefined

export function loadUpdates(): Update[] {
	cached ??= JSON.parse(
		readFileSync(join(process.cwd(), "src", "generated", "updates.json"), "utf8"),
	) as Update[]
	return cached
}
