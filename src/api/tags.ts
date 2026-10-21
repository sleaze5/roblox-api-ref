import { compareNames } from "./compare-names.ts"
import { securityLevels, type Member, type Security } from "./types.ts"

export type TagTone = "neutral" | "warning" | "danger"

export type DisplayTag = {
	label: string
	tone: TagTone
}

const tagLabels: Record<string, string> = {
	Deprecated: "Deprecated",
	Hidden: "Hidden",
	NotBrowsable: "Not browsable",
	NotCreatable: "Not creatable",
	NotReplicated: "Not replicated",
	NotScriptable: "Not scriptable",
	PlayerReplicated: "Player replicated",
	ReadOnly: "Read only",
	WriteOnly: "Write only",
	Service: "Service",
	Yields: "Yields",
	CanYield: "Can yield",
	NoYield: "No yield",
	CustomLuaState: "Custom Lua state",
}

const securityLabels: Record<Security, string> = {
	None: "No security",
	PluginSecurity: "Plugin security",
	LocalUserSecurity: "Local user security",
	WritePlayerSecurity: "Write player security",
	RobloxScriptSecurity: "Roblox script security",
	RobloxSecurity: "Roblox engine security",
	NotAccessibleSecurity: "Not accessible",
}

export function securityLabel(security: Security): string {
	return securityLabels[security]
}

export function securityRank(security: Security): number {
	return securityLevels.indexOf(security)
}

export function isInternalSecurity(security: Security): boolean {
	return securityRank(security) >= securityRank("RobloxScriptSecurity")
}

export function tagLabel(tag: string): string {
	return tagLabels[tag] ?? tag.replaceAll(/(?<=[a-z])(?=[A-Z])/g, " ")
}

function tagTone(tag: string): TagTone {
	return tag === "Deprecated" ? "warning" : "neutral"
}

export function plainTags(tags: string[]): DisplayTag[] {
	const ordered = [...tags].sort((a, b) =>
		a === "Deprecated" ? -1 : b === "Deprecated" ? 1 : compareNames(a, b),
	)
	return ordered.map((tag) => ({ label: tagLabel(tag), tone: tagTone(tag) }))
}

function securityTag(prefix: string, security: Security): DisplayTag {
	return {
		label: `${prefix}${securityLabels[security]}`,
		tone: isInternalSecurity(security) ? "danger" : "neutral",
	}
}

function securityTags(member: Member): DisplayTag[] {
	if (member.kind !== "Property" || member.security === member.writeSecurity) {
		return member.security === "None" ? [] : [securityTag("", member.security)]
	}
	const tags: DisplayTag[] = []
	if (member.security !== "None") {
		tags.push(securityTag("Read: ", member.security))
	}
	if (member.writeSecurity !== "None") {
		tags.push(securityTag("Write: ", member.writeSecurity))
	}
	return tags
}

function threadSafetyTag(member: Member): DisplayTag[] {
	const safety = member.threadSafety
	if (safety === "Unknown") {
		return []
	}
	if (member.kind === "Property") {
		return safety === "Unsafe" ? [{ label: "Thread unsafe", tone: "neutral" }] : []
	}
	if (safety === "Unsafe") {
		return []
	}
	return [
		{
			label: safety === "LocalSafe" ? "Local thread safe" : "Thread safe",
			tone: "neutral",
		},
	]
}

function serializationTag(member: Member): DisplayTag[] {
	if (member.kind !== "Property" || member.canSave === member.canLoad) {
		return []
	}
	return [{ label: member.canSave ? "Save only" : "Load only", tone: "neutral" }]
}

function capabilityTags(member: Member): DisplayTag[] {
	const { read, write } = member.capabilities
	const readText = read.join(" + ")
	const writeText = write.join(" + ")
	if (readText === writeText) {
		return readText === ""
			? []
			: [{ label: `Capabilities: ${readText}`, tone: "neutral" }]
	}
	const tags: DisplayTag[] = []
	if (readText !== "") {
		tags.push({ label: `Read capabilities: ${readText}`, tone: "neutral" })
	}
	if (writeText !== "") {
		tags.push({ label: `Write capabilities: ${writeText}`, tone: "neutral" })
	}
	return tags
}

function simulationTag(member: Member): DisplayTag[] {
	return (member.kind === "Property" || member.kind === "Method") &&
		member.simulationAccess
		? [{ label: "Simulation access", tone: "neutral" }]
		: []
}

export const memberTagGroups = [
	"tag",
	"security",
	"threadSafety",
	"capability",
	"other",
] as const

export type MemberTagGroup = (typeof memberTagGroups)[number]

export const memberTagGroupTitles: Record<MemberTagGroup, string> = {
	tag: "Tags",
	security: "Security",
	threadSafety: "Thread safety",
	capability: "Capabilities",
	other: "Other",
}

export function groupedMemberTags(
	member: Member,
): Record<MemberTagGroup, DisplayTag[]> {
	return {
		tag: plainTags(member.tags),
		security: securityTags(member),
		threadSafety: threadSafetyTag(member),
		capability: capabilityTags(member),
		other: [...serializationTag(member), ...simulationTag(member)],
	}
}

export function memberTags(member: Member): DisplayTag[] {
	const groups = groupedMemberTags(member)
	return memberTagGroups.flatMap((group) => groups[group])
}
