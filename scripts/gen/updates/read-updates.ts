import type { Update } from "../../../src/api/update-types.ts"
import { readDiff } from "../../diff/diff-file.ts"
import type { RobloxVersion } from "../../roblox-version.ts"
import { buildUpdate } from "./build-update.ts"

async function readUpdate(from: RobloxVersion, to: RobloxVersion): Promise<Update> {
	const diff = await readDiff(to)
	if (diff.from.hash !== from.hash || diff.to.hash !== to.hash) {
		throw new Error(
			`diffs/${to.version}.json compares ${diff.from.version} to ${diff.to.version}, but roblox-versions.json lists ${from.version} before ${to.version}.`,
		)
	}
	return buildUpdate(diff, to.published)
}

export async function readUpdates(versions: RobloxVersion[]): Promise<Update[]> {
	const updates = await Promise.all(
		versions
			.slice(1)
			.map((to, index) => readUpdate(versions[index] as RobloxVersion, to)),
	)
	return updates.reverse()
}
