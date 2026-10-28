import { accessRank } from "./members.ts"
import type { Member, MemberKind } from "./types.ts"

export const accessTiers = ["game", "plugin", "command", "roblox"] as const

export type AccessTier = (typeof accessTiers)[number]

export const accessTierLabels: Record<AccessTier, string> = {
	game: "Game scripts",
	plugin: "Plugins only",
	command: "Command bar only",
	roblox: "Roblox only",
}

export const accessTierHints: Record<AccessTier, string> = {
	game: "Members that game scripts can use, and so can plugins and the command bar.",
	plugin: "Members that plugins and the command bar can use, but game scripts cannot.",
	command:
		"Members that the command bar can use, but plugins and game scripts cannot.",
	roblox: "Members that only Roblox's own scripts and the engine can use.",
}

export function accessTier(member: Member): AccessTier {
	return accessTiers[Math.min(accessRank(member), accessTiers.length - 1)] ?? "roblox"
}

export function tierCounts(members: Member[]): Record<AccessTier, number> {
	const counts = { game: 0, plugin: 0, command: 0, roblox: 0 }
	for (const member of members) {
		counts[accessTier(member)]++
	}
	return counts
}

export type TierRow = {
	tier: AccessTier
	kinds: Record<MemberKind, number>
}
