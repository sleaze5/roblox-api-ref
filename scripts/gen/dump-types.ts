export type DumpType = {
	Category?: string
	Name: string
}

export type DumpTag =
	| string
	| { PreferredDescriptorName?: string; ThreadSafety?: string }

export type DumpCapabilities = string[] | { Read?: string[]; Write?: string[] }

export type DumpParameter = {
	Name: string
	Type: DumpType
	Default?: string
}

type DumpMemberBase = {
	Name: string
	Tags?: DumpTag[]
	ThreadSafety?: string
	Capabilities?: DumpCapabilities
	SimulationAccess?: boolean
}

export type DumpProperty = DumpMemberBase & {
	MemberType: "Property"
	Category: string
	Default?: string
	ValueType: DumpType
	Security: { Read: string; Write: string }
	Serialization: { CanLoad: boolean; CanSave: boolean }
}

export type DumpFunction = DumpMemberBase & {
	MemberType: "Function"
	Parameters: DumpParameter[]
	ReturnType: DumpType | DumpType[]
	Security: string
}

export type DumpEvent = DumpMemberBase & {
	MemberType: "Event"
	Parameters: DumpParameter[]
	Security: string
}

export type DumpCallback = DumpMemberBase & {
	MemberType: "Callback"
	Parameters: DumpParameter[]
	ReturnType: DumpType | DumpType[]
	Security: string
}

export type DumpMember = DumpProperty | DumpFunction | DumpEvent | DumpCallback

export type DumpClass = {
	Name: string
	Superclass: string
	MemoryCategory: string
	Tags?: DumpTag[]
	Members: DumpMember[]
}

export type DumpEnumItem = {
	Name: string
	Value: number
	Tags?: DumpTag[]
	LegacyNames?: string[]
}

export type DumpEnum = {
	Name: string
	Tags?: DumpTag[]
	Items: DumpEnumItem[]
}

export type Dump = {
	Version: number
	Classes: DumpClass[]
	Enums: DumpEnum[]
}
