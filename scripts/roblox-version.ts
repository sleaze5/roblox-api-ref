import { versionsFile } from "./paths.ts"

export type RobloxVersion = {
	channel: string
	version: string
	hash: string
	published: string
}

export async function readVersions(): Promise<RobloxVersion[]> {
	const file = Bun.file(versionsFile)
	if (!(await file.exists())) {
		return []
	}
	return (await file.json()) as RobloxVersion[]
}

export async function writeVersions(versions: RobloxVersion[]): Promise<void> {
	await Bun.write(versionsFile, `${JSON.stringify(versions, null, "\t")}\n`)
}

export async function readCurrentVersion(): Promise<RobloxVersion> {
	const current = (await readVersions()).at(-1)
	if (current === undefined) {
		throw new Error(
			"roblox-versions.json has no versions. Run `bun vp run dump:update` first.",
		)
	}
	return current
}

export function dumpFileName({ hash, channel }: RobloxVersion): string {
	return `Full-API-Dump-${hash}-${channel}.json`
}

export function apiDocsFileName({ hash, channel }: RobloxVersion): string {
	return `Api-Docs-en-us-${hash}-${channel}.json`
}
