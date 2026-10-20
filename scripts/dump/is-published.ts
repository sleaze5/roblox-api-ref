import type { RobloxVersion } from "../roblox-version.ts"
import { apiDocsUrl, apiDumpUrl } from "./cdn-urls.ts"

async function exists(url: string): Promise<boolean> {
	const response = await fetch(url, { method: "HEAD" })
	return response.ok
}

export async function isPublished(version: RobloxVersion): Promise<boolean> {
	const urls = [apiDumpUrl(version), apiDocsUrl(version)]
	const found = await Promise.all(urls.map(exists))
	return found.every(Boolean)
}
