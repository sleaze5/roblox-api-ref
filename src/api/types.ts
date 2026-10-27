import type { Doc } from "./doc-types.ts"
import type { ItemHistory } from "./item-history.ts"

export const securityLevels = [
	"None",
	"PluginSecurity",
	"LocalUserSecurity",
	"WritePlayerSecurity",
	"RobloxScriptSecurity",
	"RobloxSecurity",
	"NotAccessibleSecurity",
] as const

export type Security = (typeof securityLevels)[number]

export type TypeCategory = "Primitive" | "Class" | "DataType" | "Enum" | "Group"

export type ApiType = {
	category: TypeCategory
	name: string
	optional: boolean
	members?: ApiType[]
}

export type Parameter = {
	name: string
	type: ApiType
	default?: string
	description?: Doc
}

export type MemberKind = "Property" | "Method" | "Event" | "Callback"

export type Capabilities = {
	read: string[]
	write: string[]
}

type MemberBase = {
	name: string
	history?: ItemHistory
	tags: string[]
	security: Security
	threadSafety: string
	capabilities: Capabilities
	preferred?: string
	preferredTarget?: { owner: string; kind: MemberKind }
	description?: Doc
}

export type Property = MemberBase & {
	kind: "Property"
	type: ApiType
	category: string
	default?: string
	writeSecurity: Security
	canLoad: boolean
	canSave: boolean
	simulationAccess: boolean
}

export type Method = MemberBase & {
	kind: "Method"
	parameters: Parameter[]
	returns: ApiType
	returnDescription?: Doc
	simulationAccess: boolean
}

export type Event = MemberBase & {
	kind: "Event"
	parameters: Parameter[]
}

export type Callback = MemberBase & {
	kind: "Callback"
	parameters: Parameter[]
	returns: ApiType
	returnDescription?: Doc
}

export type Member = Property | Method | Event | Callback

export type Class = {
	name: string
	history?: ItemHistory
	superclass?: string
	memoryCategory: string
	tags: string[]
	members: Member[]
	subclasses: string[]
	description?: Doc
}

export type EnumItem = {
	name: string
	history?: ItemHistory
	value: number
	tags: string[]
	legacyNames: string[]
	description?: Doc
}

export type TypeRole = "Value" | "Parameter" | "Return"

export type TypeUsage = {
	className: string
	memberName: string
	kind: MemberKind
	role: TypeRole
}

export type DataTypeCategory = "Primitive" | "DataType" | "Group"

export type DataType = {
	name: string
	category: DataTypeCategory
	usedBy: TypeUsage[]
}

export type Enum = {
	name: string
	history?: ItemHistory
	tags: string[]
	items: EnumItem[]
	usedBy: TypeUsage[]
	description?: Doc
}

export const facetKinds = [
	"tag",
	"propertyCategory",
	"security",
	"capability",
	"threadSafety",
	"memoryCategory",
] as const

export type FacetKind = (typeof facetKinds)[number]

export type IndexGroup = "type" | FacetKind

export type FacetAccess = "Read" | "Write"

export type FacetUse =
	| { kind: "Class" | "Enum"; name: string }
	| { kind: "EnumItem"; owner: string; name: string }
	| { kind: MemberKind; owner: string; name: string; access?: FacetAccess }

export type Facet = {
	kind: FacetKind
	name: string
	uses: FacetUse[]
}

export type Api = {
	version: string
	hash: string
	channel: string
	published: string
	classes: Record<string, Class>
	enums: Record<string, Enum>
	dataTypes: Record<string, DataType>
	facets: Record<FacetKind, Record<string, Facet>>
	historyStart?: string
}
