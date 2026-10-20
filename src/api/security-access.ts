import type { Security } from "./types.ts"

export type SecurityAccess = {
	security: Security
	identities: number[]
	allowed: Security[]
}

export const securityAccessLevels: readonly SecurityAccess[] = [
	{
		security: "RobloxSecurity",
		identities: [7, 8],
		allowed: [
			"None",
			"PluginSecurity",
			"LocalUserSecurity",
			"WritePlayerSecurity",
			"RobloxScriptSecurity",
			"RobloxSecurity",
			"NotAccessibleSecurity",
		],
	},
	{
		security: "RobloxScriptSecurity",
		identities: [3, 6],
		allowed: [
			"None",
			"PluginSecurity",
			"LocalUserSecurity",
			"RobloxScriptSecurity",
		],
	},
	{
		security: "LocalUserSecurity",
		identities: [1, 4],
		allowed: ["None", "PluginSecurity", "LocalUserSecurity"],
	},
	{
		security: "PluginSecurity",
		identities: [5],
		allowed: ["None", "PluginSecurity"],
	},
	{
		security: "None",
		identities: [2],
		allowed: ["None"],
	},
]

export function identitiesAllowing(security: Security): number[] {
	return securityAccessLevels
		.filter((level) => level.allowed.includes(security))
		.flatMap((level) => level.identities)
		.sort((a, b) => a - b)
}
