import type { Capabilities, Member, Parameter } from "../../src/api/types.ts"
import type {
	DumpCapabilities,
	DumpMember,
	DumpParameter,
	DumpTag,
} from "./dump-types.ts"
import { normalizeReturns, normalizeSecurity, normalizeType } from "./normalize-type.ts"

export function normalizeTags(tags: DumpTag[] | undefined): string[] {
	const names = (tags ?? []).filter((tag) => typeof tag === "string")
	const cleaned = names.includes("ReadOnly")
		? names.filter((tag) => tag !== "NotReplicated")
		: names
	return [...new Set(cleaned)].sort()
}

export function preferredName(tags: DumpTag[] | undefined): string | undefined {
	for (const tag of tags ?? []) {
		if (typeof tag !== "string" && tag.PreferredDescriptorName !== undefined) {
			return tag.PreferredDescriptorName
		}
	}
	return undefined
}

function normalizeCapabilities(
	capabilities: DumpCapabilities | undefined,
): Capabilities {
	if (capabilities === undefined) {
		return { read: [], write: [] }
	}
	if (Array.isArray(capabilities)) {
		const sorted = [...capabilities].sort()
		return { read: sorted, write: sorted }
	}
	return {
		read: [...(capabilities.Read ?? [])].sort(),
		write: [...(capabilities.Write ?? [])].sort(),
	}
}

function normalizeParameter(parameter: DumpParameter): Parameter {
	const normalized: Parameter = {
		name: parameter.Name,
		type: normalizeType(parameter.Type),
	}
	if (parameter.Default !== undefined) {
		normalized.default = parameter.Default
	}
	return normalized
}

function normalizeDefault(value: string | undefined): { default?: string } {
	return value === undefined || value.startsWith("__api_dump")
		? {}
		: { default: value }
}

export function normalizeMember(member: DumpMember): Member {
	const preferred = preferredName(member.Tags)
	const base = {
		name: member.Name,
		tags: normalizeTags(member.Tags),
		threadSafety: member.ThreadSafety ?? "Unknown",
		capabilities: normalizeCapabilities(member.Capabilities),
		...(preferred === undefined ? {} : { preferred }),
	}
	switch (member.MemberType) {
		case "Property":
			return {
				...base,
				kind: "Property",
				type: normalizeType(member.ValueType),
				category: member.Category,
				...normalizeDefault(member.Default),
				security: normalizeSecurity(member.Security.Read),
				writeSecurity: normalizeSecurity(member.Security.Write),
				canLoad: member.Serialization.CanLoad,
				canSave: member.Serialization.CanSave,
				simulationAccess: member.SimulationAccess ?? false,
			}
		case "Function":
			return {
				...base,
				kind: "Method",
				security: normalizeSecurity(member.Security),
				parameters: member.Parameters.map(normalizeParameter),
				returns: normalizeReturns(member.ReturnType),
				simulationAccess: member.SimulationAccess ?? false,
			}
		case "Event":
			return {
				...base,
				kind: "Event",
				security: normalizeSecurity(member.Security),
				parameters: member.Parameters.map(normalizeParameter),
			}
		case "Callback":
			return {
				...base,
				kind: "Callback",
				security: normalizeSecurity(member.Security),
				parameters: member.Parameters.map(normalizeParameter),
				returns: normalizeReturns(member.ReturnType),
			}
	}
}
