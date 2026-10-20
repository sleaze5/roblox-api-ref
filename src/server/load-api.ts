import { readFileSync } from "node:fs"
import { join } from "node:path"
import type { Api } from "../api/types.ts"

let cached: Api | undefined

export function loadApi(): Api {
	cached ??= JSON.parse(
		readFileSync(join(process.cwd(), "src", "generated", "api.json"), "utf8"),
	) as Api
	return cached
}
