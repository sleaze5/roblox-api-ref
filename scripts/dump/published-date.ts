import type { RobloxVersion } from "../roblox-version.ts"
import { apiDocsUrl, apiDumpUrl } from "./cdn-urls.ts"

async function lastModified(url: string): Promise<number | undefined> {
	const response = await fetch(url, { method: "HEAD" })
	const header = response.headers.get("last-modified")
	return response.ok && header !== null ? Date.parse(header) : undefined
}

export async function publishedDate(
	version: Pick<RobloxVersion, "hash">,
): Promise<string | undefined> {
	const urls = [apiDumpUrl(version), apiDocsUrl(version)]
	const times = await Promise.all(urls.map(lastModified))
	if (times.some((time) => time === undefined || Number.isNaN(time))) {
		return undefined
	}
	return new Date(Math.max(...(times as number[]))).toISOString()
}
