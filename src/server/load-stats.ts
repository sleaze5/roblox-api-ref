import { readFileSync } from "node:fs"
import { join } from "node:path"
import type { Stats } from "../api/stats-types.ts"

let cached: Stats | undefined

export function loadStats(): Stats {
	cached ??= JSON.parse(
		readFileSync(join(process.cwd(), "src", "generated", "stats.json"), "utf8"),
	) as Stats
	return cached
}
