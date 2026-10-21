import { securityLabel } from "../../../src/api/tags.ts"
import type { Capabilities } from "../../../src/api/types.ts"
import type {
	ChangeDetail,
	DetailTone,
	DetailValue,
} from "../../../src/api/update-types.ts"
import type { FieldChange } from "../../diff/diff-types.ts"
import type { DumpTag } from "../dump-types.ts"
import { normalizeCapabilities, preferredName } from "../normalize-member.ts"
import { normalizeSecurity } from "../normalize-type.ts"

type Serialization = { CanLoad: boolean; CanSave: boolean }

const signatureFields = new Set([
	"Default",
	"MemberType",
	"Parameters",
	"ReturnType",
	"Value",
	"ValueType",
])

function value(text: string, tone: DetailTone): DetailValue {
	return { text, tone }
}

function rawText(raw: unknown): string {
	return typeof raw === "string" ? raw : JSON.stringify(raw)
}

function valueDetail(
	label: string,
	before: DetailValue | undefined,
	after: DetailValue | undefined,
): ChangeDetail[] {
	if (before?.text === after?.text) {
		return []
	}
	const detail: ChangeDetail = { kind: "value", label }
	if (before !== undefined) {
		detail.before = before
	}
	if (after !== undefined) {
		detail.after = after
	}
	return [detail]
}

function securityValue(security: unknown): DetailValue {
	return value(securityLabel(normalizeSecurity(rawText(security ?? "None"))), "text")
}

function securityDetails(change: FieldChange): ChangeDetail[] {
	const before = change.before as { Read?: string; Write?: string } | string | null
	const after = change.after as { Read?: string; Write?: string } | string | null
	if (typeof before === "object" && typeof after === "object") {
		return [
			...valueDetail(
				"Read security",
				securityValue(before?.Read),
				securityValue(after?.Read),
			),
			...valueDetail(
				"Write security",
				securityValue(before?.Write),
				securityValue(after?.Write),
			),
		]
	}
	return valueDetail("Security", securityValue(before), securityValue(after))
}

function serializationText(serialization: Serialization | null): string {
	const canLoad = serialization?.CanLoad ?? false
	const canSave = serialization?.CanSave ?? false
	if (canLoad && canSave) {
		return "Load and save"
	}
	if (canLoad || canSave) {
		return canLoad ? "Load only" : "Save only"
	}
	return "Not serialized"
}

function capabilityValue(names: string[]): DetailValue {
	return names.length === 0 ? value("None", "text") : value(names.join(" + "), "code")
}

function capabilitiesOf(raw: unknown): Capabilities {
	return normalizeCapabilities(raw === null ? undefined : raw)
}

function capabilityDetails(change: FieldChange): ChangeDetail[] {
	const before = capabilitiesOf(change.before)
	const after = capabilitiesOf(change.after)
	const shared = (capabilities: Capabilities) =>
		capabilities.read.join() === capabilities.write.join()
	if (shared(before) && shared(after)) {
		return valueDetail(
			"Capabilities",
			capabilityValue(before.read),
			capabilityValue(after.read),
		)
	}
	return [
		...valueDetail(
			"Read capabilities",
			capabilityValue(before.read),
			capabilityValue(after.read),
		),
		...valueDetail(
			"Write capabilities",
			capabilityValue(before.write),
			capabilityValue(after.write),
		),
	]
}

export function stringTags(tags: unknown): string[] {
	return ((tags ?? []) as DumpTag[]).filter((tag) => typeof tag === "string")
}

function preferredValue(tags: unknown): DetailValue | undefined {
	const name = preferredName((tags ?? undefined) as DumpTag[] | undefined)
	return name === undefined ? undefined : value(name, "code")
}

function tagDetails(before: unknown, after: unknown): ChangeDetail[] {
	const previous = new Set(stringTags(before))
	const next = new Set(stringTags(after))
	const added = [...next].filter((tag) => !previous.has(tag)).sort()
	const removed = [...previous].filter((tag) => !next.has(tag)).sort()
	const details: ChangeDetail[] = []
	if (added.length > 0) {
		details.push({ kind: "tags", label: "Added tags", tags: added })
	}
	if (removed.length > 0) {
		details.push({ kind: "tags", label: "Removed tags", tags: removed })
	}
	return [
		...details,
		...valueDetail("Use instead", preferredValue(before), preferredValue(after)),
	]
}

function optionalValue(raw: unknown, tone: DetailTone): DetailValue | undefined {
	return raw === null || raw === undefined ? undefined : value(rawText(raw), tone)
}

function listValue(raw: unknown): DetailValue {
	const names = (raw ?? []) as string[]
	return names.length === 0 ? value("None", "text") : value(names.join(", "), "code")
}

function genericDetail(change: FieldChange): ChangeDetail[] {
	return valueDetail(
		change.field,
		optionalValue(change.before, "code"),
		optionalValue(change.after, "code"),
	)
}

function fieldDetails(change: FieldChange): ChangeDetail[] {
	switch (change.field) {
		case "Tags":
			return tagDetails(change.before, change.after)
		case "Security":
			return securityDetails(change)
		case "Capabilities":
			return capabilityDetails(change)
		case "Serialization":
			return valueDetail(
				"Serialization",
				value(serializationText(change.before as Serialization | null), "text"),
				value(serializationText(change.after as Serialization | null), "text"),
			)
		case "ThreadSafety":
			return valueDetail(
				"Thread safety",
				value(rawText(change.before ?? "Unknown"), "code"),
				value(rawText(change.after ?? "Unknown"), "code"),
			)
		case "SimulationAccess":
			return valueDetail(
				"Simulation access",
				value(change.before === true ? "Yes" : "No", "text"),
				value(change.after === true ? "Yes" : "No", "text"),
			)
		case "Category":
			return valueDetail(
				"Property category",
				optionalValue(change.before, "code"),
				optionalValue(change.after, "code"),
			)
		case "Superclass":
			return valueDetail(
				"Superclass",
				optionalValue(change.before, "type"),
				optionalValue(change.after, "type"),
			)
		case "MemoryCategory":
			return valueDetail(
				"Memory category",
				optionalValue(change.before, "code"),
				optionalValue(change.after, "code"),
			)
		case "LegacyNames":
			return valueDetail(
				"Legacy names",
				listValue(change.before),
				listValue(change.after),
			)
		default:
			return genericDetail(change)
	}
}

export function changeDetails(changes: FieldChange[]): ChangeDetail[] {
	return changes
		.filter((change) => !signatureFields.has(change.field))
		.flatMap(fieldDetails)
}
