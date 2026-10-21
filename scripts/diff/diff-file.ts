import { join } from "node:path"
import { diffsDir } from "../paths.ts"
import type { RobloxVersion } from "../roblox-version.ts"
import type { DumpDiff } from "./diff-types.ts"

export function diffPath({ version }: Pick<RobloxVersion, "version">): string {
	return join(diffsDir, `${version}.json`)
}

export async function readDiff(to: Pick<RobloxVersion, "version">): Promise<DumpDiff> {
	return (await Bun.file(diffPath(to)).json()) as DumpDiff
}

export async function writeDiff(diff: DumpDiff): Promise<void> {
	await Bun.write(diffPath(diff.to), `${JSON.stringify(diff, null, "\t")}\n`)
}
