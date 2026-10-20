import { versionFile } from "./paths.ts"

export type RobloxVersion = {
	channel: string
	version: string
	hash: string
}

export async function readVersion(): Promise<RobloxVersion | undefined> {
	const file = Bun.file(versionFile)
	if (!(await file.exists())) {
		return undefined
	}
	return (await file.json()) as RobloxVersion
}

export async function writeVersion(version: RobloxVersion): Promise<void> {
	await Bun.write(versionFile, `${JSON.stringify(version, null, "\t")}\n`)
}

export function dumpFileName({ hash, channel }: RobloxVersion): string {
	return `Full-API-Dump-${hash}-${channel}.json`
}

export function apiDocsFileName({ hash, channel }: RobloxVersion): string {
	return `Api-Docs-en-us-${hash}-${channel}.json`
}
