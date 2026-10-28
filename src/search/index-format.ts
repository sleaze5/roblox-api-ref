export const entryKinds = [
	"Class",
	"Enum",
	"Property",
	"Method",
	"Event",
	"Callback",
	"EnumItem",
	"DataType",
] as const

export type EntryKind = (typeof entryKinds)[number]

export const entryFlags = {
	deprecated: 1,
	hidden: 2,
	notScriptable: 4,
	service: 8,
	readOnly: 16,
	yields: 32,
	notCreatable: 64,
	notReplicated: 128,
	internal: 256,
} as const

export type EntryFlag = keyof typeof entryFlags

export type SearchUpdate = {
	version: string
	hash: string
	published: string
}

export type RemovedEntry = {
	kind: EntryKind
	name: string
	parent?: string
	version: string
	published: string
}

export type SearchIndex = {
	names: string[]
	kinds: number[]
	parents: number[]
	flags: number[]
	security: number[]
	valueTypes: number[][]
	parameterTypes: number[][]
	typeNames: string[]
	typeEntries: number[]
	updates: SearchUpdate[]
	removed: RemovedEntry[]
	classIcons: (number | null)[]
}
