export const siteName = "Roblox API Reference"

export const siteTagline = "Unofficial Roblox Engine API reference."

export const siteSummary =
	"Search and browse every class, member, enum, and data type, and see what changed in each Roblox Studio build."

export const siteDescription = `${siteTagline} ${siteSummary}`

export function brandTitle(pageName?: string): string {
	return pageName === undefined ? siteName : `${pageName} | ${siteName}`
}
