import type { RobloxVersion } from "../roblox-version.ts"

const cdn = "https://setup.rbxcdn.com"

export function apiDumpUrl({ hash }: Pick<RobloxVersion, "hash">): string {
	return `${cdn}/version-${hash}-Full-API-Dump.json`
}

export function apiDocsUrl({ hash }: Pick<RobloxVersion, "hash">): string {
	return `${cdn}/version-${hash}-content-api-docs.zip`
}

export function studioIconsUrl({ hash }: Pick<RobloxVersion, "hash">): string {
	return `${cdn}/version-${hash}-content-studio_svg_textures.zip`
}
