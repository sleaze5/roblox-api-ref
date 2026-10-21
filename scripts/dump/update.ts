import { diffDump } from "../diff/diff-dump.ts"
import { writeDiff } from "../diff/diff-file.ts"
import type { ClientVersion } from "../../src/api/release.ts"
import type { Dump } from "../gen/dump-types.ts"
import { compareVersions } from "../compare-versions.ts"
import { readVersions, writeVersions, type RobloxVersion } from "../roblox-version.ts"
import { apiDumpFile, ensureFile } from "./data-files.ts"
import { fetchLiveVersion } from "./live-version.ts"
import { publishedDate } from "./published-date.ts"
import {
	buildBefore,
	fetchStudioBuilds,
	latestBuildFor,
	type StudioBuild,
} from "./studio-builds.ts"

const channel = "LIVE"

async function release(
	build: StudioBuild,
	player?: ClientVersion,
): Promise<RobloxVersion | undefined> {
	const published = await publishedDate(build)
	if (published === undefined) {
		return undefined
	}
	return player === undefined
		? { channel, ...build, published }
		: { channel, ...build, player, published }
}

async function readDump(version: RobloxVersion): Promise<Dump> {
	return (await Bun.file(await ensureFile(apiDumpFile, version)).json()) as Dump
}

async function saveDiff(from: RobloxVersion, to: RobloxVersion): Promise<void> {
	const [before, after] = await Promise.all([readDump(from), readDump(to)])
	const changes = diffDump(before, after)
	await writeDiff({
		from: { version: from.version, hash: from.hash },
		to: { version: to.version, hash: to.hash },
		changes,
	})
	console.log(
		`Saved ${changes.length} changes from ${from.version} to ${to.version}.`,
	)
}

async function startingVersions(
	builds: StudioBuild[],
	build: StudioBuild,
): Promise<RobloxVersion[]> {
	const previous = buildBefore(builds, build)
	const start = previous === undefined ? undefined : await release(previous)
	return start === undefined ? [] : [start]
}

const player = await fetchLiveVersion("WindowsPlayer", channel)
const builds = await fetchStudioBuilds()
const build = latestBuildFor(builds, player.version)
if (build === undefined) {
	throw new Error(`No Studio build in the version history matches ${player.version}.`)
}
console.log(
	`WindowsPlayer ${channel} is ${player.version} (${player.hash}). The matching Studio build is ${build.version} (${build.hash}).`,
)

async function update(versions: RobloxVersion[], target: StudioBuild): Promise<void> {
	const next = await release(target, player)
	if (next === undefined) {
		console.log(
			`${target.version} is out, but its API files are not on the CDN yet. Keeping ${versions.at(-1)?.version ?? "(none)"}.`,
		)
		return
	}
	const known =
		versions.length > 0 ? versions : await startingVersions(builds, target)
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

async function updatePlayer(
	versions: RobloxVersion[],
	current: RobloxVersion,
): Promise<void> {
	if (current.player?.hash === player.hash) {
		console.log(`Already on ${channel} ${current.version} (${current.hash}).`)
		return
	}
	await writeVersions([...versions.slice(0, -1), { ...current, player }])
	console.log(
		`Already on ${channel} ${current.version} (${current.hash}). Updated its WindowsPlayer version to ${player.version} (${player.hash}).`,
	)
}

const versions = await readVersions()
const current = versions.at(-1)
if (current?.hash === build.hash) {
	await updatePlayer(versions, current)
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
