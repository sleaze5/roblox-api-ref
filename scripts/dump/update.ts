import { compareVersions } from "../compare-versions.ts"
import { diffDescriptions, type DocumentedDump } from "../diff/diff-descriptions.ts"
import { diffDump } from "../diff/diff-dump.ts"
import { writeDiff } from "../diff/diff-file.ts"
import type { ApiDocs } from "../gen/api-docs/api-docs-types.ts"
import type { Dump } from "../gen/dump-types.ts"
import { readVersions, writeVersions, type RobloxVersion } from "../roblox-version.ts"
import { apiDocsFile, apiDumpFile, ensureFile } from "./data-files.ts"
import { fetchLiveVersion } from "./live-version.ts"
import { publishedDate } from "./published-date.ts"
import { buildBefore, fetchStudioBuilds, type StudioBuild } from "./studio-builds.ts"

const channel = "LIVE"

async function release(build: StudioBuild): Promise<RobloxVersion | undefined> {
	const published = await publishedDate(build)
	return published === undefined ? undefined : { channel, ...build, published }
}

async function readDump(version: RobloxVersion): Promise<DocumentedDump> {
	const [dumpPath, docsPath] = await Promise.all([
		ensureFile(apiDumpFile, version),
		ensureFile(apiDocsFile, version),
	])
	return {
		dump: (await Bun.file(dumpPath).json()) as Dump,
		docs: (await Bun.file(docsPath).json()) as ApiDocs,
	}
}

async function saveDiff(from: RobloxVersion, to: RobloxVersion): Promise<void> {
	const [before, after] = await Promise.all([readDump(from), readDump(to)])
	const changes = diffDump(before.dump, after.dump)
	const descriptions = diffDescriptions(before, after)
	await writeDiff({
		from: { version: from.version, hash: from.hash },
		to: { version: to.version, hash: to.hash },
		changes,
		descriptions,
	})
	console.log(
		`Saved ${changes.length} API changes and ${descriptions.length} description changes from ${from.version} to ${to.version}.`,
	)
}

async function startingVersions(build: StudioBuild): Promise<RobloxVersion[]> {
	const previous = buildBefore(await fetchStudioBuilds(), build)
	const start = previous === undefined ? undefined : await release(previous)
	return start === undefined ? [] : [start]
}

async function update(versions: RobloxVersion[], target: StudioBuild): Promise<void> {
	const next = await release(target)
	if (next === undefined) {
		console.log(
			`${target.version} is out, but its API files are not on the CDN yet. Keeping ${versions.at(-1)?.version ?? "(none)"}.`,
		)
		return
	}
	const known = versions.length > 0 ? versions : await startingVersions(target)
	const previous = known.at(-1)
	if (previous !== undefined) {
		await saveDiff(previous, next)
	}
	await writeVersions([...known, next])
	console.log(
		`Updated ${channel} ${previous?.version ?? "(none)"} -> ${next.version} (${next.hash}).`,
	)
	console.log("Run `vp run dump` to download the API docs.")
}

const build = await fetchLiveVersion("WindowsStudio64", channel)
const versions = await readVersions()
const current = versions.at(-1)
if (current?.hash === build.hash) {
	console.log(`Already on ${channel} ${current.version} (${current.hash}).`)
} else if (
	current !== undefined &&
	compareVersions(build.version, current.version) <= 0
) {
	console.log(
		`${build.version} is not newer than ${current.version}. Keeping ${current.version}.`,
	)
} else {
	await update(versions, build)
}
