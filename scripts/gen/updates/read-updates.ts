import type { Update } from "../../../src/api/update-types.ts"
import type { ApiNames } from "../api-docs/doc/api-names.ts"
import { readDiff } from "../../diff/diff-file.ts"
import type { RobloxVersion } from "../../roblox-version.ts"
import type { DatedDiff } from "../history/history-table.ts"
import { buildUpdate } from "./build-update.ts"

async function readVersionDiff(
	from: RobloxVersion,
	to: RobloxVersion,
): Promise<DatedDiff> {
	const diff = await readDiff(to)
	if (diff.from.hash !== from.hash || diff.to.hash !== to.hash) {
		throw new Error(
			`diffs/${to.version}.json compares ${diff.from.version} to ${diff.to.version}, but roblox-versions.json lists ${from.version} before ${to.version}.`,
		)
	}
	return { diff, published: to.published }
}

export async function readDiffs(versions: RobloxVersion[]): Promise<DatedDiff[]> {
	return Promise.all(
		versions
			.slice(1)
			.map((to, index) => readVersionDiff(versions[index] as RobloxVersion, to)),
	)
}

export function buildUpdates(diffs: DatedDiff[], names: ApiNames): Update[] {
	return diffs
		.map(({ diff, published }) => buildUpdate(diff, published, names))
		.reverse()
}
